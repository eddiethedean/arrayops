#![allow(non_local_definitions)] // PyO3 macros generate non-local impl blocks

use pyo3::buffer::{Element, PyBuffer};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;

mod allocator;
mod lazy;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

// SIMD optimizations: Use compiler auto-vectorization with chunked processing
// For explicit SIMD, use std::arch intrinsics (stable) or std::simd (nightly)

/// Supported array.array typecodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypeCode {
    // Signed integers
    Int8,  // 'b'
    Int16, // 'h'
    Int32, // 'i'
    Int64, // 'l'
    // Unsigned integers
    UInt8,  // 'B'
    UInt16, // 'H'
    UInt32, // 'I'
    UInt64, // 'L'
    // Floats
    Float32, // 'f'
    Float64, // 'd'
}

impl TypeCode {
    /// Parse typecode from Python array.array typecode
    fn from_char(typecode: char) -> PyResult<Self> {
        match typecode {
            'b' => Ok(TypeCode::Int8),
            'h' => Ok(TypeCode::Int16),
            'i' => Ok(TypeCode::Int32),
            'l' => Ok(TypeCode::Int64),
            'B' => Ok(TypeCode::UInt8),
            'H' => Ok(TypeCode::UInt16),
            'I' => Ok(TypeCode::UInt32),
            'L' => Ok(TypeCode::UInt64),
            'f' => Ok(TypeCode::Float32),
            'd' => Ok(TypeCode::Float64),
            _ => Err(PyTypeError::new_err(format!(
                "Unsupported typecode: '{}'. Supported: b, B, h, H, i, I, l, L, f, d",
                typecode
            ))),
        }
    }

    /// Get typecode as char
    fn as_char(&self) -> char {
        match self {
            TypeCode::Int8 => 'b',
            TypeCode::Int16 => 'h',
            TypeCode::Int32 => 'i',
            TypeCode::Int64 => 'l',
            TypeCode::UInt8 => 'B',
            TypeCode::UInt16 => 'H',
            TypeCode::UInt32 => 'I',
            TypeCode::UInt64 => 'L',
            TypeCode::Float32 => 'f',
            TypeCode::Float64 => 'd',
        }
    }
}

/// Get typecode from Python array.array object
fn get_typecode(array: &PyAny) -> PyResult<TypeCode> {
    let typecode_str = array.getattr("typecode")?.str()?.to_string_lossy();
    if typecode_str.len() != 1 {
        return Err(PyTypeError::new_err("Invalid typecode"));
    }
    TypeCode::from_char(typecode_str.chars().next().unwrap())
}

/// Get typecode from numpy.ndarray dtype
fn get_numpy_typecode(arr: &PyAny) -> PyResult<TypeCode> {
    let dtype = arr.getattr("dtype")?;
    let dtype_str = dtype.getattr("char")?.str()?.to_string_lossy();
    let dtype_char = dtype_str.chars().next().ok_or_else(|| {
        PyTypeError::new_err("Could not extract dtype character from numpy.ndarray")
    })?;

    // Map numpy dtype characters to TypeCode
    // numpy uses: 'b'=int8, 'h'=int16, 'i'=int32, 'l'=int64
    //             'B'=uint8, 'H'=uint16, 'I'=uint32, 'L'=uint64
    //             'f'=float32, 'd'=float64
    match dtype_char {
        'b' => Ok(TypeCode::Int8),
        'h' => Ok(TypeCode::Int16),
        'i' => Ok(TypeCode::Int32),
        'l' => Ok(TypeCode::Int64),
        'B' => Ok(TypeCode::UInt8),
        'H' => Ok(TypeCode::UInt16),
        'I' => Ok(TypeCode::UInt32),
        'L' => Ok(TypeCode::UInt64),
        'f' => Ok(TypeCode::Float32),
        'd' => Ok(TypeCode::Float64),
        _ => Err(PyTypeError::new_err(format!(
            "Unsupported numpy dtype: '{}'. Supported: b, B, h, H, i, I, l, L, f, d",
            dtype_char
        ))),
    }
}

/// Get typecode from memoryview format string
fn get_memoryview_typecode(mv: &PyAny) -> PyResult<TypeCode> {
    let format_str = mv.getattr("format")?.str()?.to_string_lossy();

    // Parse format string (handle endianness: '<i4', '>f8', 'i', 'I', etc.)
    // Remove endianness prefix if present (<, >, =, !)
    let cleaned_format = format_str.trim_start_matches(['<', '>', '=', '!']);

    // Extract the base type character (first character after endianness)
    let base_char = cleaned_format
        .chars()
        .next()
        .ok_or_else(|| PyTypeError::new_err("Could not parse memoryview format string"))?;

    // Map memoryview format to TypeCode
    // Standard formats: 'b'=signed char, 'B'=unsigned char, 'h'=short, 'H'=unsigned short
    //                   'i'=int, 'I'=unsigned int, 'l'=long, 'L'=unsigned long
    //                   'f'=float, 'd'=double
    match base_char {
        'b' => Ok(TypeCode::Int8),
        'B' => Ok(TypeCode::UInt8),
        'h' => Ok(TypeCode::Int16),
        'H' => Ok(TypeCode::UInt16),
        'i' => Ok(TypeCode::Int32),
        'I' => Ok(TypeCode::UInt32),
        'l' => Ok(TypeCode::Int64),
        'L' => Ok(TypeCode::UInt64),
        'f' => Ok(TypeCode::Float32),
        'd' => Ok(TypeCode::Float64),
        _ => Err(PyTypeError::new_err(format!(
            "Unsupported memoryview format: '{}'. Supported: b, B, h, H, i, I, l, L, f, d",
            base_char
        ))),
    }
}

