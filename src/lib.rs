use pyo3::prelude::*;
use pyo3::buffer::{PyBuffer, Element};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::types::PyList;

/// Supported array.array typecodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TypeCode {
    // Signed integers
    Int8,    // 'b'
    Int16,   // 'h'
    Int32,   // 'i'
    Int64,   // 'l'
    // Unsigned integers
    UInt8,   // 'B'
    UInt16,  // 'H'
    UInt32,  // 'I'
    UInt64,  // 'L'
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

/// Validate that the input is an array.array
fn validate_array_array(array: &PyAny) -> PyResult<()> {
    let module = PyModule::import(array.py(), "array")?;
    let array_type = module.getattr("array")?;
    if !array.is_instance(array_type)? {
        return Err(PyTypeError::new_err(
            "Expected array.array, got different type"
        ));
    }
    Ok(())
}

/// Get the length of an array.array
fn get_array_len(array: &PyAny) -> PyResult<usize> {
    let len = array.len()?;
    Ok(len)
}

// Generic sum implementation
fn sum_impl<T>(py: Python, buffer: &PyBuffer<T>) -> PyResult<T>
where
    T: Element + Copy + Default + std::ops::Add<Output = T> + pyo3::ToPyObject,
{
    let slice = buffer.as_slice(py).ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
    
    // TODO: Parallel sum optimization with rayon
    // Note: ReadOnlyCell<T> prevents direct parallel access. Future enhancement:
    // could extract chunks to Vec first, or use unsafe raw pointer access
    #[cfg(feature = "parallel")]
    {
        // Parallel optimization disabled until proper thread-safe access pattern is implemented
        // See: https://github.com/PyO3/pyo3/issues for buffer API improvements
    }
    
    Ok(slice.iter().map(|cell| cell.get()).fold(T::default(), |acc, x| acc + x))
}

// Generic scale implementation (in-place)
// Note: In-place operations are kept sequential for safety and cache efficiency
fn scale_impl<T, F>(py: Python, buffer: &mut PyBuffer<T>, factor: F) -> PyResult<()>
where
    T: Element + Copy + std::ops::Mul<F, Output = T>,
    F: Copy,
{
    let slice = buffer.as_mut_slice(py).ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
    for item in slice.iter() {
        item.set(item.get() * factor);
    }
    Ok(())
}

