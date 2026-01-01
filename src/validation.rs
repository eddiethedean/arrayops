use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

use crate::types::{
    get_arrow_typecode, get_memoryview_typecode, get_numpy_typecode, get_typecode, TypeCode,
};

/// Input type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    ArrayArray,
    NumPyArray,
    MemoryView,
    ArrowBuffer, // Apache Arrow buffer/array
}

/// Detect the input type (array.array, numpy.ndarray, memoryview, or Arrow buffer/array)
pub(crate) fn detect_input_type(obj: &PyAny) -> PyResult<InputType> {
    // Check array.array first (maintain backward compatibility)
    let module = PyModule::import(obj.py(), "array")?;
    if let Ok(array_type) = module.getattr("array") {
        if obj.is_instance(array_type)? {
            return Ok(InputType::ArrayArray);
        }
    }

    // Check numpy.ndarray (graceful handling if NumPy not available)
    if let Ok(numpy_module) = PyModule::import(obj.py(), "numpy") {
        if let Ok(ndarray_type) = numpy_module.getattr("ndarray") {
            if obj.is_instance(ndarray_type)? {
                return Ok(InputType::NumPyArray);
            }
        }
    }

    // Check memoryview (built-in type)
    // Use Python's builtins to get memoryview type
    let builtins = PyModule::import(obj.py(), "builtins")?;
    if let Ok(memoryview_type) = builtins.getattr("memoryview") {
        if obj.is_instance(memoryview_type)? {
            return Ok(InputType::MemoryView);
        }
    }

    // Check for Arrow buffer/array (pyarrow.Buffer or pyarrow.Array)
    if let Ok(pyarrow_module) = PyModule::import(obj.py(), "pyarrow") {
        // Check for Buffer
        if let Ok(buffer_type) = pyarrow_module.getattr("Buffer") {
            if obj.is_instance(buffer_type)? {
                return Ok(InputType::ArrowBuffer);
            }
        }
        // Check for Array
        if let Ok(array_type) = pyarrow_module.getattr("Array") {
            if obj.is_instance(array_type)? {
                return Ok(InputType::ArrowBuffer);
            }
        }
        // Check for ChunkedArray
        if let Ok(chunked_array_type) = pyarrow_module.getattr("ChunkedArray") {
            if obj.is_instance(chunked_array_type)? {
                return Ok(InputType::ArrowBuffer);
            }
        }
    }

    Err(PyTypeError::new_err(
        "Expected array.array, numpy.ndarray, memoryview, or Arrow buffer/array",
    ))
}

/// Get typecode from unified input (array.array, numpy.ndarray, memoryview, or Arrow buffer/array)
pub(crate) fn get_typecode_unified(obj: &PyAny, input_type: InputType) -> PyResult<TypeCode> {
    match input_type {
        InputType::ArrayArray => get_typecode(obj),
        InputType::NumPyArray => get_numpy_typecode(obj),
        InputType::MemoryView => get_memoryview_typecode(obj),
        InputType::ArrowBuffer => get_arrow_typecode(obj),
    }
}

/// Validate that the input is an array.array
pub(crate) fn validate_array_array(array: &PyAny) -> PyResult<()> {
    let module = PyModule::import(array.py(), "array")?;
    let array_type = module.getattr("array")?;
    if !array.is_instance(array_type)? {
        return Err(PyTypeError::new_err(
            "Expected array.array, numpy.ndarray, or memoryview",
        ));
    }
    Ok(())
}

/// Validate numpy.ndarray (1D, contiguous)
pub(crate) fn validate_numpy_array(arr: &PyAny) -> PyResult<()> {
    // Check if it's a numpy array (should already be detected, but double-check)
    let numpy_module = PyModule::import(arr.py(), "numpy")?;
    let ndarray_type = numpy_module.getattr("ndarray")?;
    if !arr.is_instance(ndarray_type)? {
        return Err(PyTypeError::new_err("Expected numpy.ndarray"));
    }

    // Check dimensions (must be 1D)
    let ndim: i32 = arr.getattr("ndim")?.extract()?;
    if ndim != 1 {
        return Err(PyTypeError::new_err(
            "numpy.ndarray must be 1-dimensional (ndim == 1)",
        ));
    }

    // Check contiguity (must be C_CONTIGUOUS or F_CONTIGUOUS)
    // NumPy flags object - use lowercase attribute names (c_contiguous, f_contiguous)
    let flags = arr.getattr("flags")?;
    let c_contiguous: bool = flags.getattr("c_contiguous")?.extract()?;
    let f_contiguous: bool = flags.getattr("f_contiguous")?.extract()?;
    if !c_contiguous && !f_contiguous {
        return Err(PyTypeError::new_err(
            "numpy.ndarray must be contiguous (C_CONTIGUOUS or F_CONTIGUOUS)",
        ));
    }

    Ok(())
}

/// Validate memoryview
pub(crate) fn validate_memoryview(mv: &PyAny) -> PyResult<()> {
    // Check if it's a memoryview (should already be detected, but double-check)
    let builtins = PyModule::import(mv.py(), "builtins")?;
    let memoryview_type = builtins.getattr("memoryview")?;
    if !mv.is_instance(memoryview_type)? {
        return Err(PyTypeError::new_err("Expected memoryview"));
    }
    Ok(())
}

/// Check if memoryview is writable
pub(crate) fn is_memoryview_writable(mv: &PyAny) -> PyResult<bool> {
    let readonly: bool = mv.getattr("readonly")?.extract()?;
    Ok(!readonly)
}

/// Validate Arrow buffer/array
pub(crate) fn validate_arrow_buffer(arrow_obj: &PyAny) -> PyResult<()> {
    // Check if it's an Arrow object (should already be detected, but double-check)
    if let Ok(pyarrow_module) = PyModule::import(arrow_obj.py(), "pyarrow") {
        let is_buffer = pyarrow_module
            .getattr("Buffer")
            .and_then(|t| arrow_obj.is_instance(t))
            .unwrap_or(false);
        let is_array = pyarrow_module
            .getattr("Array")
            .and_then(|t| arrow_obj.is_instance(t))
            .unwrap_or(false);
        let is_chunked = pyarrow_module
            .getattr("ChunkedArray")
            .and_then(|t| arrow_obj.is_instance(t))
            .unwrap_or(false);

        if !is_buffer && !is_array && !is_chunked {
            return Err(PyTypeError::new_err(
                "Expected Arrow Buffer, Array, or ChunkedArray",
            ));
        }

        // For ChunkedArray, check if it can be converted to a single chunk
        if is_chunked {
            // ChunkedArray needs special handling - may need to combine chunks
            // For now, we'll allow it but note it may need conversion
        }
    } else {
        return Err(PyTypeError::new_err("pyarrow module not available"));
    }

    Ok(())
}

/// Validate input for operation
pub(crate) fn validate_for_operation(
    obj: &PyAny,
    input_type: InputType,
    in_place: bool,
) -> PyResult<()> {
    match input_type {
        InputType::ArrayArray => validate_array_array(obj)?,
        InputType::NumPyArray => validate_numpy_array(obj)?,
        InputType::MemoryView => {
            validate_memoryview(obj)?;
            if in_place && !is_memoryview_writable(obj)? {
                return Err(PyValueError::new_err(
                    "memoryview is read-only; in-place operations require writable memoryview",
                ));
            }
        }
        InputType::ArrowBuffer => {
            validate_arrow_buffer(obj)?;
            if in_place {
                return Err(PyValueError::new_err(
                    "Arrow buffers/arrays are immutable; in-place operations not supported",
                ));
            }
        }
    }
    Ok(())
}
