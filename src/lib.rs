use pyo3::prelude::*;
use pyo3::buffer::{PyBuffer, Element};
use pyo3::exceptions::PyTypeError;

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

// Generic sum implementation
fn sum_impl<T>(py: Python, buffer: &PyBuffer<T>) -> PyResult<T>
where
    T: Element + Copy + Default + std::ops::Add<Output = T> + pyo3::ToPyObject,
{
    let slice = unsafe { 
        buffer.as_slice(py).ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?
    };
    
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
    let slice = unsafe { 
        buffer.as_mut_slice(py).ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?
    };
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

/// A Python module implemented in Rust.
#[pymodule]
fn _arrayops(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum, m)?)?;
    m.add_function(wrap_pyfunction!(scale, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::PyList;

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
            // 'c' is char type, not supported
            let arr = array_type
                .call1(("c", PyList::new(py, &["a", "b", "c"])))
                .unwrap();
            let result = sum(py, arr);
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Unsupported typecode"));
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
}
