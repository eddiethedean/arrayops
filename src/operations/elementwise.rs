use pyo3::buffer::{Element, PyBuffer};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

use crate::buffer::{
    create_empty_result_array, create_result_array_from_vec, get_array_len, get_itemsize,
};
use crate::operations::basic;
use crate::types::TypeCode;
use crate::validation::{
    detect_input_type, get_typecode_unified, validate_for_operation, InputType,
};

#[cfg(feature = "parallel")]
use crate::buffer::{
    extract_buffer_to_vec, should_parallelize, PARALLEL_THRESHOLD_ADD, PARALLEL_THRESHOLD_MULTIPLY,
};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[allow(unused_variables)] // len is only used when parallel feature is enabled
fn add_impl<T>(
    py: Python,
    buffer1: &PyBuffer<T>,
    buffer2: &PyBuffer<T>,
    len: usize,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + std::ops::Add<Output = T> + Send + Sync + IntoPy<PyObject>,
{
    #[cfg(feature = "parallel")]
    {
        if should_parallelize(len, PARALLEL_THRESHOLD_ADD) {
            let data1 = extract_buffer_to_vec(py, buffer1)?;
            let data2 = extract_buffer_to_vec(py, buffer2)?;
            let result_vec: Vec<T> = data1
                .par_iter()
                .zip(data2.par_iter())
                .map(|(a, b)| *a + *b)
                .collect();
            return create_result_array_from_vec(py, typecode, input_type, result_vec);
        }
    }

    let slice1 = buffer1
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
    let slice2 = buffer2
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    // Build result in Vec<T> using iterators (much faster than PyList::append)
    let result_vec: Vec<T> = slice1
        .iter()
        .zip(slice2.iter())
        .map(|(a, b)| a.get() + b.get())
        .collect();

    create_result_array_from_vec(py, typecode, input_type, result_vec)
}

#[allow(unused_variables)] // len is only used when parallel feature is enabled
fn multiply_impl<T>(
    py: Python,
    buffer1: &PyBuffer<T>,
    buffer2: &PyBuffer<T>,
    len: usize,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + std::ops::Mul<Output = T> + Send + Sync + IntoPy<PyObject>,
{
    #[cfg(feature = "parallel")]
    {
        if should_parallelize(len, PARALLEL_THRESHOLD_MULTIPLY) {
            let data1 = extract_buffer_to_vec(py, buffer1)?;
            let data2 = extract_buffer_to_vec(py, buffer2)?;
            let result_vec: Vec<T> = data1
                .par_iter()
                .zip(data2.par_iter())
                .map(|(a, b)| *a * *b)
                .collect();
            return create_result_array_from_vec(py, typecode, input_type, result_vec);
        }
    }

    let slice1 = buffer1
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
    let slice2 = buffer2
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    // Build result in Vec<T> using iterators (much faster than PyList::append)
    let result_vec: Vec<T> = slice1
        .iter()
        .zip(slice2.iter())
        .map(|(a, b)| a.get() * b.get())
        .collect();

    create_result_array_from_vec(py, typecode, input_type, result_vec)
}

#[pyfunction]
pub fn add(py: Python<'_>, arr1: &Bound<'_, PyAny>, arr2: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let input_type1 = detect_input_type(arr1)?;
    validate_for_operation(arr1, input_type1, false)?;
    let typecode1 = get_typecode_unified(arr1, input_type1)?;

    let input_type2 = detect_input_type(arr2)?;
    validate_for_operation(arr2, input_type2, false)?;
    let typecode2 = get_typecode_unified(arr2, input_type2)?;

    // Check types match
    if typecode1 != typecode2 {
        return Err(PyTypeError::new_err(
            "Arrays must have the same type for element-wise operations",
        ));
    }

    // Check lengths match
    let len1 = get_array_len(arr1)?;
    let len2 = get_array_len(arr2)?;
    if len1 != len2 {
        return Err(PyValueError::new_err(
            "Arrays must have the same length for element-wise operations",
        ));
    }

    // Determine result type (NumPy if both NumPy, otherwise array.array)
    let result_type =
        if input_type1 == InputType::NumPyArray && input_type2 == InputType::NumPyArray {
            InputType::NumPyArray
        } else {
            InputType::ArrayArray
        };

    // Handle empty arrays
    if len1 == 0 {
        return create_empty_result_array(py, typecode1, result_type);
    }

    match typecode1 {
        TypeCode::Int8 => {
            let buffer1 = PyBuffer::<i8>::get(arr1)?;
            let buffer2 = PyBuffer::<i8>::get(arr2)?;
            add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::Int16 => {
            let buffer1 = PyBuffer::<i16>::get(arr1)?;
            let buffer2 = PyBuffer::<i16>::get(arr2)?;
            add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::Int32 => {
            let buffer1 = PyBuffer::<i32>::get(arr1)?;
            let buffer2 = PyBuffer::<i32>::get(arr2)?;
            add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::Int64 => {
            let itemsize1 = get_itemsize(arr1)?;
            let itemsize2 = get_itemsize(arr2)?;
            if itemsize1 == 4 && itemsize2 == 4 {
                let buffer1 = PyBuffer::<i32>::get(arr1)?;
                let buffer2 = PyBuffer::<i32>::get(arr2)?;
                add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
            } else if itemsize1 == 8 && itemsize2 == 8 {
                let buffer1 = PyBuffer::<i64>::get(arr1)?;
                let buffer2 = PyBuffer::<i64>::get(arr2)?;
                add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
            } else {
                Err(PyTypeError::new_err("Array itemsizes must match"))
            }
        }
        TypeCode::UInt8 => {
            let buffer1 = PyBuffer::<u8>::get(arr1)?;
            let buffer2 = PyBuffer::<u8>::get(arr2)?;
            add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::UInt16 => {
            let buffer1 = PyBuffer::<u16>::get(arr1)?;
            let buffer2 = PyBuffer::<u16>::get(arr2)?;
            add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::UInt32 => {
            let buffer1 = PyBuffer::<u32>::get(arr1)?;
            let buffer2 = PyBuffer::<u32>::get(arr2)?;
            add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::UInt64 => {
            let itemsize1 = get_itemsize(arr1)?;
            let itemsize2 = get_itemsize(arr2)?;
            if itemsize1 == 4 && itemsize2 == 4 {
                let buffer1 = PyBuffer::<u32>::get(arr1)?;
                let buffer2 = PyBuffer::<u32>::get(arr2)?;
                add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
            } else if itemsize1 == 8 && itemsize2 == 8 {
                let buffer1 = PyBuffer::<u64>::get(arr1)?;
                let buffer2 = PyBuffer::<u64>::get(arr2)?;
                add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
            } else {
                Err(PyTypeError::new_err("Array itemsizes must match"))
            }
        }
        TypeCode::Float32 => {
            let buffer1 = PyBuffer::<f32>::get(arr1)?;
            let buffer2 = PyBuffer::<f32>::get(arr2)?;
            add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::Float64 => {
            let buffer1 = PyBuffer::<f64>::get(arr1)?;
            let buffer2 = PyBuffer::<f64>::get(arr2)?;
            add_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
    }
}

#[pyfunction]
pub fn multiply(py: Python<'_>, arr1: &Bound<'_, PyAny>, arr2: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    let input_type1 = detect_input_type(arr1)?;
    validate_for_operation(arr1, input_type1, false)?;
    let typecode1 = get_typecode_unified(arr1, input_type1)?;

    let input_type2 = detect_input_type(arr2)?;
    validate_for_operation(arr2, input_type2, false)?;
    let typecode2 = get_typecode_unified(arr2, input_type2)?;

    // Check types match
    if typecode1 != typecode2 {
        return Err(PyTypeError::new_err(
            "Arrays must have the same type for element-wise operations",
        ));
    }

    // Check lengths match
    let len1 = get_array_len(arr1)?;
    let len2 = get_array_len(arr2)?;
    if len1 != len2 {
        return Err(PyValueError::new_err(
            "Arrays must have the same length for element-wise operations",
        ));
    }

    // Determine result type (NumPy if both NumPy, otherwise array.array)
    let result_type =
        if input_type1 == InputType::NumPyArray && input_type2 == InputType::NumPyArray {
            InputType::NumPyArray
        } else {
            InputType::ArrayArray
        };

    // Handle empty arrays
    if len1 == 0 {
        return create_empty_result_array(py, typecode1, result_type);
    }

    match typecode1 {
        TypeCode::Int8 => {
            let buffer1 = PyBuffer::<i8>::get(arr1)?;
            let buffer2 = PyBuffer::<i8>::get(arr2)?;
            multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::Int16 => {
            let buffer1 = PyBuffer::<i16>::get(arr1)?;
            let buffer2 = PyBuffer::<i16>::get(arr2)?;
            multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::Int32 => {
            let buffer1 = PyBuffer::<i32>::get(arr1)?;
            let buffer2 = PyBuffer::<i32>::get(arr2)?;
            multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::Int64 => {
            let itemsize1 = get_itemsize(arr1)?;
            let itemsize2 = get_itemsize(arr2)?;
            if itemsize1 == 4 && itemsize2 == 4 {
                let buffer1 = PyBuffer::<i32>::get(arr1)?;
                let buffer2 = PyBuffer::<i32>::get(arr2)?;
                multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
            } else if itemsize1 == 8 && itemsize2 == 8 {
                let buffer1 = PyBuffer::<i64>::get(arr1)?;
                let buffer2 = PyBuffer::<i64>::get(arr2)?;
                multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
            } else {
                Err(PyTypeError::new_err("Array itemsizes must match"))
            }
        }
        TypeCode::UInt8 => {
            let buffer1 = PyBuffer::<u8>::get(arr1)?;
            let buffer2 = PyBuffer::<u8>::get(arr2)?;
            multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::UInt16 => {
            let buffer1 = PyBuffer::<u16>::get(arr1)?;
            let buffer2 = PyBuffer::<u16>::get(arr2)?;
            multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::UInt32 => {
            let buffer1 = PyBuffer::<u32>::get(arr1)?;
            let buffer2 = PyBuffer::<u32>::get(arr2)?;
            multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::UInt64 => {
            let itemsize1 = get_itemsize(arr1)?;
            let itemsize2 = get_itemsize(arr2)?;
            if itemsize1 == 4 && itemsize2 == 4 {
                let buffer1 = PyBuffer::<u32>::get(arr1)?;
                let buffer2 = PyBuffer::<u32>::get(arr2)?;
                multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
            } else if itemsize1 == 8 && itemsize2 == 8 {
                let buffer1 = PyBuffer::<u64>::get(arr1)?;
                let buffer2 = PyBuffer::<u64>::get(arr2)?;
                multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
            } else {
                Err(PyTypeError::new_err("Array itemsizes must match"))
            }
        }
        TypeCode::Float32 => {
            let buffer1 = PyBuffer::<f32>::get(arr1)?;
            let buffer2 = PyBuffer::<f32>::get(arr2)?;
            multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
        TypeCode::Float64 => {
            let buffer1 = PyBuffer::<f64>::get(arr1)?;
            let buffer2 = PyBuffer::<f64>::get(arr2)?;
            multiply_impl(py, &buffer1, &buffer2, len1, typecode1, result_type)
        }
    }
}

/// Clip operation (in-place) for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn clip(py: Python<'_>, array: &Bound<'_, PyAny>, min_val: f64, max_val: f64) -> PyResult<()> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, true)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays
    let len = get_array_len(array)?;
    if len == 0 {
        return Ok(());
    }

    // Validate min <= max
    if min_val > max_val {
        return Err(PyValueError::new_err("min_val must be <= max_val"));
    }

    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            let slice = buffer
                .as_mut_slice(py)
                .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
            for item in slice.iter() {
                let val = item.get() as f64;
                let clipped = if val < min_val {
                    min_val
                } else if val > max_val {
                    max_val
                } else {
                    val
                };
                item.set(clipped as i8);
            }
            Ok(())
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            let slice = buffer
                .as_mut_slice(py)
                .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
            for item in slice.iter() {
                let val = item.get() as f64;
                let clipped = if val < min_val {
                    min_val
                } else if val > max_val {
                    max_val
                } else {
                    val
                };
                item.set(clipped as i16);
            }
            Ok(())
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            let slice = buffer
                .as_mut_slice(py)
                .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
            for item in slice.iter() {
                let val = item.get() as f64;
                let clipped = if val < min_val {
                    min_val
                } else if val > max_val {
                    max_val
                } else {
                    val
                };
                item.set(clipped as i32);
            }
            Ok(())
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<i32>::get(array)?;
                let slice = buffer
                    .as_mut_slice(py)
                    .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
                for item in slice.iter() {
                    let val = item.get() as f64;
                    let clipped = if val < min_val {
                        min_val
                    } else if val > max_val {
                        max_val
                    } else {
                        val
                    };
                    item.set(clipped as i32);
                }
                Ok(())
            } else {
                let buffer = PyBuffer::<i64>::get(array)?;
                let slice = buffer
                    .as_mut_slice(py)
                    .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
                for item in slice.iter() {
                    let val = item.get() as f64;
                    let clipped = if val < min_val {
                        min_val
                    } else if val > max_val {
                        max_val
                    } else {
                        val
                    };
                    item.set(clipped as i64);
                }
                Ok(())
            }
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            let slice = buffer
                .as_mut_slice(py)
                .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
            for item in slice.iter() {
                let val = item.get() as f64;
                let clipped = if val < min_val {
                    min_val
                } else if val > max_val {
                    max_val
                } else {
                    val
                };
                item.set(clipped as u8);
            }
            Ok(())
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            let slice = buffer
                .as_mut_slice(py)
                .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
            for item in slice.iter() {
                let val = item.get() as f64;
                let clipped = if val < min_val {
                    min_val
                } else if val > max_val {
                    max_val
                } else {
                    val
                };
                item.set(clipped as u16);
            }
            Ok(())
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            let slice = buffer
                .as_mut_slice(py)
                .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
            for item in slice.iter() {
                let val = item.get() as f64;
                let clipped = if val < min_val {
                    min_val
                } else if val > max_val {
                    max_val
                } else {
                    val
                };
                item.set(clipped as u32);
            }
            Ok(())
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<u32>::get(array)?;
                let slice = buffer
                    .as_mut_slice(py)
                    .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
                for item in slice.iter() {
                    let val = item.get() as f64;
                    let clipped = if val < min_val {
                        min_val
                    } else if val > max_val {
                        max_val
                    } else {
                        val
                    };
                    item.set(clipped as u32);
                }
                Ok(())
            } else {
                let buffer = PyBuffer::<u64>::get(array)?;
                let slice = buffer
                    .as_mut_slice(py)
                    .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
                for item in slice.iter() {
                    let val = item.get() as f64;
                    let clipped = if val < min_val {
                        min_val
                    } else if val > max_val {
                        max_val
                    } else {
                        val
                    };
                    item.set(clipped as u64);
                }
                Ok(())
            }
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            let slice = buffer
                .as_mut_slice(py)
                .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
            for item in slice.iter() {
                let val = item.get();
                let clipped = if val < min_val as f32 {
                    min_val as f32
                } else if val > max_val as f32 {
                    max_val as f32
                } else {
                    val
                };
                item.set(clipped);
            }
            Ok(())
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            let slice = buffer
                .as_mut_slice(py)
                .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
            for item in slice.iter() {
                let val = item.get();
                let clipped = if val < min_val {
                    min_val
                } else if val > max_val {
                    max_val
                } else {
                    val
                };
                item.set(clipped);
            }
            Ok(())
        }
    }
}

/// Normalize operation (in-place) for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn normalize(py: Python<'_>, array: &Bound<'_, PyAny>) -> PyResult<()> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, true)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays
    let len = get_array_len(array)?;
    if len == 0 {
        return Ok(());
    }

    // Get min and max using our min/max functions
    let min_val_py = basic::min(py, array)?;
    let max_val_py = basic::max(py, array)?;

    // Extract min and max as f64
    let min_val: f64 = min_val_py.extract(py)?;
    let max_val: f64 = max_val_py.extract(py)?;

    // Check if min == max (all values are the same)
    if (max_val - min_val).abs() < f64::EPSILON {
        // All values are the same, set to 0.0 (or could set to 0.5, but 0.0 is more common)
        match typecode {
            TypeCode::Float32 => {
                let buffer = PyBuffer::<f32>::get(array)?;
                let slice = buffer
                    .as_mut_slice(py)
                    .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
                for item in slice.iter() {
                    item.set(0.0f32);
                }
            }
            TypeCode::Float64 => {
                let buffer = PyBuffer::<f64>::get(array)?;
                let slice = buffer
                    .as_mut_slice(py)
                    .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
                for item in slice.iter() {
                    item.set(0.0f64);
                }
            }
            _ => {
                // For integer types, we can't set to 0.0, so return error
                return Err(PyValueError::new_err(
                    "Cannot normalize integer array where all values are the same",
                ));
            }
        }
        return Ok(());
    }

    let range = max_val - min_val;

    match typecode {
        TypeCode::Int8 => Err(PyValueError::new_err(
            "normalize() requires float arrays (use 'f' or 'd' typecode)",
        )),
        TypeCode::Int16 => Err(PyValueError::new_err(
            "normalize() requires float arrays (use 'f' or 'd' typecode)",
        )),
        TypeCode::Int32 => Err(PyValueError::new_err(
            "normalize() requires float arrays (use 'f' or 'd' typecode)",
        )),
        TypeCode::Int64 => Err(PyValueError::new_err(
            "normalize() requires float arrays (use 'f' or 'd' typecode)",
        )),
        TypeCode::UInt8 => Err(PyValueError::new_err(
            "normalize() requires float arrays (use 'f' or 'd' typecode)",
        )),
        TypeCode::UInt16 => Err(PyValueError::new_err(
            "normalize() requires float arrays (use 'f' or 'd' typecode)",
        )),
        TypeCode::UInt32 => Err(PyValueError::new_err(
            "normalize() requires float arrays (use 'f' or 'd' typecode)",
        )),
        TypeCode::UInt64 => Err(PyValueError::new_err(
            "normalize() requires float arrays (use 'f' or 'd' typecode)",
        )),
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            let slice = buffer
                .as_mut_slice(py)
                .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
            for item in slice.iter() {
                let val = item.get();
                let normalized = (val - min_val as f32) / (range as f32);
                item.set(normalized);
            }
            Ok(())
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            let slice = buffer
                .as_mut_slice(py)
                .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
            for item in slice.iter() {
                let val = item.get();
                let normalized = (val - min_val) / range;
                item.set(normalized);
            }
            Ok(())
        }
    }
}
