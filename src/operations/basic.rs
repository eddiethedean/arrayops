use pyo3::buffer::{Element, PyBuffer};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::types::TypeCode;
use crate::validation::{detect_input_type, validate_for_operation, get_typecode_unified};
use crate::buffer::{
    get_array_len, get_itemsize, CACHE_BLOCK_SIZE,
    PARALLEL_THRESHOLD_SUM, PARALLEL_THRESHOLD_SCALE, PARALLEL_THRESHOLD_MEAN, PARALLEL_THRESHOLD_MINMAX,
};

#[cfg(feature = "parallel")]
use crate::buffer::{extract_buffer_to_vec, should_parallelize};

// Generic sum implementation with cache-friendly processing
fn sum_impl<T>(py: Python<'_>, buffer: &PyBuffer<T>, len: usize) -> PyResult<T>
where
    T: Element + Copy + Default + std::ops::Add<Output = T> + IntoPy<PyObject> + Send + Sync,
{
    #[cfg(feature = "parallel")]
    {
        if should_parallelize(len, PARALLEL_THRESHOLD_SUM) {
            let data = extract_buffer_to_vec(py, buffer)?;
            return Ok(data
                .par_iter()
                .copied()
                .reduce(|| T::default(), |a, b| a + b));
        }
    }

    #[cfg(not(feature = "parallel"))]
    let _ = len; // Suppress unused parameter warning when parallel is disabled

    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    // Cache-friendly processing: process in chunks to improve memory access patterns
    // For large arrays, chunking helps with cache locality
    if len > CACHE_BLOCK_SIZE {
        let mut sum = T::default();
        for chunk in slice.chunks(CACHE_BLOCK_SIZE) {
            let chunk_sum = chunk
                .iter()
                .map(|cell| cell.get())
                .fold(T::default(), |acc, x| acc + x);
            sum = sum + chunk_sum;
        }
        Ok(sum)
    } else {
        // Small arrays: direct processing
        Ok(slice
            .iter()
            .map(|cell| cell.get())
            .fold(T::default(), |acc, x| acc + x))
    }
}

// Generic scale implementation (in-place)
fn scale_impl<T, F>(py: Python, buffer: &mut PyBuffer<T>, factor: F, len: usize) -> PyResult<()>
where
    T: Element + Copy + std::ops::Mul<F, Output = T> + Send + Sync,
    F: Copy + Send + Sync,
{
    let slice = buffer
        .as_mut_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;

    #[cfg(feature = "parallel")]
    {
        if should_parallelize(len, PARALLEL_THRESHOLD_SCALE) {
            // Extract data to Vec for parallel processing
            let mut data: Vec<T> = slice.iter().map(|cell| cell.get()).collect();

            // Process in parallel
            data.par_iter_mut().for_each(|x| *x = *x * factor);

            // Write back to buffer
            for (item, &val) in slice.iter().zip(data.iter()) {
                item.set(val);
            }
            return Ok(());
        }
    }

    #[cfg(not(feature = "parallel"))]
    let _ = len; // Suppress unused parameter warning when parallel is disabled

    for item in slice.iter() {
        item.set(item.get() * factor);
    }
    Ok(())
}

// Generic mean implementation for integer types
fn mean_impl_int<T>(py: Python, buffer: &PyBuffer<T>, len: usize) -> PyResult<f64>
where
    T: Element + Copy + Default + std::ops::Add<Output = T> + Send + Sync,
    f64: From<T>,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    #[cfg(feature = "parallel")]
    {
        if should_parallelize(len, PARALLEL_THRESHOLD_MEAN) {
            let data = extract_buffer_to_vec(py, buffer)?;
            let sum: T = data
                .par_iter()
                .copied()
                .reduce(|| T::default(), |a, b| a + b);
            return Ok(f64::from(sum) / len as f64);
        }
    }

    // Fast path for small arrays
    if len <= 16 {
        let sum: T = slice
            .iter()
            .map(|cell| cell.get())
            .fold(T::default(), |acc, x| acc + x);
        return Ok(f64::from(sum) / len as f64);
    }

    // Cache-friendly processing for larger arrays
    let sum: T = if len > CACHE_BLOCK_SIZE {
        let mut total = T::default();
        for chunk in slice.chunks(CACHE_BLOCK_SIZE) {
            let chunk_sum = chunk
                .iter()
                .map(|cell| cell.get())
                .fold(T::default(), |acc, x| acc + x);
            total = total + chunk_sum;
        }
        total
    } else {
        slice
            .iter()
            .map(|cell| cell.get())
            .fold(T::default(), |acc, x| acc + x)
    };
    Ok(f64::from(sum) / len as f64)
}