/// Get typecode from Arrow buffer/array
fn get_arrow_typecode(arrow_obj: &PyAny) -> PyResult<TypeCode> {
    // Arrow arrays have a 'type' attribute with type information
    // For buffers, we need to check the underlying data type
    // Try to get dtype from Arrow array
    if let Ok(arrow_type) = arrow_obj.getattr("type") {
        if let Ok(type_str) = arrow_type.str() {
            let type_str = type_str.to_string_lossy();
            // Parse Arrow type string (e.g., "Int8", "Int32", "Float64")
            // Map to TypeCode
            if type_str.contains("Int8") || type_str.contains("int8") {
                return Ok(TypeCode::Int8);
            } else if type_str.contains("UInt8") || type_str.contains("uint8") {
                return Ok(TypeCode::UInt8);
            } else if type_str.contains("Int16") || type_str.contains("int16") {
                return Ok(TypeCode::Int16);
            } else if type_str.contains("UInt16") || type_str.contains("uint16") {
                return Ok(TypeCode::UInt16);
            } else if type_str.contains("Int32") || type_str.contains("int32") {
                return Ok(TypeCode::Int32);
            } else if type_str.contains("UInt32") || type_str.contains("uint32") {
                return Ok(TypeCode::UInt32);
            } else if type_str.contains("Int64") || type_str.contains("int64") {
                return Ok(TypeCode::Int64);
            } else if type_str.contains("UInt64") || type_str.contains("uint64") {
                return Ok(TypeCode::UInt64);
            } else if type_str.contains("Float32") || type_str.contains("float32") {
                return Ok(TypeCode::Float32);
            } else if type_str.contains("Float64")
                || type_str.contains("float64")
                || type_str.contains("Double")
            {
                return Ok(TypeCode::Float64);
            }
        }

        // Try to get id (Arrow type ID) as fallback
        if let Ok(id) = arrow_type.getattr("id") {
            if let Ok(id_val) = id.extract::<i32>() {
                // Arrow type IDs: Int8=2, Int16=3, Int32=4, Int64=5, UInt8=6, UInt16=7, UInt32=8, UInt64=9, Float32=10, Float64=11
                match id_val {
                    2 => return Ok(TypeCode::Int8),
                    3 => return Ok(TypeCode::Int16),
                    4 => return Ok(TypeCode::Int32),
                    5 => return Ok(TypeCode::Int64),
                    6 => return Ok(TypeCode::UInt8),
                    7 => return Ok(TypeCode::UInt16),
                    8 => return Ok(TypeCode::UInt32),
                    9 => return Ok(TypeCode::UInt64),
                    10 => return Ok(TypeCode::Float32),
                    11 => return Ok(TypeCode::Float64),
                    _ => {}
                }
            }
        }
    }

    // Note: Arrow buffers support buffer protocol, but type detection via buffer protocol
    // is complex. The type should be available via the Arrow type system above.

    Err(PyTypeError::new_err(
        "Could not determine Arrow buffer/array type. Supported types: Int8, UInt8, Int16, UInt16, Int32, UInt32, Int64, UInt64, Float32, Float64"
    ))
}

/// Get typecode from unified input (array.array, numpy.ndarray, memoryview, or Arrow buffer/array)
fn get_typecode_unified(obj: &PyAny, input_type: InputType) -> PyResult<TypeCode> {
    match input_type {
        InputType::ArrayArray => get_typecode(obj),
        InputType::NumPyArray => get_numpy_typecode(obj),
        InputType::MemoryView => get_memoryview_typecode(obj),
        InputType::ArrowBuffer => get_arrow_typecode(obj),
    }
}

/// Input type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputType {
    ArrayArray,
    NumPyArray,
    MemoryView,
    ArrowBuffer, // Apache Arrow buffer/array
}

/// Detect the input type (array.array, numpy.ndarray, memoryview, or Arrow buffer/array)
fn detect_input_type(obj: &PyAny) -> PyResult<InputType> {
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

/// Validate that the input is an array.array
fn validate_array_array(array: &PyAny) -> PyResult<()> {
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
fn validate_numpy_array(arr: &PyAny) -> PyResult<()> {
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
fn validate_memoryview(mv: &PyAny) -> PyResult<()> {
    // Check if it's a memoryview (should already be detected, but double-check)
    let builtins = PyModule::import(mv.py(), "builtins")?;
    let memoryview_type = builtins.getattr("memoryview")?;
    if !mv.is_instance(memoryview_type)? {
        return Err(PyTypeError::new_err("Expected memoryview"));
    }
    Ok(())
}

/// Check if memoryview is writable
fn is_memoryview_writable(mv: &PyAny) -> PyResult<bool> {
    let readonly: bool = mv.getattr("readonly")?.extract()?;
    Ok(!readonly)
}

/// Validate Arrow buffer/array
fn validate_arrow_buffer(arrow_obj: &PyAny) -> PyResult<()> {
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
fn validate_for_operation(obj: &PyAny, input_type: InputType, in_place: bool) -> PyResult<()> {
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

/// Get the length of an array.array
fn get_array_len(array: &PyAny) -> PyResult<usize> {
    let len = array.len()?;
    Ok(len)
}

/// Get itemsize of an array.array, numpy.ndarray, or memoryview
fn get_itemsize(array: &PyAny) -> PyResult<usize> {
    let itemsize: usize = array.getattr("itemsize")?.extract()?;
    Ok(itemsize)
}

// ============================================================================
// Macro System for Typecode Dispatch (Phase 1)
// ============================================================================

/// Macro to generate a complete match statement for typecode dispatch with immutable buffers
/// 
/// This macro generates a full match expression that handles all typecodes,
/// including special handling for Int64/UInt64 itemsize checking.
/// The body block will be repeated for each typecode with the appropriate buffer type.
/// 
/// Usage:
/// ```rust,ignore
/// dispatch_by_typecode!(typecode, array, |buffer| {
///     let result = operation_impl(py, &buffer)?;
///     Ok(result.to_object(py))
/// })
/// ```
macro_rules! dispatch_by_typecode {
    ($typecode:expr, $array:expr, |$buffer:ident| $body:block) => {
        match $typecode {
            TypeCode::Int8 => {
                let $buffer = PyBuffer::<i8>::get($array)?;
                $body
            }
            TypeCode::Int16 => {
                let $buffer = PyBuffer::<i16>::get($array)?;
                $body
            }
            TypeCode::Int32 => {
                let $buffer = PyBuffer::<i32>::get($array)?;
                $body
            }
            TypeCode::Int64 => {
                let itemsize = get_itemsize($array)?;
                if itemsize == 4 {
                    let $buffer = PyBuffer::<i32>::get($array)?;
                    $body
                } else {
                    let $buffer = PyBuffer::<i64>::get($array)?;
                    $body
                }
            }
            TypeCode::UInt8 => {
                let $buffer = PyBuffer::<u8>::get($array)?;
                $body
            }
            TypeCode::UInt16 => {
                let $buffer = PyBuffer::<u16>::get($array)?;
                $body
            }
            TypeCode::UInt32 => {
                let $buffer = PyBuffer::<u32>::get($array)?;
                $body
            }
            TypeCode::UInt64 => {
                let itemsize = get_itemsize($array)?;
                if itemsize == 4 {
                    let $buffer = PyBuffer::<u32>::get($array)?;
                    $body
                } else {
                    let $buffer = PyBuffer::<u64>::get($array)?;
                    $body
                }
            }
            TypeCode::Float32 => {
                let $buffer = PyBuffer::<f32>::get($array)?;
                $body
            }
            TypeCode::Float64 => {
                let $buffer = PyBuffer::<f64>::get($array)?;
                $body
            }
        }
    };
}

/// Macro to generate a complete match statement for typecode dispatch with mutable buffers
macro_rules! dispatch_by_typecode_mut {
    ($typecode:expr, $array:expr, |$buffer:ident| $body:block) => {
        match $typecode {
            TypeCode::Int8 => {
                let mut $buffer = PyBuffer::<i8>::get($array)?;
                $body
            }
            TypeCode::Int16 => {
                let mut $buffer = PyBuffer::<i16>::get($array)?;
                $body
            }
            TypeCode::Int32 => {
                let mut $buffer = PyBuffer::<i32>::get($array)?;
                $body
            }
            TypeCode::Int64 => {
                let itemsize = get_itemsize($array)?;
                if itemsize == 4 {
                    let mut $buffer = PyBuffer::<i32>::get($array)?;
                    $body
                } else {
                    let mut $buffer = PyBuffer::<i64>::get($array)?;
                    $body
                }
            }
            TypeCode::UInt8 => {
                let mut $buffer = PyBuffer::<u8>::get($array)?;
                $body
            }
            TypeCode::UInt16 => {
                let mut $buffer = PyBuffer::<u16>::get($array)?;
                $body
            }
            TypeCode::UInt32 => {
                let mut $buffer = PyBuffer::<u32>::get($array)?;
                $body
            }
            TypeCode::UInt64 => {
                let itemsize = get_itemsize($array)?;
                if itemsize == 4 {
                    let mut $buffer = PyBuffer::<u32>::get($array)?;
                    $body
                } else {
                    let mut $buffer = PyBuffer::<u64>::get($array)?;
                    $body
                }
            }
            TypeCode::Float32 => {
                let mut $buffer = PyBuffer::<f32>::get($array)?;
                $body
            }
            TypeCode::Float64 => {
                let mut $buffer = PyBuffer::<f64>::get($array)?;
                $body
            }
        }
    };
}

/// Create an empty result array based on input type
fn create_empty_result_array(
    py: Python,
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
            let arr = numpy_array.call1((PyList::empty(py),))?;
            Ok(arr.call_method1("astype", (dtype,))?.to_object(py))
        }
        InputType::ArrayArray | InputType::MemoryView => {
            // Create array.array
            let array_module = PyModule::import(py, "array")?;
            let array_type = array_module.getattr("array")?;
            let typecode_char = typecode.as_char();
            Ok(array_type
                .call1((typecode_char, PyList::empty(py)))?
                .to_object(py))
        }
        InputType::ArrowBuffer => {
            // Create empty Arrow array
            let pyarrow_module = PyModule::import(py, "pyarrow")?;
            let array_func = pyarrow_module.getattr("array")?;
            // Create empty list and convert to Arrow array
            let empty_list = PyList::empty(py);
            Ok(array_func.call1((empty_list,))?.to_object(py))
        }
    }
}

/// Create result array from list based on input type
fn create_result_array_from_list(
    py: Python,
    typecode: TypeCode,
    input_type: InputType,
    values: &PyList,
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
            Ok(arr.call_method1("astype", (dtype,))?.to_object(py))
        }
        InputType::ArrayArray | InputType::MemoryView => {
            // Create array.array
            let array_module = PyModule::import(py, "array")?;
            let array_type = array_module.getattr("array")?;
            let typecode_char = typecode.as_char();
            Ok(array_type.call1((typecode_char, values))?.to_object(py))
        }
        InputType::ArrowBuffer => {
            // Create Arrow array from list
            let pyarrow_module = PyModule::import(py, "pyarrow")?;
            let array_func = pyarrow_module.getattr("array")?;
            // Arrow arrays can be created from Python lists
            Ok(array_func.call1((values,))?.to_object(py))
        }
    }
}

/// Check if an operation can return a view instead of a copy
/// Note: Currently returns false - filter and map cannot return views because
/// filter changes array size and map transforms elements
#[allow(dead_code)]
fn can_return_view(_input_type: InputType, _operation: &str) -> bool {
    // Filter changes array size, map transforms elements - views not applicable
    // This function exists for future optimizations (e.g., detecting identity map operations)
    false
}

/// Create a memoryview slice from a buffer (helper function for zero-copy slicing)
/// Note: This is used by the slice() function. For filter/map operations, views are
/// not applicable because they change data/size
fn create_slice_view_helper(
    py: Python,
    array: &PyAny,
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
    Ok(slice_obj.to_object(py))
}

/// Slice operation - returns a zero-copy memoryview of a portion of the array
#[pyfunction]
fn slice(
    py: Python,
    array: &PyAny,
    start: Option<usize>,
    end: Option<usize>,
) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let len = get_array_len(array)?;

    let start_idx = start.unwrap_or(0);
    let end_idx = end.unwrap_or(len);

    // Bounds checking
    if start_idx > len || end_idx > len || start_idx > end_idx {
        return Err(PyValueError::new_err(format!(
            "Invalid slice indices: start={}, end={}, length={}",
            start_idx, end_idx, len
        )));
    }

    // Use the helper function to create the slice view
    create_slice_view_helper(py, array, start_idx, end_idx)
}