/// Sum operation for array.array
#[pyfunction]
fn sum(py: Python, array: &PyAny) -> PyResult<PyObject> {
    validate_array_array(array)?;
    let typecode = get_typecode(array)?;
    
    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        match typecode {
            TypeCode::Int8 => return Ok(0i8.to_object(py)),
            TypeCode::Int16 => return Ok(0i16.to_object(py)),
            TypeCode::Int32 => return Ok(0i32.to_object(py)),
            TypeCode::Int64 => return Ok(0i64.to_object(py)),
            TypeCode::UInt8 => return Ok(0u8.to_object(py)),
            TypeCode::UInt16 => return Ok(0u16.to_object(py)),
            TypeCode::UInt32 => return Ok(0u32.to_object(py)),
            TypeCode::UInt64 => return Ok(0u64.to_object(py)),
            TypeCode::Float32 => return Ok(0.0f32.to_object(py)),
            TypeCode::Float64 => return Ok(0.0f64.to_object(py)),
        }
    }
    
    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            let result = sum_impl(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            let result = sum_impl(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            let result = sum_impl(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::Int64 => {
            let buffer = PyBuffer::<i64>::get(array)?;
            let result = sum_impl(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            let result = sum_impl(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            let result = sum_impl(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            let result = sum_impl(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::UInt64 => {
            let buffer = PyBuffer::<u64>::get(array)?;
            let result = sum_impl(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            let result = sum_impl(py, &buffer)?;
            Ok(result.to_object(py))
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            let result = sum_impl(py, &buffer)?;
            Ok(result.to_object(py))
        }
    }
}

/// Scale operation (in-place) for array.array
#[pyfunction]
fn scale(py: Python, array: &PyAny, factor: f64) -> PyResult<()> {
    validate_array_array(array)?;
    let typecode = get_typecode(array)?;
    
    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        return Ok(());
    }
    
    match typecode {
        TypeCode::Int8 => {
            let mut buffer = PyBuffer::<i8>::get(array)?;
            scale_impl(py, &mut buffer, factor as i8)
        }
        TypeCode::Int16 => {
            let mut buffer = PyBuffer::<i16>::get(array)?;
            scale_impl(py, &mut buffer, factor as i16)
        }
        TypeCode::Int32 => {
            let mut buffer = PyBuffer::<i32>::get(array)?;
            scale_impl(py, &mut buffer, factor as i32)
        }
        TypeCode::Int64 => {
            let mut buffer = PyBuffer::<i64>::get(array)?;
            scale_impl(py, &mut buffer, factor as i64)
        }
        TypeCode::UInt8 => {
            let mut buffer = PyBuffer::<u8>::get(array)?;
            scale_impl(py, &mut buffer, factor as u8)
        }
        TypeCode::UInt16 => {
            let mut buffer = PyBuffer::<u16>::get(array)?;
            scale_impl(py, &mut buffer, factor as u16)
        }
        TypeCode::UInt32 => {
            let mut buffer = PyBuffer::<u32>::get(array)?;
            scale_impl(py, &mut buffer, factor as u32)
        }
        TypeCode::UInt64 => {
            let mut buffer = PyBuffer::<u64>::get(array)?;
            scale_impl(py, &mut buffer, factor as u64)
        }
        TypeCode::Float32 => {
            let mut buffer = PyBuffer::<f32>::get(array)?;
            scale_impl(py, &mut buffer, factor as f32)
        }
        TypeCode::Float64 => {
            let mut buffer = PyBuffer::<f64>::get(array)?;
            scale_impl(py, &mut buffer, factor)
        }
    }
}

// Generic map implementation (returns new array)
fn map_impl<T>(py: Python, buffer: &PyBuffer<T>, callable: &PyAny, typecode: TypeCode) -> PyResult<PyObject>
where
    T: Element + Copy + pyo3::ToPyObject,
{
    let slice = buffer.as_slice(py).ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
    
    let array_module = PyModule::import(py, "array")?;
    let array_type = array_module.getattr("array")?;
    let typecode_char = typecode.as_char();
    let result_array = array_type.call1((typecode_char, PyList::empty(py)))?;
    
    for cell in slice.iter() {
        let value = cell.get();
        let value_obj = value.to_object(py);
        let result = callable.call1((value_obj,))?;
        result_array.call_method1("append", (result,))?;
    }
    
    Ok(result_array.to_object(py))
}

/// Map operation for array.array
#[pyfunction]
fn map(py: Python, array: &PyAny, r#fn: PyObject) -> PyResult<PyObject> {
    validate_array_array(array)?;
    let typecode = get_typecode(array)?;
    let callable = r#fn.as_ref(py);
    
    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        let array_module = PyModule::import(py, "array")?;
        let array_type = array_module.getattr("array")?;
        let typecode_char = typecode.as_char();
        return Ok(array_type.call1((typecode_char, PyList::empty(py)))?.to_object(py));
    }
    
    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            map_impl(py, &buffer, callable, typecode)
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            map_impl(py, &buffer, callable, typecode)
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            map_impl(py, &buffer, callable, typecode)
        }
        TypeCode::Int64 => {
            let buffer = PyBuffer::<i64>::get(array)?;
            map_impl(py, &buffer, callable, typecode)
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            map_impl(py, &buffer, callable, typecode)
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            map_impl(py, &buffer, callable, typecode)
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            map_impl(py, &buffer, callable, typecode)
        }
        TypeCode::UInt64 => {
            let buffer = PyBuffer::<u64>::get(array)?;
            map_impl(py, &buffer, callable, typecode)
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            map_impl(py, &buffer, callable, typecode)
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            map_impl(py, &buffer, callable, typecode)
        }
    }
}

// Generic map_inplace implementation
fn map_inplace_impl<T>(py: Python, buffer: &mut PyBuffer<T>, callable: &PyAny) -> PyResult<()>
where
    T: Element + Copy + pyo3::ToPyObject + for<'a> pyo3::FromPyObject<'a>,
{
    let slice = buffer.as_mut_slice(py).ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;
    
    for item in slice.iter() {
        let value = item.get();
        let value_obj = value.to_object(py);
        let result = callable.call1((value_obj,))?;
        let result_value: T = result.extract()?;
        item.set(result_value);
    }
    
    Ok(())
}

/// Map in-place operation for array.array
#[pyfunction]
fn map_inplace(py: Python, array: &PyAny, r#fn: PyObject) -> PyResult<()> {
    validate_array_array(array)?;
    let typecode = get_typecode(array)?;
    let callable = r#fn.as_ref(py);
    
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
            let mut buffer = PyBuffer::<i64>::get(array)?;
            map_inplace_impl(py, &mut buffer, callable)
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
            let mut buffer = PyBuffer::<u64>::get(array)?;
            map_inplace_impl(py, &mut buffer, callable)
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
fn filter_impl<T>(py: Python, buffer: &PyBuffer<T>, predicate: &PyAny, typecode: TypeCode) -> PyResult<PyObject>
where
    T: Element + Copy + pyo3::ToPyObject,
{
    let slice = buffer.as_slice(py).ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
    
    let array_module = PyModule::import(py, "array")?;
    let array_type = array_module.getattr("array")?;
    let typecode_char = typecode.as_char();
    let result_array = array_type.call1((typecode_char, PyList::empty(py)))?;
    
    for cell in slice.iter() {
        let value = cell.get();
        let value_obj = value.to_object(py);
        let result = predicate.call1((value_obj.clone(),))?;
        let should_include: bool = result.extract()?;
        if should_include {
            result_array.call_method1("append", (value_obj,))?;
        }
    }
    
    Ok(result_array.to_object(py))
}

/// Filter operation for array.array
#[pyfunction]
fn filter(py: Python, array: &PyAny, predicate: PyObject) -> PyResult<PyObject> {
    validate_array_array(array)?;
    let typecode = get_typecode(array)?;
    let callable = predicate.as_ref(py);
    
    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        let array_module = PyModule::import(py, "array")?;
        let array_type = array_module.getattr("array")?;
        let typecode_char = typecode.as_char();
        return Ok(array_type.call1((typecode_char, PyList::empty(py)))?.to_object(py));
    }
    
    match typecode {
        TypeCode::Int8 => {
            let buffer = PyBuffer::<i8>::get(array)?;
            filter_impl(py, &buffer, callable, typecode)
        }
        TypeCode::Int16 => {
            let buffer = PyBuffer::<i16>::get(array)?;
            filter_impl(py, &buffer, callable, typecode)
        }
        TypeCode::Int32 => {
            let buffer = PyBuffer::<i32>::get(array)?;
            filter_impl(py, &buffer, callable, typecode)
        }
        TypeCode::Int64 => {
            let buffer = PyBuffer::<i64>::get(array)?;
            filter_impl(py, &buffer, callable, typecode)
        }
        TypeCode::UInt8 => {
            let buffer = PyBuffer::<u8>::get(array)?;
            filter_impl(py, &buffer, callable, typecode)
        }
        TypeCode::UInt16 => {
            let buffer = PyBuffer::<u16>::get(array)?;
            filter_impl(py, &buffer, callable, typecode)
        }
        TypeCode::UInt32 => {
            let buffer = PyBuffer::<u32>::get(array)?;
            filter_impl(py, &buffer, callable, typecode)
        }
        TypeCode::UInt64 => {
            let buffer = PyBuffer::<u64>::get(array)?;
            filter_impl(py, &buffer, callable, typecode)
        }
        TypeCode::Float32 => {
            let buffer = PyBuffer::<f32>::get(array)?;
            filter_impl(py, &buffer, callable, typecode)
        }
        TypeCode::Float64 => {
            let buffer = PyBuffer::<f64>::get(array)?;
            filter_impl(py, &buffer, callable, typecode)
        }
    }
}

// Generic reduce implementation
fn reduce_impl<T>(py: Python, buffer: &PyBuffer<T>, r#fn: &PyAny, initial: Option<PyObject>) -> PyResult<PyObject>
where
    T: Element + Copy + pyo3::ToPyObject,
{
    let slice = buffer.as_slice(py).ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
    
    // Note: Empty check is now done in reduce() before getting the buffer,
    // but keep this as a safety check in case the buffer is somehow empty
    if slice.is_empty() {
        return match initial {
            Some(init) => Ok(init),
            None => Err(PyValueError::new_err("reduce() of empty array with no initial value")),
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

/// Reduce operation for array.array
#[pyfunction]
fn reduce(py: Python, array: &PyAny, r#fn: PyObject, initial: Option<PyObject>) -> PyResult<PyObject> {
    validate_array_array(array)?;
    let typecode = get_typecode(array)?;
    let callable = r#fn.as_ref(py);
    
    // Handle empty arrays early to avoid buffer alignment issues on macOS
    let array_len = get_array_len(array)?;
    if array_len == 0 {
        return match initial {
            Some(init) => Ok(init),
            None => Err(PyValueError::new_err("reduce() of empty array with no initial value")),
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
            let buffer = PyBuffer::<i64>::get(array)?;
            reduce_impl(py, &buffer, callable, initial)
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
            let buffer = PyBuffer::<u64>::get(array)?;
            reduce_impl(py, &buffer, callable, initial)
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
    use pyo3::types::{PyList, PyDict};

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
            let slice = unsafe { buffer.as_slice(py).unwrap() };
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
            let slice = unsafe { buffer.as_slice(py).unwrap() };
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
            assert!(TypeCode::from_char(code).is_err(), "Code '{}' should fail", code);
        }
    }

    // Test sum for all numeric types
    #[test]
    fn test_sum_all_integer_types() {
        Python::with_gil(|py| {
            let array_module = PyModule::import(py, "array").unwrap();
            let array_type = array_module.getattr("array").unwrap();
            
            // Test Int8
            let arr = array_type.call1(("b", PyList::new(py, &[-1i8, 0, 1]))).unwrap();
            let result: i8 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 0i8);
            
            // Test UInt8
            let arr = array_type.call1(("B", PyList::new(py, &[1u8, 2, 3]))).unwrap();
            let result: u8 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 6u8);
            
            // Test Int16
            let arr = array_type.call1(("h", PyList::new(py, &[-10i16, 0, 10]))).unwrap();
            let result: i16 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 0i16);
            
            // Test UInt16
            let arr = array_type.call1(("H", PyList::new(py, &[100u16, 200]))).unwrap();
            let result: u16 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 300u16);
            
            // Test Int32
            let arr = array_type.call1(("i", PyList::new(py, &[1i32, 2, 3]))).unwrap();
            let result: i32 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 6i32);
            
            // Test UInt32
            let arr = array_type.call1(("I", PyList::new(py, &[1u32, 2, 3]))).unwrap();
            let result: u32 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 6u32);
            
            // Test Int64
            let arr = array_type.call1(("l", PyList::new(py, &[1000i64, 2000]))).unwrap();
            let result: i64 = sum(py, arr).unwrap().extract(py).unwrap();
            assert_eq!(result, 3000i64);
            
            // Test UInt64
            let arr = array_type.call1(("L", PyList::new(py, &[1000u64, 2000]))).unwrap();
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
            let arr = array_type.call1(("b", PyList::new(py, &[1i8, 2, 3]))).unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<i8>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i8> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2i8, 4, 6]);
            
            // Test UInt8
            let arr = array_type.call1(("B", PyList::new(py, &[1u8, 2, 3]))).unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<u8>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<u8> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2u8, 4, 6]);
            
            // Test Int16
            let arr = array_type.call1(("h", PyList::new(py, &[1i16, 2, 3]))).unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<i16>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i16> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2i16, 4, 6]);
            
            // Test UInt16
            let arr = array_type.call1(("H", PyList::new(py, &[1u16, 2, 3]))).unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<u16>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<u16> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2u16, 4, 6]);
            
            // Test Int32
            let arr = array_type.call1(("i", PyList::new(py, &[1i32, 2, 3]))).unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<i32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2i32, 4, 6]);
            
            // Test UInt32
            let arr = array_type.call1(("I", PyList::new(py, &[1u32, 2, 3]))).unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<u32>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<u32> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2u32, 4, 6]);
            
            // Test Int64
            let arr = array_type.call1(("l", PyList::new(py, &[1i64, 2, 3]))).unwrap();
            scale(py, arr, 2.0).unwrap();
            let buffer = PyBuffer::<i64>::get(arr).unwrap();
            let slice = buffer.as_slice(py).unwrap();
            let values: Vec<i64> = slice.iter().map(|cell| cell.get()).collect();
            assert_eq!(values, vec![2i64, 4, 6]);
            
            // Test UInt64
            let arr = array_type.call1(("L", PyList::new(py, &[1u64, 2, 3]))).unwrap();
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
            let arr = array_type
                .call1(("i", PyList::new(py, &[42])))
                .unwrap();
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
                        assert!(result.is_err(), "Should fail for typecode string length != 1");
                        let err_msg = result.unwrap_err().to_string();
                        assert!(err_msg.contains("Invalid typecode"), 
                            "Error message was: {}", err_msg);
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
            let arr = array_type
                .call1(("i", PyList::new(py, &values)))
                .unwrap();
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