// Generic mean implementation for float types
fn mean_impl_float<T>(py: Python, buffer: &PyBuffer<T>, len: usize) -> PyResult<f64>
where
    T: Element + Copy + Default + std::ops::Add<Output = T> + Send + Sync,
    f64: From<T>,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    #[cfg(feature = "parallel")]
    {
        if should_parallelize(len, PARALLEL_THRESHOLD_MEAN) {
            let data = extract_buffer_to_vec(py, buffer)?;
            let sum: T = data
                .par_iter()
                .copied()
                .reduce(|| T::default(), |a, b| a + b);
            return Ok(f64::from(sum) / len as f64);
        }
    }

    // Fast path for small arrays
    if len <= 16 {
        let sum: T = slice
            .iter()
            .map(|cell| cell.get())
            .fold(T::default(), |acc, x| acc + x);
        return Ok(f64::from(sum) / len as f64);
    }

    // Cache-friendly processing for larger arrays
    let sum: T = if len > CACHE_BLOCK_SIZE {
        let mut total = T::default();
        for chunk in slice.chunks(CACHE_BLOCK_SIZE) {
            let chunk_sum = chunk
                .iter()
                .map(|cell| cell.get())
                .fold(T::default(), |acc, x| acc + x);
            total = total + chunk_sum;
        }
        total
    } else {
        slice
            .iter()
            .map(|cell| cell.get())
            .fold(T::default(), |acc, x| acc + x)
    };
    Ok(f64::from(sum) / len as f64)
}

// Generic min implementation
fn min_impl<T>(py: Python<'_>, buffer: &PyBuffer<T>) -> PyResult<T>
where
    T: Element + Copy + PartialOrd + Send + Sync,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    let len = slice.len();

    // Fast path for very small arrays
    match len {
        0 => unreachable!(), // Should be checked before calling
        1 => return Ok(slice[0].get()),
        2 => {
            let a = slice[0].get();
            let b = slice[1].get();
            return Ok(if a < b { a } else { b });
        }
        3 => {
            let a = slice[0].get();
            let b = slice[1].get();
            let c = slice[2].get();
            let ab = if a < b { a } else { b };
            return Ok(if ab < c { ab } else { c });
        }
        4 => {
            let a = slice[0].get();
            let b = slice[1].get();
            let c = slice[2].get();
            let d = slice[3].get();
            let ab = if a < b { a } else { b };
            let cd = if c < d { c } else { d };
            return Ok(if ab < cd { ab } else { cd });
        }
        _ => {} // Continue to general case
    }

    #[cfg(feature = "parallel")]
    {
        if should_parallelize(len, PARALLEL_THRESHOLD_MINMAX) {
            let data = extract_buffer_to_vec(py, buffer)?;
            return Ok(data
                .par_iter()
                .copied()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap());
        }
    }

    let mut min_val = slice[0].get();
    for cell in slice.iter().skip(1) {
        let val = cell.get();
        if val < min_val {
            min_val = val;
        }
    }
    Ok(min_val)
}

// Generic max implementation
fn max_impl<T>(py: Python<'_>, buffer: &PyBuffer<T>) -> PyResult<T>
where
    T: Element + Copy + PartialOrd + Send + Sync,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    let len = slice.len();

    // Fast path for very small arrays
    match len {
        0 => unreachable!(), // Should be checked before calling
        1 => return Ok(slice[0].get()),
        2 => {
            let a = slice[0].get();
            let b = slice[1].get();
            return Ok(if a > b { a } else { b });
        }
        3 => {
            let a = slice[0].get();
            let b = slice[1].get();
            let c = slice[2].get();
            let ab = if a > b { a } else { b };
            return Ok(if ab > c { ab } else { c });
        }
        4 => {
            let a = slice[0].get();
            let b = slice[1].get();
            let c = slice[2].get();
            let d = slice[3].get();
            let ab = if a > b { a } else { b };
            let cd = if c > d { c } else { d };
            return Ok(if ab > cd { ab } else { cd });
        }
        _ => {} // Continue to general case
    }

    #[cfg(feature = "parallel")]
    {
        if should_parallelize(len, PARALLEL_THRESHOLD_MINMAX) {
            let data = extract_buffer_to_vec(py, buffer)?;
            return Ok(data
                .par_iter()
                .copied()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap());
        }
    }

    let mut max_val = slice[0].get();
    for cell in slice.iter().skip(1) {
        let val = cell.get();
        if val > max_val {
            max_val = val;
        }
    }
    Ok(max_val)
}

/// Sum operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn sum(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        match typecode {
            TypeCode::Int8 => return Ok(0i8.into_py(py)),
            TypeCode::Int16 => return Ok(0i16.into_py(py)),
            TypeCode::Int32 => return Ok(0i32.into_py(py)),
            TypeCode::Int64 => {
                let itemsize = get_itemsize(array)?;
                if itemsize == 4 {
                    return Ok(0i32.into_py(py));
                } else {
                    return Ok(0i64.into_py(py));
                }
            }
            TypeCode::UInt8 => return Ok(0u8.into_py(py)),
            TypeCode::UInt16 => return Ok(0u16.into_py(py)),
            TypeCode::UInt32 => return Ok(0u32.into_py(py)),
            TypeCode::UInt64 => {
                let itemsize = get_itemsize(array)?;
                if itemsize == 4 {
                    return Ok(0u32.into_py(py));
                } else {
                    return Ok(0u64.into_py(py));
                }
            }
            TypeCode::Float32 => return Ok(0.0f32.into_py(py)),
            TypeCode::Float64 => return Ok(0.0f64.into_py(py)),
        }
    }

    let len = get_array_len(array)?;
    crate::dispatch_by_typecode!(typecode, array, |buffer| {
        let result = sum_impl(py, &buffer, len)?;
        Ok(result.into_py(py))
    })
}