/// Create result array from Rust Vec (much faster than building PyList incrementally)
fn create_result_array_from_vec<T>(
    py: Python,
    typecode: TypeCode,
    input_type: InputType,
    values: Vec<T>,
) -> PyResult<PyObject>
where
    T: Copy + pyo3::ToPyObject,
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
            let py_list = PyList::new(py, values.iter().map(|v| v.to_object(py)));
            let arr = numpy_array.call1((py_list,))?;
            Ok(arr.call_method1("astype", (dtype,))?.to_object(py))
        }
        InputType::ArrayArray | InputType::MemoryView => {
            // Create array.array from Vec
            let array_module = PyModule::import(py, "array")?;
            let array_type = array_module.getattr("array")?;
            let typecode_char = typecode.as_char();
            // Convert Vec to PyList once (much faster than incremental append)
            let py_list = PyList::new(py, values.iter().map(|v| v.to_object(py)));
            Ok(array_type.call1((typecode_char, py_list))?.to_object(py))
        }
        InputType::ArrowBuffer => {
            // Create Arrow array from Vec
            let pyarrow_module = PyModule::import(py, "pyarrow")?;
            let array_func = pyarrow_module.getattr("array")?;
            // Convert Vec to PyList once
            let py_list = PyList::new(py, values.iter().map(|v| v.to_object(py)));
            // Arrow arrays can be created from Python lists
            Ok(array_func.call1((py_list,))?.to_object(py))
        }
    }
}

// Parallel execution thresholds (lowered for better performance)
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
const PARALLEL_THRESHOLD_SUM: usize = 1_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
const PARALLEL_THRESHOLD_SCALE: usize = 1_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
const PARALLEL_THRESHOLD_MEAN: usize = 1_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
const PARALLEL_THRESHOLD_MINMAX: usize = 50_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
const PARALLEL_THRESHOLD_ADD: usize = 1_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
const PARALLEL_THRESHOLD_MULTIPLY: usize = 1_000;
#[allow(dead_code)] // Reserved for future parallel implementation
const PARALLEL_THRESHOLD_CLIP: usize = 1_000;
#[allow(dead_code)] // Reserved for future parallel implementation
const PARALLEL_THRESHOLD_NORMALIZE: usize = 2_000;
// Note: MAP, FILTER, and REDUCE thresholds reserved for future use
// (parallel execution for these operations is limited by Python's GIL)
#[allow(dead_code)]
const PARALLEL_THRESHOLD_MAP: usize = 10_000;
#[allow(dead_code)]
const PARALLEL_THRESHOLD_FILTER: usize = 10_000;
#[allow(dead_code)]
const PARALLEL_THRESHOLD_REDUCE: usize = 10_000;

