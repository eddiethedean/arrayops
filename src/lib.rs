use pyo3::buffer::{Element, PyBuffer};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

// Note: Full SIMD support requires nightly Rust with portable_simd feature
// For now, SIMD optimizations are stubbed with scalar fallback
// Full implementation will be added when std::simd API stabilizes further

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
    let base_char = cleaned_format.chars().next().ok_or_else(|| {
        PyTypeError::new_err("Could not parse memoryview format string")
    })?;

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

/// Get typecode from unified input (array.array, numpy.ndarray, or memoryview)
fn get_typecode_unified(obj: &PyAny, input_type: InputType) -> PyResult<TypeCode> {
    match input_type {
        InputType::ArrayArray => get_typecode(obj),
        InputType::NumPyArray => get_numpy_typecode(obj),
        InputType::MemoryView => get_memoryview_typecode(obj),
    }
}

/// Input type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputType {
    ArrayArray,
    NumPyArray,
    MemoryView,
}

/// Detect the input type (array.array, numpy.ndarray, or memoryview)
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

    Err(PyTypeError::new_err(
        "Expected array.array, numpy.ndarray, or memoryview",
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
    }
}

// Parallel execution thresholds
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
const PARALLEL_THRESHOLD_SUM: usize = 10_000;
#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
const PARALLEL_THRESHOLD_SCALE: usize = 5_000;
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

// SIMD thresholds - minimum array size to use SIMD
// Reserved for future SIMD implementation
#[cfg(feature = "simd")]
#[allow(dead_code)]
const SIMD_THRESHOLD: usize = 32;

// SIMD optimization infrastructure
// Note: Full SIMD implementation requires std::simd API which is still evolving
// For now, SIMD feature flag is available but uses optimized scalar code
// This provides the structure for future SIMD implementation
// TODO: Implement full SIMD when std::simd API stabilizes or use portable-simd crate

