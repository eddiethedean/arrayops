use pyo3::buffer::{Element, PyBuffer};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyList;

use crate::buffer::{
    create_empty_result_array, create_result_array_from_list, get_array_len, get_itemsize,
};
use crate::types::TypeCode;
use crate::validation::{
    detect_input_type, get_typecode_unified, validate_for_operation, InputType,
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

fn reverse_impl<T>(py: Python<'_>, buffer: &mut PyBuffer<T>) -> PyResult<()>
where
    T: Element + Copy + Send + Sync,
{
    let slice = buffer
        .as_mut_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;

    // Extract to Vec, reverse, write back
    let mut data: Vec<T> = slice.iter().map(|cell| cell.get()).collect();
    data.reverse();

    for (item, &val) in slice.iter().zip(data.iter()) {
        item.set(val);
    }
    Ok(())
}
/// Reverse operation (in-place) for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn reverse(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<()> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, true)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays
    let len = get_array_len(array)?;
    if len == 0 {
        return Ok(());
    }

    match typecode {
        TypeCode::Int8 => {
            let mut buffer = PyBuffer::<i8>::get(array)?;
            reverse_impl(py, &mut buffer)
        }
        TypeCode::Int16 => {
            let mut buffer = PyBuffer::<i16>::get(array)?;
            reverse_impl(py, &mut buffer)
        }
        TypeCode::Int32 => {
            let mut buffer = PyBuffer::<i32>::get(array)?;
            reverse_impl(py, &mut buffer)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let mut buffer = PyBuffer::<i32>::get(array)?;
                reverse_impl(py, &mut buffer)
            } else {
                let mut buffer = PyBuffer::<i64>::get(array)?;
                reverse_impl(py, &mut buffer)
            }
        }
        TypeCode::UInt8 => {
            let mut buffer = PyBuffer::<u8>::get(array)?;
            reverse_impl(py, &mut buffer)
        }
        TypeCode::UInt16 => {
            let mut buffer = PyBuffer::<u16>::get(array)?;
            reverse_impl(py, &mut buffer)
        }
        TypeCode::UInt32 => {
            let mut buffer = PyBuffer::<u32>::get(array)?;
            reverse_impl(py, &mut buffer)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let mut buffer = PyBuffer::<u32>::get(array)?;
                reverse_impl(py, &mut buffer)
            } else {
                let mut buffer = PyBuffer::<u64>::get(array)?;
                reverse_impl(py, &mut buffer)
            }
        }
        TypeCode::Float32 => {
            let mut buffer = PyBuffer::<f32>::get(array)?;
            reverse_impl(py, &mut buffer)
        }
        TypeCode::Float64 => {
            let mut buffer = PyBuffer::<f64>::get(array)?;
            reverse_impl(py, &mut buffer)
        }
    }
}

fn sort_impl_int<T>(py: Python, buffer: &mut PyBuffer<T>) -> PyResult<()>
where
    T: Element + Copy + Ord + Send + Sync,
{
    let slice = buffer
        .as_mut_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;

    let len = slice.len();

    // Fast path for very small arrays
    if len <= 1 {
        return Ok(());
    }

    // Extract to Vec, sort, write back
    let mut data: Vec<T> = slice.iter().map(|cell| cell.get()).collect();

    #[cfg(feature = "parallel")]
    {
        // Use parallel sort for larger arrays
        if len >= 10_000 {
            data.par_sort();
        } else {
            data.sort();
        }
    }

    #[cfg(not(feature = "parallel"))]
    {
        data.sort();
    }

    for (item, &val) in slice.iter().zip(data.iter()) {
        item.set(val);
    }
    Ok(())
}

fn sort_impl_float<T>(py: Python, buffer: &mut PyBuffer<T>) -> PyResult<()>
where
    T: Element + Copy + PartialOrd + Send + Sync,
{
    let slice = buffer
        .as_mut_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;

    let len = slice.len();

    // Fast path for very small arrays
    if len <= 1 {
        return Ok(());
    }

    // Extract to Vec, sort using PartialOrd, write back
    let mut data: Vec<T> = slice.iter().map(|cell| cell.get()).collect();

    #[cfg(feature = "parallel")]
    {
        // Use parallel sort for larger arrays
        if len >= 10_000 {
            data.par_sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        } else {
            data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        }
    }

    #[cfg(not(feature = "parallel"))]
    {
        data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    }

    for (item, &val) in slice.iter().zip(data.iter()) {
        item.set(val);
    }
    Ok(())
}

