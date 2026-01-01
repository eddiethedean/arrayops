use pyo3::buffer::{Element, PyBuffer};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::IntoPyObjectExt;

#[cfg(feature = "parallel")]
#[allow(unused_imports)] // Only used when parallel feature is enabled
use rayon::prelude::*;

use crate::types::TypeCode;
use crate::validation::InputType;

/// Get the length of an array.array
pub(crate) fn get_array_len(array: &Bound<'_, PyAny>) -> PyResult<usize> {
    let len = array.len()?;
    Ok(len)
}

/// Get itemsize of an array.array, numpy.ndarray, or memoryview
pub(crate) fn get_itemsize(array: &Bound<'_, PyAny>) -> PyResult<usize> {
    let itemsize: usize = array.getattr("itemsize")?.extract()?;
    Ok(itemsize)
}

/// Helper function to extract an element at a specific index from a buffer
pub(crate) fn extract_element_at_index<T>(
    py: Python<'_>,
    source_ref: &Bound<'_, PyAny>,
    index: usize,
) -> PyResult<PyObject>
where
    T: Element + Copy + for<'py> IntoPyObject<'py>,
{
    let buffer = PyBuffer::<T>::get(source_ref)?;
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
    slice[index].get().into_py_any(py)
}

/// Create an empty result array based on input type
pub(crate) fn create_empty_result_array(
    py: Python<'_>,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject> {
    match input_type {
        InputType::NumPyArray => {
            // Create numpy array with same dtype
            let numpy_module = PyModule::import(py, "numpy")?;
            let numpy_array = numpy_module.getattr("array")?;
            // Map TypeCode to numpy dtype string
            let dtype_str = match typecode {
                TypeCode::Int8 => "int8",
                TypeCode::Int16 => "int16",
                TypeCode::Int32 => "int32",
                TypeCode::Int64 => "int64",
                TypeCode::UInt8 => "uint8",
                TypeCode::UInt16 => "uint16",
                TypeCode::UInt32 => "uint32",
                TypeCode::UInt64 => "uint64",
                TypeCode::Float32 => "float32",
                TypeCode::Float64 => "float64",
            };
            let dtype = numpy_module.getattr("dtype")?.call1((dtype_str,))?;
            let empty_list = PyList::empty(py);
            let arr = numpy_array.call1((empty_list,))?;
            Ok(arr.call_method1("astype", (dtype,))?.into())
        }
        InputType::ArrayArray | InputType::MemoryView => {
            // Create array.array
            let array_module = PyModule::import(py, "array")?;
            let array_type = array_module.getattr("array")?;
            let typecode_char = typecode.as_char();
            let empty_list = PyList::empty(py);
            Ok(array_type.call1((typecode_char, empty_list))?.into())
        }
        InputType::ArrowBuffer => {
            // Create empty Arrow array
            let pyarrow_module = PyModule::import(py, "pyarrow")?;
            let array_func = pyarrow_module.getattr("array")?;
            // Create empty list and convert to Arrow array
            let empty_list = PyList::empty(py);
            Ok(array_func.call1((empty_list,))?.into())
        }
    }
}

/// Create result array from list based on input type
pub(crate) fn create_result_array_from_list(
    py: Python<'_>,
    typecode: TypeCode,
    input_type: InputType,
    values: &Bound<'_, PyList>,
) -> PyResult<PyObject> {
    match input_type {
        InputType::NumPyArray => {
            // Create numpy array from list
            let numpy_module = PyModule::import(py, "numpy")?;
            let numpy_array = numpy_module.getattr("array")?;
            // Map TypeCode to numpy dtype string
            let dtype_str = match typecode {
                TypeCode::Int8 => "int8",
                TypeCode::Int16 => "int16",
                TypeCode::Int32 => "int32",
                TypeCode::Int64 => "int64",
                TypeCode::UInt8 => "uint8",
                TypeCode::UInt16 => "uint16",
                TypeCode::UInt32 => "uint32",
                TypeCode::UInt64 => "uint64",
                TypeCode::Float32 => "float32",
                TypeCode::Float64 => "float64",
            };
            let dtype = numpy_module.getattr("dtype")?.call1((dtype_str,))?;
            let arr = numpy_array.call1((values,))?;
            Ok(arr.call_method1("astype", (dtype,))?.into())
        }
        InputType::ArrayArray | InputType::MemoryView => {
            // Create array.array
            let array_module = PyModule::import(py, "array")?;
            let array_type = array_module.getattr("array")?;
            let typecode_char = typecode.as_char();
            Ok(array_type.call1((typecode_char, values))?.into())
        }
        InputType::ArrowBuffer => {
            // Create Arrow array from list
            let pyarrow_module = PyModule::import(py, "pyarrow")?;
            let array_func = pyarrow_module.getattr("array")?;
            // Arrow arrays can be created from Python lists
            Ok(array_func.call1((values,))?.into())
        }
    }
}