/// Extract buffer data to Vec for parallel processing
#[cfg(feature = "parallel")]
fn extract_buffer_to_vec<T>(py: Python, buffer: &PyBuffer<T>) -> PyResult<Vec<T>>
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
fn should_parallelize(len: usize, threshold: usize) -> bool {
    len >= threshold
}

// Cache blocking constants
// Typical L1 cache is 32KB, L2 is 256KB-1MB
// Process arrays in cache-friendly chunks to improve memory access patterns
const CACHE_BLOCK_SIZE: usize = 8192; // ~8KB blocks (works well for most cache sizes)

// SIMD thresholds - minimum array size to use SIMD
// Reserved for future SIMD implementation
#[cfg(feature = "simd")]
#[allow(dead_code)]
const SIMD_THRESHOLD: usize = 32;

// SIMD optimization infrastructure
// Using compiler auto-vectorization with cache-friendly code patterns
// For explicit SIMD, use std::arch intrinsics (stable) or std::simd (nightly)

// Generic sum implementation with cache-friendly processing
fn sum_impl<T>(py: Python, buffer: &PyBuffer<T>, len: usize) -> PyResult<T>
where
    T: Element + Copy + Default + std::ops::Add<Output = T> + pyo3::ToPyObject + Send + Sync,
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
fn min_impl<T>(py: Python, buffer: &PyBuffer<T>) -> PyResult<T>
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
fn max_impl<T>(py: Python, buffer: &PyBuffer<T>) -> PyResult<T>
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
fn sum(py: Python, array: &PyAny) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        match typecode {
            TypeCode::Int8 => return Ok(0i8.to_object(py)),
            TypeCode::Int16 => return Ok(0i16.to_object(py)),
            TypeCode::Int32 => return Ok(0i32.to_object(py)),
            TypeCode::Int64 => {
                let itemsize = get_itemsize(array)?;
                if itemsize == 4 {
                    return Ok(0i32.to_object(py));
                } else {
                    return Ok(0i64.to_object(py));
                }
            }
            TypeCode::UInt8 => return Ok(0u8.to_object(py)),
            TypeCode::UInt16 => return Ok(0u16.to_object(py)),
            TypeCode::UInt32 => return Ok(0u32.to_object(py)),
            TypeCode::UInt64 => {
                let itemsize = get_itemsize(array)?;
                if itemsize == 4 {
                    return Ok(0u32.to_object(py));
                } else {
                    return Ok(0u64.to_object(py));
                }
            }
            TypeCode::Float32 => return Ok(0.0f32.to_object(py)),
            TypeCode::Float64 => return Ok(0.0f64.to_object(py)),
        }
    }

    let len = get_array_len(array)?;
    dispatch_by_typecode!(typecode, array, |buffer| {
        let result = sum_impl(py, &buffer, len)?;
        Ok(result.to_object(py))
    })
}

/// Helper function to extract an element at a specific index from a buffer
fn extract_element_at_index<T>(
    py: Python,
    source_ref: &PyAny,
    index: usize,
) -> PyResult<PyObject>
where
    T: Element + Copy + pyo3::ToPyObject,
{
    let buffer = PyBuffer::<T>::get(source_ref)?;
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
    Ok(slice[index].get().to_object(py))
}

/// Scale operation (in-place) for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn scale(py: Python, array: &PyAny, factor: f64) -> PyResult<()> {
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

// Generic map implementation (returns new array)
fn map_impl<T>(
    py: Python,
    buffer: &PyBuffer<T>,
    callable: &PyAny,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + pyo3::ToPyObject,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    let result_list = PyList::empty(py);

    for cell in slice.iter() {
        let value = cell.get();
        let value_obj = value.to_object(py);
        let result = callable.call1((value_obj,))?;
        result_list.append(result)?;
    }

    create_result_array_from_list(py, typecode, input_type, result_list)
}

/// Map operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn map(py: Python, array: &PyAny, r#fn: PyObject) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;
    let callable = r#fn.as_ref(py);

    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        return create_empty_result_array(py, typecode, input_type);
    }

    dispatch_by_typecode!(typecode, array, |buffer| {
        map_impl(py, &buffer, callable, typecode, input_type)
    })
}

// Generic map_inplace implementation
fn map_inplace_impl<T>(py: Python, buffer: &mut PyBuffer<T>, callable: &PyAny) -> PyResult<()>
where
    T: Element + Copy + pyo3::ToPyObject + for<'a> pyo3::FromPyObject<'a>,
{
    let slice = buffer
        .as_mut_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;

    for item in slice.iter() {
        let value = item.get();
        let value_obj = value.to_object(py);
        let result = callable.call1((value_obj,))?;
        let result_value: T = result.extract()?;
        item.set(result_value);
    }

    Ok(())
}

/// Map in-place operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn map_inplace(py: Python, array: &PyAny, r#fn: PyObject) -> PyResult<()> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, true)?;
    let typecode = get_typecode_unified(array, input_type)?;
    let callable = r#fn.as_ref(py);

    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        return Ok(());
    }

    dispatch_by_typecode_mut!(typecode, array, |buffer| {
        map_inplace_impl(py, &mut buffer, callable)
    })
}

// Generic filter implementation
fn filter_impl<T>(
    py: Python,
    buffer: &PyBuffer<T>,
    predicate: &PyAny,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + pyo3::ToPyObject,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    let result_list = PyList::empty(py);

    for cell in slice.iter() {
        let value = cell.get();
        let value_obj = value.to_object(py);
        let result = predicate.call1((value_obj.clone(),))?;
        let should_include: bool = result.extract()?;
        if should_include {
            result_list.append(value_obj)?;
        }
    }

    create_result_array_from_list(py, typecode, input_type, result_list)
}

/// Filter operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn filter(py: Python, array: &PyAny, predicate: PyObject) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;
    let callable = predicate.as_ref(py);

    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        return create_empty_result_array(py, typecode, input_type);
    }

    dispatch_by_typecode!(typecode, array, |buffer| {
        filter_impl(py, &buffer, callable, typecode, input_type)
    })
}

// Generic reduce implementation
fn reduce_impl<T>(
    py: Python,
    buffer: &PyBuffer<T>,
    r#fn: &PyAny,
    initial: Option<PyObject>,
) -> PyResult<PyObject>
where
    T: Element + Copy + pyo3::ToPyObject,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    // Note: Empty check is now done in reduce() before getting the buffer,
    // but keep this as a safety check in case the buffer is somehow empty
    if slice.is_empty() {
        return match initial {
            Some(init) => Ok(init),
            None => Err(PyValueError::new_err(
                "reduce() of empty array with no initial value",
            )),
        };
    }

    let (mut acc, start_idx) = match initial {
        Some(init) => (init, 0),
        None => {
            let first = slice.iter().next().unwrap().get();
            (first.to_object(py), 1)
        }
    };
    for cell in slice.iter().skip(start_idx) {
        let value = cell.get();
        let value_obj = value.to_object(py);
        let result = r#fn.call1((acc, value_obj))?;
        acc = result.to_object(py);
    }

    Ok(acc)
}