// Generic sum implementation
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

    Ok(slice
        .iter()
        .map(|cell| cell.get())
        .fold(T::default(), |acc, x| acc + x))
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
    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            let result = sum_impl(py, &buffer, len)?;
            Ok(result.to_object(py))
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            let result = sum_impl(py, &buffer, len)?;
            Ok(result.to_object(py))
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            let result = sum_impl(py, &buffer, len)?;
            Ok(result.to_object(py))
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<i32>::get(array)?;
                let result = sum_impl(py, &buffer, len)?;
                Ok(result.to_object(py))
            } else {
                let buffer = PyBuffer::<i64>::get(array)?;
                let result = sum_impl(py, &buffer, len)?;
                Ok(result.to_object(py))
            }
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            let result = sum_impl(py, &buffer, len)?;
            Ok(result.to_object(py))
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            let result = sum_impl(py, &buffer, len)?;
            Ok(result.to_object(py))
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            let result = sum_impl(py, &buffer, len)?;
            Ok(result.to_object(py))
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<u32>::get(array)?;
                let result = sum_impl(py, &buffer, len)?;
                Ok(result.to_object(py))
            } else {
                let buffer = PyBuffer::<u64>::get(array)?;
                let result = sum_impl(py, &buffer, len)?;
                Ok(result.to_object(py))
            }
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            let result = sum_impl(py, &buffer, len)?;
            Ok(result.to_object(py))
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            let result = sum_impl(py, &buffer, len)?;
            Ok(result.to_object(py))
        }
    }
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

    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            map_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            map_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            map_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<i32>::get(array)?;
                map_impl(py, &buffer, callable, typecode, input_type)
            } else {
                let buffer = PyBuffer::<i64>::get(array)?;
                map_impl(py, &buffer, callable, typecode, input_type)
            }
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            map_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            map_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            map_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<u32>::get(array)?;
                map_impl(py, &buffer, callable, typecode, input_type)
            } else {
                let buffer = PyBuffer::<u64>::get(array)?;
                map_impl(py, &buffer, callable, typecode, input_type)
            }
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            map_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            map_impl(py, &buffer, callable, typecode, input_type)
        }
    }
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

    match typecode {
        TypeCode::Int8 => {
            let mut buffer = PyBuffer::<i8>::get(array)?;
            map_inplace_impl(py, &mut buffer, callable)
        }
        TypeCode::Int16 => {
            let mut buffer = PyBuffer::<i16>::get(array)?;
            map_inplace_impl(py, &mut buffer, callable)
        }
        TypeCode::Int32 => {
            let mut buffer = PyBuffer::<i32>::get(array)?;
            map_inplace_impl(py, &mut buffer, callable)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let mut buffer = PyBuffer::<i32>::get(array)?;
                map_inplace_impl(py, &mut buffer, callable)
            } else {
                let mut buffer = PyBuffer::<i64>::get(array)?;
                map_inplace_impl(py, &mut buffer, callable)
            }
        }
        TypeCode::UInt8 => {
            let mut buffer = PyBuffer::<u8>::get(array)?;
            map_inplace_impl(py, &mut buffer, callable)
        }
        TypeCode::UInt16 => {
            let mut buffer = PyBuffer::<u16>::get(array)?;
            map_inplace_impl(py, &mut buffer, callable)
        }
        TypeCode::UInt32 => {
            let mut buffer = PyBuffer::<u32>::get(array)?;
            map_inplace_impl(py, &mut buffer, callable)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let mut buffer = PyBuffer::<u32>::get(array)?;
                map_inplace_impl(py, &mut buffer, callable)
            } else {
                let mut buffer = PyBuffer::<u64>::get(array)?;
                map_inplace_impl(py, &mut buffer, callable)
            }
        }
        TypeCode::Float32 => {
            let mut buffer = PyBuffer::<f32>::get(array)?;
            map_inplace_impl(py, &mut buffer, callable)
        }
        TypeCode::Float64 => {
            let mut buffer = PyBuffer::<f64>::get(array)?;
            map_inplace_impl(py, &mut buffer, callable)
        }
    }
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

    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            filter_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            filter_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            filter_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<i32>::get(array)?;
                filter_impl(py, &buffer, callable, typecode, input_type)
            } else {
                let buffer = PyBuffer::<i64>::get(array)?;
                filter_impl(py, &buffer, callable, typecode, input_type)
            }
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            filter_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            filter_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            filter_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<u32>::get(array)?;
                filter_impl(py, &buffer, callable, typecode, input_type)
            } else {
                let buffer = PyBuffer::<u64>::get(array)?;
                filter_impl(py, &buffer, callable, typecode, input_type)
            }
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            filter_impl(py, &buffer, callable, typecode, input_type)
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            filter_impl(py, &buffer, callable, typecode, input_type)
        }
    }
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

    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            reduce_impl(py, &buffer, callable, initial)
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            reduce_impl(py, &buffer, callable, initial)
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            reduce_impl(py, &buffer, callable, initial)
        }
        TypeCode::Int64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<i32>::get(array)?;
                reduce_impl(py, &buffer, callable, initial)
            } else {
                let buffer = PyBuffer::<i64>::get(array)?;
                reduce_impl(py, &buffer, callable, initial)
            }
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            reduce_impl(py, &buffer, callable, initial)
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            reduce_impl(py, &buffer, callable, initial)
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            reduce_impl(py, &buffer, callable, initial)
        }
        TypeCode::UInt64 => {
            let itemsize = get_itemsize(array)?;
            if itemsize == 4 {
                let buffer = PyBuffer::<u32>::get(array)?;
                reduce_impl(py, &buffer, callable, initial)
            } else {
                let buffer = PyBuffer::<u64>::get(array)?;
                reduce_impl(py, &buffer, callable, initial)
            }
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            reduce_impl(py, &buffer, callable, initial)
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            reduce_impl(py, &buffer, callable, initial)
        }
    }
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
        });
    }
}