/// Sort operation (in-place) for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn sort(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<()> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, true)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays
    let len = get_array_len(array)?;
    if len == 0 {
        return Ok(());
    }

    match typecode {
        TypeCode::Int8 => {
            let mut buffer = PyBuffer::<i8>::get(array)?;
            sort_impl_int(py, &mut buffer)
        }
        TypeCode::Int16 => {
            let mut buffer = PyBuffer::<i16>::get(array)?;
            sort_impl_int(py, &mut buffer)
        }
        TypeCode::Int32 => {
            let mut buffer = PyBuffer::<i32>::get(array)?;
            sort_impl_int(py, &mut buffer)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let mut buffer = PyBuffer::<i32>::get(array)?;
                sort_impl_int(py, &mut buffer)
            } else {
                let mut buffer = PyBuffer::<i64>::get(array)?;
                sort_impl_int(py, &mut buffer)
            }
        }
        TypeCode::UInt8 => {
            let mut buffer = PyBuffer::<u8>::get(array)?;
            sort_impl_int(py, &mut buffer)
        }
        TypeCode::UInt16 => {
            let mut buffer = PyBuffer::<u16>::get(array)?;
            sort_impl_int(py, &mut buffer)
        }
        TypeCode::UInt32 => {
            let mut buffer = PyBuffer::<u32>::get(array)?;
            sort_impl_int(py, &mut buffer)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let mut buffer = PyBuffer::<u32>::get(array)?;
                sort_impl_int(py, &mut buffer)
            } else {
                let mut buffer = PyBuffer::<u64>::get(array)?;
                sort_impl_int(py, &mut buffer)
            }
        }
        TypeCode::Float32 => {
            let mut buffer = PyBuffer::<f32>::get(array)?;
            sort_impl_float(py, &mut buffer)
        }
        TypeCode::Float64 => {
            let mut buffer = PyBuffer::<f64>::get(array)?;
            sort_impl_float(py, &mut buffer)
        }
    }
}

fn unique_impl_int<T>(
    py: Python,
    buffer: &PyBuffer<T>,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + Ord + Send + Sync + IntoPy<PyObject>,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    // Extract to Vec, sort, deduplicate
    let mut data: Vec<T> = slice.iter().map(|cell| cell.get()).collect();
    data.sort();
    data.dedup();

    let result_list = PyList::empty(py);
    for val in data {
        result_list.append(val.into_py(py))?;
    }

    create_result_array_from_list(py, typecode, input_type, &result_list)
}

fn unique_impl_float<T>(
    py: Python,
    buffer: &PyBuffer<T>,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + PartialOrd + Send + Sync + IntoPy<PyObject>,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    // Extract to Vec, sort, deduplicate
    let mut data: Vec<T> = slice.iter().map(|cell| cell.get()).collect();
    data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    data.dedup_by(|a, b| (*a).partial_cmp(b) == Some(std::cmp::Ordering::Equal));

    let result_list = PyList::empty(py);
    for val in data {
        result_list.append(val.into_py(py))?;
    }

    create_result_array_from_list(py, typecode, input_type, &result_list)
}

/// Unique operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn unique(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays
    let len = get_array_len(array)?;
    if len == 0 {
        return create_empty_result_array(py, typecode, input_type);
    }

    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            unique_impl_int(py, &buffer, typecode, input_type)
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            unique_impl_int(py, &buffer, typecode, input_type)
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            unique_impl_int(py, &buffer, typecode, input_type)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<i32>::get(array)?;
                unique_impl_int(py, &buffer, typecode, input_type)
            } else {
                let buffer = PyBuffer::<i64>::get(array)?;
                unique_impl_int(py, &buffer, typecode, input_type)
            }
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            unique_impl_int(py, &buffer, typecode, input_type)
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            unique_impl_int(py, &buffer, typecode, input_type)
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            unique_impl_int(py, &buffer, typecode, input_type)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<u32>::get(array)?;
                unique_impl_int(py, &buffer, typecode, input_type)
            } else {
                let buffer = PyBuffer::<u64>::get(array)?;
                unique_impl_int(py, &buffer, typecode, input_type)
            }
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            unique_impl_float(py, &buffer, typecode, input_type)
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            unique_impl_float(py, &buffer, typecode, input_type)
        }
    }
}