/// Reduce operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn reduce(
    py: Python,
    array: &PyAny,
    r#fn: PyObject,
    initial: Option<PyObject>,
) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;
    let callable = r#fn.as_ref(py);

    // Handle empty arrays early to avoid buffer alignment issues on macOS
    let array_len = get_array_len(array)?;
    if array_len == 0 {
        return match initial {
            Some(init) => Ok(init),
            None => Err(PyValueError::new_err(
                "reduce() of empty array with no initial value",
            )),
        };
    }

    dispatch_by_typecode!(typecode, array, |buffer| {
        reduce_impl(py, &buffer, callable, initial)
    })
}

/// Mean operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn mean(py: Python, array: &PyAny) -> PyResult<f64> {
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
fn min(py: Python, array: &PyAny) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays - raise ValueError
    let len = get_array_len(array)?;
    if len == 0 {
        return Err(PyValueError::new_err("min() of empty array"));
    }

    dispatch_by_typecode!(typecode, array, |buffer| {
        let result = min_impl(py, &buffer)?;
        Ok(result.to_object(py))
    })
}

/// Max operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn max(py: Python, array: &PyAny) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays - raise ValueError
    let len = get_array_len(array)?;
    if len == 0 {
        return Err(PyValueError::new_err("max() of empty array"));
    }

    dispatch_by_typecode!(typecode, array, |buffer| {
        let result = max_impl(py, &buffer)?;
        Ok(result.to_object(py))
    })
}

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
fn var(py: Python, array: &PyAny) -> PyResult<f64> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays - raise ValueError
    let len = get_array_len(array)?;
    if len == 0 {
        return Err(PyValueError::new_err("var() of empty array"));
    }

    // Calculate mean first
    let mean_val = mean(py, array)?;

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
fn std_dev(py: Python, array: &PyAny) -> PyResult<f64> {
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
    let mid = data.len() / 2;
    Ok(data[mid])
}

/// Median operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn median(py: Python, array: &PyAny) -> PyResult<PyObject> {
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
            Ok(result.to_object(py))
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<i32>::get(array)?;
                let result = median_impl_int(py, &buffer)?;
                Ok(result.to_object(py))
            } else {
                let buffer = PyBuffer::<i64>::get(array)?;
                let result = median_impl_int(py, &buffer)?;
                Ok(result.to_object(py))
            }
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            let result = median_impl_int(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<u32>::get(array)?;
                let result = median_impl_int(py, &buffer)?;
                Ok(result.to_object(py))
            } else {
                let buffer = PyBuffer::<u64>::get(array)?;
                let result = median_impl_int(py, &buffer)?;
                Ok(result.to_object(py))
            }
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            let result = median_impl_float(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            let result = median_impl_float(py, &buffer)?;
            Ok(result.to_object(py))
        }
    }
}

