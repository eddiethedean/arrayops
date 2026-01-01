use pyo3::buffer::{Element, PyBuffer};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;

use crate::buffer::{get_array_len, get_itemsize};
use crate::operations::basic;
use crate::types::TypeCode;
use crate::validation::{detect_input_type, get_typecode_unified, validate_for_operation};

// Generic std/var implementation for integer types
fn var_impl_int<T>(py: Python, buffer: &PyBuffer<T>, len: usize, mean_val: f64) -> PyResult<f64>
where
    T: Element + Copy + Send + Sync,
    f64: From<T>,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    let variance: f64 = slice
        .iter()
        .map(|cell| {
            let val = f64::from(cell.get());
            let diff = val - mean_val;
            diff * diff
        })
        .sum::<f64>()
        / len as f64;

    Ok(variance)
}

// Generic std/var implementation for float types
fn var_impl_float<T>(py: Python, buffer: &PyBuffer<T>, len: usize, mean_val: f64) -> PyResult<f64>
where
    T: Element + Copy + Send + Sync,
    f64: From<T>,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    let variance: f64 = slice
        .iter()
        .map(|cell| {
            let val = f64::from(cell.get());
            let diff = val - mean_val;
            diff * diff
        })
        .sum::<f64>()
        / len as f64;

    Ok(variance)
}

/// Variance operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn var(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<f64> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays - raise ValueError
    let len = get_array_len(array)?;
    if len == 0 {
        return Err(PyValueError::new_err("var() of empty array"));
    }

    // Calculate mean first
    let mean_val = basic::mean(py, array)?;

    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            var_impl_int(py, &buffer, len, mean_val)
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            var_impl_int(py, &buffer, len, mean_val)
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            var_impl_int(py, &buffer, len, mean_val)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<i32>::get(array)?;
                var_impl_int(py, &buffer, len, mean_val)
            } else {
                let buffer = PyBuffer::<i64>::get(array)?;
                let slice = buffer
                    .as_slice(py)
                    .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
                let variance: f64 = slice
                    .iter()
                    .map(|cell| {
                        let val = cell.get() as f64;
                        let diff = val - mean_val;
                        diff * diff
                    })
                    .sum::<f64>()
                    / len as f64;
                Ok(variance)
            }
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            var_impl_int(py, &buffer, len, mean_val)
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            var_impl_int(py, &buffer, len, mean_val)
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            var_impl_int(py, &buffer, len, mean_val)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<u32>::get(array)?;
                var_impl_int(py, &buffer, len, mean_val)
            } else {
                let buffer = PyBuffer::<u64>::get(array)?;
                let slice = buffer
                    .as_slice(py)
                    .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
                let variance: f64 = slice
                    .iter()
                    .map(|cell| {
                        let val = cell.get() as f64;
                        let diff = val - mean_val;
                        diff * diff
                    })
                    .sum::<f64>()
                    / len as f64;
                Ok(variance)
            }
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            var_impl_float(py, &buffer, len, mean_val)
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            var_impl_float(py, &buffer, len, mean_val)
        }
    }
}

/// Standard deviation operation for array.array, numpy.ndarray, or memoryview
#[pyfunction(name = "std")]
pub fn std_dev(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<f64> {
    let variance = var(py, array)?;
    Ok(variance.sqrt())
}

// Generic median implementation for integer types (Ord)
fn median_impl_int<T>(py: Python, buffer: &PyBuffer<T>) -> PyResult<T>
where
    T: Element + Copy + Ord + Send + Sync,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    // Extract to Vec and sort
    let mut data: Vec<T> = slice.iter().map(|cell| cell.get()).collect();
    data.sort();

    // Return middle element (lower median for even length)
    // For even length: return element at (len-1)/2 (lower median)
    // For odd length: return element at (len-1)/2 (middle element)
    let mid = (data.len() - 1) / 2;
    Ok(data[mid])
}

// Generic median implementation for float types (PartialOrd)
fn median_impl_float<T>(py: Python, buffer: &PyBuffer<T>) -> PyResult<T>
where
    T: Element + Copy + PartialOrd + Send + Sync,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    // Extract to Vec and sort using PartialOrd
    let mut data: Vec<T> = slice.iter().map(|cell| cell.get()).collect();
    data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Return middle element (lower median for even length)
    // For even length: return element at (len-1)/2 (lower median)
    // For odd length: return element at (len-1)/2 (middle element)
    let mid = (data.len() - 1) / 2;
    Ok(data[mid])
}

/// Median operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn median(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays - raise ValueError
    let len = get_array_len(array)?;
    if len == 0 {
        return Err(PyValueError::new_err("median() of empty array"));
    }

    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            result.into_py_any(py)
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            result.into_py_any(py)
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            result.into_py_any(py)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<i32>::get(array)?;
                let result = median_impl_int(py, &buffer)?;
                result.into_py_any(py)
            } else {
                let buffer = PyBuffer::<i64>::get(array)?;
                let result = median_impl_int(py, &buffer)?;
                result.into_py_any(py)
            }
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            result.into_py_any(py)
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            result.into_py_any(py)
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            result.into_py_any(py)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<u32>::get(array)?;
                let result = median_impl_int(py, &buffer)?;
                result.into_py_any(py)
            } else {
                let buffer = PyBuffer::<u64>::get(array)?;
                let result = median_impl_int(py, &buffer)?;
                result.into_py_any(py)
            }
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            let result = median_impl_float(py, &buffer)?;
            result.into_py_any(py)
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            let result = median_impl_float(py, &buffer)?;
            result.into_py_any(py)
        }
    }
}