/// Scale operation (in-place) for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn scale(py: Python<'_>, array: &Bound<'_, PyAny>, factor: f64) -> PyResult<()> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, true)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays early to avoid buffer alignment issues on macOS
    let len = get_array_len(array)?;
    if len == 0 {
        return Ok(());
    }

    match typecode {
        TypeCode::Int8 => {
            let mut buffer = PyBuffer::<i8>::get(array)?;
            scale_impl(py, &mut buffer, factor as i8, len)
        }
        TypeCode::Int16 => {
            let mut buffer = PyBuffer::<i16>::get(array)?;
            scale_impl(py, &mut buffer, factor as i16, len)
        }
        TypeCode::Int32 => {
            let mut buffer = PyBuffer::<i32>::get(array)?;
            scale_impl(py, &mut buffer, factor as i32, len)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let mut buffer = PyBuffer::<i32>::get(array)?;
                scale_impl(py, &mut buffer, factor as i32, len)
            } else {
                let mut buffer = PyBuffer::<i64>::get(array)?;
                scale_impl(py, &mut buffer, factor as i64, len)
            }
        }
        TypeCode::UInt8 => {
            let mut buffer = PyBuffer::<u8>::get(array)?;
            scale_impl(py, &mut buffer, factor as u8, len)
        }
        TypeCode::UInt16 => {
            let mut buffer = PyBuffer::<u16>::get(array)?;
            scale_impl(py, &mut buffer, factor as u16, len)
        }
        TypeCode::UInt32 => {
            let mut buffer = PyBuffer::<u32>::get(array)?;
            scale_impl(py, &mut buffer, factor as u32, len)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let mut buffer = PyBuffer::<u32>::get(array)?;
                scale_impl(py, &mut buffer, factor as u32, len)
            } else {
                let mut buffer = PyBuffer::<u64>::get(array)?;
                scale_impl(py, &mut buffer, factor as u64, len)
            }
        }
        TypeCode::Float32 => {
            let mut buffer = PyBuffer::<f32>::get(array)?;
            scale_impl(py, &mut buffer, factor as f32, len)
        }
        TypeCode::Float64 => {
            let mut buffer = PyBuffer::<f64>::get(array)?;
            scale_impl(py, &mut buffer, factor, len)
        }
    }
}

/// Mean operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn mean(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<f64> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays - raise ValueError
    let len = get_array_len(array)?;
    if len == 0 {
        return Err(PyValueError::new_err("mean() of empty array"));
    }

    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            mean_impl_int(py, &buffer, len)
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            mean_impl_int(py, &buffer, len)
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            mean_impl_int(py, &buffer, len)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<i32>::get(array)?;
                mean_impl_int(py, &buffer, len)
            } else {
                let buffer = PyBuffer::<i64>::get(array)?;
                // Convert sum to f64 explicitly (i64 doesn't have From<i64> for f64)
                let slice = buffer
                    .as_slice(py)
                    .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
                let sum: i64 = slice.iter().map(|cell| cell.get()).sum();
                Ok(sum as f64 / len as f64)
            }
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            mean_impl_int(py, &buffer, len)
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            mean_impl_int(py, &buffer, len)
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            mean_impl_int(py, &buffer, len)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<u32>::get(array)?;
                mean_impl_int(py, &buffer, len)
            } else {
                let buffer = PyBuffer::<u64>::get(array)?;
                // Convert sum to f64 explicitly (u64 doesn't have From<u64> for f64)
                let slice = buffer
                    .as_slice(py)
                    .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
                let sum: u64 = slice.iter().map(|cell| cell.get()).sum();
                Ok(sum as f64 / len as f64)
            }
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            mean_impl_float(py, &buffer, len)
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            mean_impl_float(py, &buffer, len)
        }
    }
}

/// Min operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn min(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays - raise ValueError
    let len = get_array_len(array)?;
    if len == 0 {
        return Err(PyValueError::new_err("min() of empty array"));
    }

    crate::dispatch_by_typecode!(typecode, array, |buffer| {
        let result = min_impl(py, &buffer)?;
        Ok(result.into_py(py))
    })
}

/// Max operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn max(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays - raise ValueError
    let len = get_array_len(array)?;
    if len == 0 {
        return Err(PyValueError::new_err("max() of empty array"));
    }

    crate::dispatch_by_typecode!(typecode, array, |buffer| {
        let result = max_impl(py, &buffer)?;
        Ok(result.into_py(py))
    })
}