// Generic add implementation
#[allow(unused_variables)]
fn add_impl<T>(
    py: Python,
    buffer1: &PyBuffer<T>,
    buffer2: &PyBuffer<T>,
    len: usize,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + std::ops::Add<Output = T> + Send + Sync + pyo3::ToPyObject,
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

// Generic multiply implementation
#[allow(unused_variables)]
fn multiply_impl<T>(
    py: Python,
    buffer1: &PyBuffer<T>,
    buffer2: &PyBuffer<T>,
    len: usize,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + std::ops::Mul<Output = T> + Send + Sync + pyo3::ToPyObject,
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

/// Add operation (element-wise) for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn add(py: Python, arr1: &PyAny, arr2: &PyAny) -> PyResult<PyObject> {
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

/// Multiply operation (element-wise) for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn multiply(py: Python, arr1: &PyAny, arr2: &PyAny) -> PyResult<PyObject> {
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
fn clip(py: Python, array: &PyAny, min_val: f64, max_val: f64) -> PyResult<()> {
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
fn normalize(py: Python, array: &PyAny) -> PyResult<()> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, true)?;
    let typecode = get_typecode_unified(array, input_type)?;

    // Handle empty arrays
    let len = get_array_len(array)?;
    if len == 0 {
        return Ok(());
    }

    // Get min and max using our min/max functions
    let min_val_py = min(py, array)?;
    let max_val_py = max(py, array)?;

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

// Generic reverse implementation (in-place)
fn reverse_impl<T>(py: Python, buffer: &mut PyBuffer<T>) -> PyResult<()>
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
fn reverse(py: Python, array: &PyAny) -> PyResult<()> {
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

// Generic sort implementation (in-place) for integer types (Ord)
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

// Generic sort implementation (in-place) for float types (PartialOrd)
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
fn sort(py: Python, array: &PyAny) -> PyResult<()> {
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

// Generic unique implementation
fn unique_impl_int<T>(
    py: Python,
    buffer: &PyBuffer<T>,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + Ord + Send + Sync + pyo3::ToPyObject,
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
        result_list.append(val.to_object(py))?;
    }

    create_result_array_from_list(py, typecode, input_type, result_list)
}

// Generic unique implementation for float types
fn unique_impl_float<T>(
    py: Python,
    buffer: &PyBuffer<T>,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + PartialOrd + Send + Sync + pyo3::ToPyObject,
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
        result_list.append(val.to_object(py))?;
    }

    create_result_array_from_list(py, typecode, input_type, result_list)
}

/// Unique operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
fn unique(py: Python, array: &PyAny) -> PyResult<PyObject> {
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

/// ArrayIterator - Efficient Rust-optimized iterator for array types
#[pyclass]
pub struct ArrayIterator {
    source: PyObject,
    typecode: TypeCode,
    current_index: usize,
    length: usize,
}

#[pymethods]
#[allow(non_local_definitions)]
impl ArrayIterator {
    /// Create a new ArrayIterator from an array-like object
    #[new]
    pub fn new(py: Python, source: PyObject) -> PyResult<Self> {
        let source_ref = source.as_ref(py);
        let input_type = detect_input_type(source_ref)?;
        validate_for_operation(source_ref, input_type, false)?;
        let typecode = get_typecode_unified(source_ref, input_type)?;
        let length = get_array_len(source_ref)?;

        Ok(ArrayIterator {
            source,
            typecode,
            current_index: 0,
            length,
        })
    }

    /// Return self as the iterator (required by iterator protocol)
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// Return the next element or None (raises StopIteration in Python)
    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<PyObject>> {
        if slf.current_index >= slf.length {
            return Ok(None); // None signals StopIteration in PyO3
        }

        let py = slf.py();
        let index = slf.current_index;
        let typecode = slf.typecode;
        let source = slf.source.clone_ref(py);
        slf.current_index += 1;

        // Get element at index based on typecode
        let source_ref = source.as_ref(py);
        let result = match typecode {
            TypeCode::Int8 => extract_element_at_index::<i8>(py, source_ref, index)?,
            TypeCode::Int16 => extract_element_at_index::<i16>(py, source_ref, index)?,
            TypeCode::Int32 => extract_element_at_index::<i32>(py, source_ref, index)?,
            TypeCode::Int64 => {
                let itemsize = get_itemsize(source_ref)?;
                if itemsize == 4 {
                    extract_element_at_index::<i32>(py, source_ref, index)?
                } else {
                    extract_element_at_index::<i64>(py, source_ref, index)?
                }
            }
            TypeCode::UInt8 => extract_element_at_index::<u8>(py, source_ref, index)?,
            TypeCode::UInt16 => extract_element_at_index::<u16>(py, source_ref, index)?,
            TypeCode::UInt32 => extract_element_at_index::<u32>(py, source_ref, index)?,
            TypeCode::UInt64 => {
                let itemsize = get_itemsize(source_ref)?;
                if itemsize == 4 {
                    extract_element_at_index::<u32>(py, source_ref, index)?
                } else {
                    extract_element_at_index::<u64>(py, source_ref, index)?
                }
            }
            TypeCode::Float32 => extract_element_at_index::<f32>(py, source_ref, index)?,
            TypeCode::Float64 => extract_element_at_index::<f64>(py, source_ref, index)?,
        };

        Ok(Some(result))
    }
}

/// Helper function to create an ArrayIterator
#[pyfunction]
fn array_iterator(py: Python, array: PyObject) -> PyResult<Py<ArrayIterator>> {
    Py::new(py, ArrayIterator::new(py, array)?)
}

/// Helper function to create a LazyArray wrapper
#[pyfunction]
fn lazy_array(py: Python, array: PyObject) -> PyResult<Py<lazy::LazyArray>> {
    Py::new(py, lazy::LazyArray::new(array))
}

/// A Python module implemented in Rust.
#[pymodule]
fn _arrayops(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum, m)?)?;
    m.add_function(wrap_pyfunction!(scale, m)?)?;
    m.add_function(wrap_pyfunction!(map, m)?)?;
    m.add_function(wrap_pyfunction!(map_inplace, m)?)?;
    m.add_function(wrap_pyfunction!(filter, m)?)?;
    m.add_function(wrap_pyfunction!(reduce, m)?)?;
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    m.add_function(wrap_pyfunction!(min, m)?)?;
    m.add_function(wrap_pyfunction!(max, m)?)?;
    m.add_function(wrap_pyfunction!(var, m)?)?;
    m.add_function(wrap_pyfunction!(std_dev, m)?)?;
    m.add_function(wrap_pyfunction!(median, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(clip, m)?)?;
    m.add_function(wrap_pyfunction!(normalize, m)?)?;
    m.add_function(wrap_pyfunction!(reverse, m)?)?;
    m.add_function(wrap_pyfunction!(sort, m)?)?;
    m.add_function(wrap_pyfunction!(unique, m)?)?;
    m.add_function(wrap_pyfunction!(slice, m)?)?;
    m.add_class::<ArrayIterator>()?;
    m.add_function(wrap_pyfunction!(array_iterator, m)?)?;
    m.add_class::<lazy::LazyArray>()?;
    m.add_function(wrap_pyfunction!(lazy_array, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::{PyDict, PyList};

    #[test]
    fn test_typecode_parsing() {
        assert_eq!(TypeCode::from_char('i').unwrap(), TypeCode::Int32);
        assert_eq!(TypeCode::from_char('f').unwrap(), TypeCode::Float32);
        assert_eq!(TypeCode::from_char('d').unwrap(), TypeCode::Float64);
        assert_eq!(TypeCode::from_char('b').unwrap(), TypeCode::Int8);
        assert_eq!(TypeCode::from_char('B').unwrap(), TypeCode::UInt8);
        assert!(TypeCode::from_char('x').is_err());
    }

    #[test]
    fn test_typecode_roundtrip() {
        let codes = ['b', 'B', 'h', 'H', 'i', 'I', 'l', 'L', 'f', 'd'];
        for &code in &codes {
            let tc = TypeCode::from_char(code).unwrap();
            assert_eq!(tc.as_char(), code);
        }
    }

    #[test]
    fn test_sum_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[1, 2, 3, 4, 5])))
                .unwrap();
            let result: i32 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 15);
        });
    }

    #[test]
    fn test_sum_float64() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("d", PyList::new(py, &[1.5, 2.5, 3.5])))
                .unwrap();
            let result: f64 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 7.5);
        });
    }

    #[test]
    fn test_sum_empty() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let empty_list = PyList::empty(py);
            let arr = array_type.call1(("i", empty_list)).unwrap();
            let result: i32 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 0);
        });
    }

    #[test]
    fn test_scale_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[1, 2, 3, 4, 5])))
                .unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<i32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2, 4, 6, 8, 10]);
        });
    }

    #[test]
    fn test_scale_float64() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("d", PyList::new(py, &[1.0, 2.0, 3.0])))
                .unwrap();
            scale(py, arr, 2.5).unwrap();
            let buffer = PyBuffer::<f64>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<f64> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2.5, 5.0, 7.5]);
        });
    }

    #[test]
    fn test_invalid_type() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            // Try 'q' which may not be available on all platforms, or use a valid type
            // but test our validation by trying to pass an unsupported typecode directly
            let result = array_type.call1(("q", PyList::new(py, &[1, 2, 3])));
            if result.is_ok() {
                // Platform supports 'q', test our validation
                let arr = result.unwrap();
                let sum_result = sum(py, arr);
                assert!(sum_result.is_err());
                assert!(sum_result
                    .unwrap_err()
                    .to_string()
                    .contains("Unsupported typecode"));
            } else {
                // Platform doesn't support 'q', skip this test
                // This is acceptable - Python rejects it before our code sees it
            }
        });
    }

    #[test]
    fn test_not_array_array() {
        Python::with_gil(|py| {
            let list = PyList::new(py, &[1, 2, 3]);
            let result = sum(py, list);
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Expected array.array"));
        });
    }

    // Test all typecode parsing variants
    #[test]
    fn test_all_typecode_variants() {
        let test_cases = vec![
            ('b', TypeCode::Int8),
            ('h', TypeCode::Int16),
            ('i', TypeCode::Int32),
            ('l', TypeCode::Int64),
            ('B', TypeCode::UInt8),
            ('H', TypeCode::UInt16),
            ('I', TypeCode::UInt32),
            ('L', TypeCode::UInt64),
            ('f', TypeCode::Float32),
            ('d', TypeCode::Float64),
        ];

        for (code, expected) in test_cases {
            assert_eq!(TypeCode::from_char(code).unwrap(), expected);
            assert_eq!(expected.as_char(), code);
        }
    }

    // Test all typecode error cases
    #[test]
    fn test_typecode_errors() {
        let invalid_codes = vec!['x', 'X', 'c', 'C', 'u', 'U', 'q', 'Q', ' ', '1', 'a'];
        for code in invalid_codes {
            assert!(
                TypeCode::from_char(code).is_err(),
                "Code '{}' should fail",
                code
            );
        }
    }

    // Test sum for all numeric types
    #[test]
    fn test_sum_all_integer_types() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();

            // Test Int8
            let arr = array_type
                .call1(("b", PyList::new(py, &[-1i8, 0, 1])))
                .unwrap();
            let result: i8 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 0i8);

            // Test UInt8
            let arr = array_type
                .call1(("B", PyList::new(py, &[1u8, 2, 3])))
                .unwrap();
            let result: u8 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 6u8);

            // Test Int16
            let arr = array_type
                .call1(("h", PyList::new(py, &[-10i16, 0, 10])))
                .unwrap();
            let result: i16 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 0i16);

            // Test UInt16
            let arr = array_type
                .call1(("H", PyList::new(py, &[100u16, 200])))
                .unwrap();
            let result: u16 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 300u16);

            // Test Int32
            let arr = array_type
                .call1(("i", PyList::new(py, &[1i32, 2, 3])))
                .unwrap();
            let result: i32 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 6i32);

            // Test UInt32
            let arr = array_type
                .call1(("I", PyList::new(py, &[1u32, 2, 3])))
                .unwrap();
            let result: u32 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 6u32);

            // Test Int64
            let arr = array_type
                .call1(("l", PyList::new(py, &[1000i64, 2000])))
                .unwrap();
            let result: i64 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 3000i64);

            // Test UInt64
            let arr = array_type
                .call1(("L", PyList::new(py, &[1000u64, 2000])))
                .unwrap();
            let result: u64 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 3000u64);
        });
    }

    #[test]
    fn test_sum_all_float_types() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();

            // Test f32
            let arr = array_type
                .call1(("f", PyList::new(py, &[1.5f32, 2.5, 3.5])))
                .unwrap();
            let result: f32 = sum(py, arr).unwrap().extract(py).unwrap();
            assert!((result - 7.5).abs() < 0.001);

            // Test f64
            let arr = array_type
                .call1(("d", PyList::new(py, &[1.5f64, 2.5, 3.5])))
                .unwrap();
            let result: f64 = sum(py, arr).unwrap().extract(py).unwrap();
            assert!((result - 7.5).abs() < 0.001);
        });
    }

    // Test scale for all numeric types
    #[test]
    fn test_scale_all_integer_types() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();

            // Test Int8
            let arr = array_type
                .call1(("b", PyList::new(py, &[1i8, 2, 3])))
                .unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<i8>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i8> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2i8, 4, 6]);

            // Test UInt8
            let arr = array_type
                .call1(("B", PyList::new(py, &[1u8, 2, 3])))
                .unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<u8>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<u8> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2u8, 4, 6]);

            // Test Int16
            let arr = array_type
                .call1(("h", PyList::new(py, &[1i16, 2, 3])))
                .unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<i16>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i16> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2i16, 4, 6]);

            // Test UInt16
            let arr = array_type
                .call1(("H", PyList::new(py, &[1u16, 2, 3])))
                .unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<u16>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<u16> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2u16, 4, 6]);

            // Test Int32
            let arr = array_type
                .call1(("i", PyList::new(py, &[1i32, 2, 3])))
                .unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<i32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2i32, 4, 6]);

            // Test UInt32
            let arr = array_type
                .call1(("I", PyList::new(py, &[1u32, 2, 3])))
                .unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<u32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<u32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2u32, 4, 6]);

            // Test Int64
            let arr = array_type
                .call1(("l", PyList::new(py, &[1i64, 2, 3])))
                .unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<i64>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i64> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2i64, 4, 6]);

            // Test UInt64
            let arr = array_type
                .call1(("L", PyList::new(py, &[1u64, 2, 3])))
                .unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<u64>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<u64> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2u64, 4, 6]);
        });
    }

    #[test]
    fn test_scale_float_types() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();

            // Test f32
            let arr = array_type
                .call1(("f", PyList::new(py, &[1.0f32, 2.0, 3.0])))
                .unwrap();
            scale(py, arr, 2.5).unwrap();
            let buffer = PyBuffer::<f32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<f32> = slice.iter().map(|cell| cell.get()).collect();
            assert!((values[0] - 2.5).abs() < 0.001);
            assert!((values[1] - 5.0).abs() < 0.001);
            assert!((values[2] - 7.5).abs() < 0.001);

            // Test f64
            let arr = array_type
                .call1(("d", PyList::new(py, &[1.0f64, 2.0, 3.0])))
                .unwrap();
            scale(py, arr, 2.5).unwrap();
            let buffer = PyBuffer::<f64>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<f64> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2.5, 5.0, 7.5]);
        });
    }

    #[test]
    fn test_scale_empty() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type.call1(("i", PyList::empty(py))).unwrap();
            scale(py, arr, 5.0).unwrap(); // Should not panic
        });
    }

    #[test]
    fn test_scale_zero() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[1, 2, 3])))
                .unwrap();
            scale(py, arr, 0.0).unwrap();
            let buffer = PyBuffer::<i32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![0, 0, 0]);
        });
    }

    #[test]
    fn test_scale_negative() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[1, 2, 3])))
                .unwrap();
            scale(py, arr, -1.0).unwrap();
            let buffer = PyBuffer::<i32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![-1, -2, -3]);
        });
    }

    #[test]
    fn test_scale_not_array() {
        Python::with_gil(|py| {
            let list = PyList::new(py, &[1, 2, 3]);
            let result = scale(py, list, 2.0);
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Expected array.array"));
        });
    }

    #[test]
    fn test_sum_single_element() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type.call1(("i", PyList::new(py, &[42]))).unwrap();
            let result: i32 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 42);
        });
    }

    #[test]
    fn test_get_typecode_invalid_length() {
        Python::with_gil(|py| {
            // Create a Python class with a typecode attribute that's invalid length
            // This will test the length check on line 65
            let code = "class MockArray:\n    def __init__(self):\n        self.typecode = 'invalid'  # Length != 1\nobj = MockArray()";
            let globals = PyDict::new(py);
            let result = py.run(code, Some(globals), None);
            if result.is_ok() {
                if let Ok(obj) = globals.get_item("obj") {
                    if let Some(obj) = obj {
                        let result = get_typecode(obj);
                        assert!(
                            result.is_err(),
                            "Should fail for typecode string length != 1"
                        );
                        let err_msg = result.unwrap_err().to_string();
                        assert!(
                            err_msg.contains("Invalid typecode"),
                            "Error message was: {}",
                            err_msg
                        );
                        return;
                    }
                }
            }
            // Fallback: test that error occurs (may fail at different point)
            // but at least we exercise the code path
            let dict = PyDict::new(py);
            let _result = get_typecode(dict.as_ref());
            // This will fail, but that's ok - we're testing error handling
        });
    }

    #[test]
    fn test_module_initialization() {
        // Test the pymodule function (lines 231-234)
        Python::with_gil(|py| {
            let module = PyModule::new(py, "_arrayops").unwrap();
            let result = _arrayops(py, module);
            assert!(result.is_ok());

            // Verify functions are registered
            assert!(module.getattr("sum").is_ok());
            assert!(module.getattr("scale").is_ok());
        });
    }

    #[test]
    fn test_sum_large_array() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let values: Vec<i32> = (0..1000).collect();
            let arr = array_type.call1(("i", PyList::new(py, &values))).unwrap();
            let result: i32 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, (0..1000).sum::<i32>());
        });
    }

    #[test]
    fn test_module_initialization_with_new_functions() {
        Python::with_gil(|py| {
            let module = PyModule::new(py, "_arrayops").unwrap();
            let result = _arrayops(py, module);
            assert!(result.is_ok());

            // Verify all functions are registered
            assert!(module.getattr("sum").is_ok());
            assert!(module.getattr("scale").is_ok());
            assert!(module.getattr("map").is_ok());
            assert!(module.getattr("map_inplace").is_ok());
            assert!(module.getattr("filter").is_ok());
            assert!(module.getattr("reduce").is_ok());
            assert!(module.getattr("mean").is_ok());
            assert!(module.getattr("min").is_ok());
            assert!(module.getattr("max").is_ok());
            assert!(module.getattr("std").is_ok());
            assert!(module.getattr("var").is_ok());
            assert!(module.getattr("median").is_ok());
            assert!(module.getattr("add").is_ok());
            assert!(module.getattr("multiply").is_ok());
            assert!(module.getattr("clip").is_ok());
            assert!(module.getattr("normalize").is_ok());
            assert!(module.getattr("reverse").is_ok());
            assert!(module.getattr("sort").is_ok());
            assert!(module.getattr("unique").is_ok());
        });
    }

    // Tests for statistical operations
    #[test]
    fn test_mean_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[1, 2, 3, 4, 5])))
                .unwrap();
            let result: f64 = mean(py, arr).unwrap();
            assert!((result - 3.0).abs() < 1e-10);
        });
    }

    #[test]
    fn test_mean_float64() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("d", PyList::new(py, &[1.5, 2.5, 3.5, 4.5])))
                .unwrap();
            let result: f64 = mean(py, arr).unwrap();
            assert!((result - 3.0).abs() < 1e-10);
        });
    }

    #[test]
    fn test_mean_empty() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type.call1(("i", PyList::empty(py))).unwrap();
            let result = mean(py, arr);
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("empty"));
        });
    }

    #[test]
    fn test_min_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[5, 2, 8, 1, 9])))
                .unwrap();
            let result: i32 = min(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 1);
        });
    }

    #[test]
    fn test_max_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[5, 2, 8, 1, 9])))
                .unwrap();
            let result: i32 = max(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 9);
        });
    }

    #[test]
    fn test_var_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[1, 2, 3, 4, 5])))
                .unwrap();
            let result: f64 = var(py, arr).unwrap();
            // Population variance: sum((x-mean)^2)/n = 10/5 = 2.0
            assert!((result - 2.0).abs() < 1e-10);
        });
    }

    #[test]
    fn test_std_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[1, 2, 3, 4, 5])))
                .unwrap();
            let result: f64 = std_dev(py, arr).unwrap();
            // Population std: sqrt(2.0) ≈ 1.414
            assert!((result - (2.0_f64).sqrt()).abs() < 1e-10);
        });
    }

    #[test]
    fn test_median_int32_odd() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[5, 2, 8, 1, 9])))
                .unwrap();
            let result: i32 = median(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 5);
        });
    }

    #[test]
    fn test_median_int32_even() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[5, 2, 8, 1])))
                .unwrap();
            let result: i32 = median(py, arr).unwrap().extract(py).unwrap();
            // Sorted: [1, 2, 5, 8], lower median = 2
            assert_eq!(result, 2);
        });
    }

    // Tests for element-wise operations
    #[test]
    fn test_add_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr1 = array_type
                .call1(("i", PyList::new(py, &[1, 2, 3, 4, 5])))
                .unwrap();
            let arr2 = array_type
                .call1(("i", PyList::new(py, &[10, 20, 30, 40, 50])))
                .unwrap();
            let result = add(py, arr1, arr2).unwrap();
            let buffer = PyBuffer::<i32>::get(result.as_ref(py)).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![11, 22, 33, 44, 55]);
        });
    }

    #[test]
    fn test_multiply_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr1 = array_type
                .call1(("i", PyList::new(py, &[1, 2, 3, 4, 5])))
                .unwrap();
            let arr2 = array_type
                .call1(("i", PyList::new(py, &[2, 3, 4, 5, 6])))
                .unwrap();
            let result = multiply(py, arr1, arr2).unwrap();
            let buffer = PyBuffer::<i32>::get(result.as_ref(py)).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2, 6, 12, 20, 30]);
        });
    }

    #[test]
    fn test_clip_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[1, 5, 10, 15, 20])))
                .unwrap();
            clip(py, arr, 5.0, 15.0).unwrap();
            let buffer = PyBuffer::<i32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![5, 5, 10, 15, 15]);
        });
    }

    #[test]
    fn test_normalize_float64() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("d", PyList::new(py, &[10.0, 20.0, 30.0, 40.0, 50.0])))
                .unwrap();
            normalize(py, arr).unwrap();
            let buffer = PyBuffer::<f64>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<f64> = slice.iter().map(|cell| cell.get()).collect();
            // After normalization: (x - 10) / (50 - 10) = (x - 10) / 40
            let expected = vec![0.0, 0.25, 0.5, 0.75, 1.0];
            for (a, b) in values.iter().zip(expected.iter()) {
                assert!((a - b).abs() < 1e-10);
            }
        });
    }

    // Tests for array manipulation operations
    #[test]
    fn test_reverse_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[1, 2, 3, 4, 5])))
                .unwrap();
            reverse(py, arr).unwrap();
            let buffer = PyBuffer::<i32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![5, 4, 3, 2, 1]);
        });
    }

    #[test]
    fn test_sort_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[5, 2, 8, 1, 9])))
                .unwrap();
            sort(py, arr).unwrap();
            let buffer = PyBuffer::<i32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![1, 2, 5, 8, 9]);
        });
    }

    #[test]
    fn test_unique_int32() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type
                .call1(("i", PyList::new(py, &[5, 2, 8, 2, 1, 5, 9])))
                .unwrap();
            let result = unique(py, arr).unwrap();
            let buffer = PyBuffer::<i32>::get(result.as_ref(py)).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![1, 2, 5, 8, 9]);
        });
    }

    #[test]
    fn test_unique_empty() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            let arr = array_type.call1(("i", PyList::empty(py))).unwrap();
            let result = unique(py, arr).unwrap();
            let buffer = PyBuffer::<i32>::get(result.as_ref(py)).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            assert_eq!(slice.len(), 0);
        });
    }
}