/// Create result array from Rust Vec (much faster than building PyList incrementally)
pub(crate) fn create_result_array_from_vec<T>(
    py: Python<'_>,
    typecode: TypeCode,
    input_type: InputType,
    values: Vec<T>,
) -> PyResult<PyObject>
where
    T: Copy + for<'py> IntoPyObject<'py>,
{
    match input_type {
        InputType::NumPyArray => {
            // Create numpy array from Vec
            let numpy_module = PyModule::import(py, "numpy")?;
            let numpy_array = numpy_module.getattr("array")?;
            // Map TypeCode to numpy dtype string
            let dtype_str = match typecode {
                TypeCode::Int8 => "int8",
                TypeCode::Int16 => "int16",
                TypeCode::Int32 => "int32",
                TypeCode::Int64 => "int64",
                TypeCode::UInt8 => "uint8",
                TypeCode::UInt16 => "uint16",
                TypeCode::UInt32 => "uint32",
                TypeCode::UInt64 => "uint64",
                TypeCode::Float32 => "float32",
                TypeCode::Float64 => "float64",
            };
            let dtype = numpy_module.getattr("dtype")?.call1((dtype_str,))?;
            // Convert Vec to PyList once (much faster than incremental append)
            let py_list = PyList::new(py, values.iter().copied())?;
            let arr = numpy_array.call1((&py_list,))?;
            Ok(arr.call_method1("astype", (dtype,))?.into())
        }
        InputType::ArrayArray | InputType::MemoryView => {
            // Create array.array from Vec
            let array_module = PyModule::import(py, "array")?;
            let array_type = array_module.getattr("array")?;
            let typecode_char = typecode.as_char();
            // Convert Vec to PyList once (much faster than incremental append)
            let py_list = PyList::new(py, values.iter().copied())?;
            Ok(array_type.call1((typecode_char, &py_list))?.into())
        }
        InputType::ArrowBuffer => {
            // Create Arrow array from Vec
            let pyarrow_module = PyModule::import(py, "pyarrow")?;
            let array_func = pyarrow_module.getattr("array")?;
            // Convert Vec to PyList once
            let py_list = PyList::new(py, values.iter().copied())?;
            // Arrow arrays can be created from Python lists
            Ok(array_func.call1((&py_list,))?.into())
        }
    }
}

/// Create a memoryview slice from a buffer (helper function for zero-copy slicing)
/// Note: This is used by the slice() function. For filter/map operations, views are
/// not applicable because they change data/size
pub(crate) fn create_slice_view_helper(
    py: Python<'_>,
    array: &Bound<'_, PyAny>,
    start: usize,
    end: usize,
) -> PyResult<PyObject> {
    // Create memoryview from array (zero-copy view)
    let builtins = PyModule::import(py, "builtins")?;
    let memoryview_type = builtins.getattr("memoryview")?;
    let full_view = memoryview_type.call1((array,))?;

    // Use Python's slice object for slicing the memoryview
    let slice_type = builtins.getattr("slice")?;
    let py_slice = slice_type.call1((start, end, None::<()>))?;

    // Call __getitem__ on the memoryview with the slice object
    let slice_obj = full_view.call_method1("__getitem__", (py_slice,))?;
    Ok(slice_obj.into())
}

// Parallel execution thresholds (lowered for better performance)
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
pub(crate) const PARALLEL_THRESHOLD_SUM: usize = 1_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
pub(crate) const PARALLEL_THRESHOLD_SCALE: usize = 1_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
pub(crate) const PARALLEL_THRESHOLD_MEAN: usize = 1_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
pub(crate) const PARALLEL_THRESHOLD_MINMAX: usize = 50_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
pub(crate) const PARALLEL_THRESHOLD_ADD: usize = 1_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
pub(crate) const PARALLEL_THRESHOLD_MULTIPLY: usize = 1_000;
#[allow(dead_code)] // Reserved for future parallel implementation
pub(crate) const PARALLEL_THRESHOLD_CLIP: usize = 1_000;
#[allow(dead_code)] // Reserved for future parallel implementation
pub(crate) const PARALLEL_THRESHOLD_NORMALIZE: usize = 2_000;
// Note: MAP, FILTER, and REDUCE thresholds reserved for future use
// (parallel execution for these operations is limited by Python's GIL)
#[allow(dead_code)]
pub(crate) const PARALLEL_THRESHOLD_MAP: usize = 10_000;
#[allow(dead_code)]
pub(crate) const PARALLEL_THRESHOLD_FILTER: usize = 10_000;
#[allow(dead_code)]
pub(crate) const PARALLEL_THRESHOLD_REDUCE: usize = 10_000;

/// Extract buffer data to Vec for parallel processing
#[cfg(feature = "parallel")]
pub(crate) fn extract_buffer_to_vec<T>(py: Python, buffer: &PyBuffer<T>) -> PyResult<Vec<T>>
where
    T: Element + Copy,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
    Ok(slice.iter().map(|cell| cell.get()).collect())
}

/// Check if array should be parallelized based on length and threshold
#[cfg(feature = "parallel")]
pub(crate) fn should_parallelize(len: usize, threshold: usize) -> bool {
    len >= threshold
}

// Cache blocking constants
// Typical L1 cache is 32KB, L2 is 256KB-1MB
// Process arrays in cache-friendly chunks to improve memory access patterns
pub(crate) const CACHE_BLOCK_SIZE: usize = 8192; // ~8KB blocks (works well for most cache sizes)
