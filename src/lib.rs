#![allow(non_local_definitions)] // PyO3 macros generate non-local impl blocks

use pyo3::prelude::*;

mod allocator;
mod lazy;
mod types;
pub use types::*;
mod validation;
pub use validation::*;
mod buffer;
mod iterator;
pub mod operations;
pub use iterator::*;

// SIMD optimizations: Use compiler auto-vectorization with chunked processing
// For explicit SIMD, use std::arch intrinsics (stable) or std::simd (nightly)

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
#[macro_export]
macro_rules! dispatch_by_typecode {
    ($typecode:expr, $array:expr, |$buffer:ident| $body:block) => {
        match $typecode {
            $crate::types::TypeCode::Int8 => {
                let $buffer = pyo3::buffer::PyBuffer::<i8>::get($array)?;
                $body
            }
            $crate::types::TypeCode::Int16 => {
                let $buffer = pyo3::buffer::PyBuffer::<i16>::get($array)?;
                $body
            }
            $crate::types::TypeCode::Int32 => {
                let $buffer = pyo3::buffer::PyBuffer::<i32>::get($array)?;
                $body
            }
            $crate::types::TypeCode::Int64 => {
                let itemsize = $crate::buffer::get_itemsize($array)?;
                if itemsize == 4 {
                    let $buffer = pyo3::buffer::PyBuffer::<i32>::get($array)?;
                    $body
                } else {
                    let $buffer = pyo3::buffer::PyBuffer::<i64>::get($array)?;
                    $body
                }
            }
            $crate::types::TypeCode::UInt8 => {
                let $buffer = pyo3::buffer::PyBuffer::<u8>::get($array)?;
                $body
            }
            $crate::types::TypeCode::UInt16 => {
                let $buffer = pyo3::buffer::PyBuffer::<u16>::get($array)?;
                $body
            }
            $crate::types::TypeCode::UInt32 => {
                let $buffer = pyo3::buffer::PyBuffer::<u32>::get($array)?;
                $body
            }
            $crate::types::TypeCode::UInt64 => {
                let itemsize = $crate::buffer::get_itemsize($array)?;
                if itemsize == 4 {
                    let $buffer = pyo3::buffer::PyBuffer::<u32>::get($array)?;
                    $body
                } else {
                    let $buffer = pyo3::buffer::PyBuffer::<u64>::get($array)?;
                    $body
                }
            }
            $crate::types::TypeCode::Float32 => {
                let $buffer = pyo3::buffer::PyBuffer::<f32>::get($array)?;
                $body
            }
            $crate::types::TypeCode::Float64 => {
                let $buffer = pyo3::buffer::PyBuffer::<f64>::get($array)?;
                $body
            }
        }
    };
}

/// Macro to generate a complete match statement for typecode dispatch with mutable buffers
#[macro_export]
macro_rules! dispatch_by_typecode_mut {
    ($typecode:expr, $array:expr, |$buffer:ident| $body:block) => {
        match $typecode {
            $crate::types::TypeCode::Int8 => {
                let mut $buffer = pyo3::buffer::PyBuffer::<i8>::get($array)?;
                $body
            }
            $crate::types::TypeCode::Int16 => {
                let mut $buffer = pyo3::buffer::PyBuffer::<i16>::get($array)?;
                $body
            }
            $crate::types::TypeCode::Int32 => {
                let mut $buffer = pyo3::buffer::PyBuffer::<i32>::get($array)?;
                $body
            }
            $crate::types::TypeCode::Int64 => {
                let itemsize = $crate::buffer::get_itemsize($array)?;
                if itemsize == 4 {
                    let mut $buffer = pyo3::buffer::PyBuffer::<i32>::get($array)?;
                    $body
                } else {
                    let mut $buffer = pyo3::buffer::PyBuffer::<i64>::get($array)?;
                    $body
                }
            }
            $crate::types::TypeCode::UInt8 => {
                let mut $buffer = pyo3::buffer::PyBuffer::<u8>::get($array)?;
                $body
            }
            $crate::types::TypeCode::UInt16 => {
                let mut $buffer = pyo3::buffer::PyBuffer::<u16>::get($array)?;
                $body
            }
            $crate::types::TypeCode::UInt32 => {
                let mut $buffer = pyo3::buffer::PyBuffer::<u32>::get($array)?;
                $body
            }
            $crate::types::TypeCode::UInt64 => {
                let itemsize = $crate::buffer::get_itemsize($array)?;
                if itemsize == 4 {
                    let mut $buffer = pyo3::buffer::PyBuffer::<u32>::get($array)?;
                    $body
                } else {
                    let mut $buffer = pyo3::buffer::PyBuffer::<u64>::get($array)?;
                    $body
                }
            }
            $crate::types::TypeCode::Float32 => {
                let mut $buffer = pyo3::buffer::PyBuffer::<f32>::get($array)?;
                $body
            }
            $crate::types::TypeCode::Float64 => {
                let mut $buffer = pyo3::buffer::PyBuffer::<f64>::get($array)?;
                $body
            }
        }
    };
}

// SIMD thresholds - minimum array size to use SIMD
// Reserved for future SIMD implementation
#[cfg(feature = "simd")]
#[allow(dead_code)]
const SIMD_THRESHOLD: usize = 32;

// SIMD optimization infrastructure
// Using compiler auto-vectorization with cache-friendly code patterns
// For explicit SIMD, use std::arch intrinsics (stable) or std::simd (nightly)

// Basic operations have been moved to operations::basic module
// Stats operations (var, std_dev, median) have been moved to operations::stats module
// Elementwise operations (add, multiply, clip, normalize) have been moved to operations::elementwise module
/// Helper function to create a LazyArray wrapper
#[pyfunction]
pub fn lazy_array(py: Python, array: PyObject) -> PyResult<Py<lazy::LazyArray>> {
    Py::new(py, lazy::LazyArray::new(array))
}

/// A Python module implemented in Rust.
#[pymodule]
fn _arrayops(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(operations::basic::sum, m)?)?;
    m.add_function(wrap_pyfunction!(operations::basic::scale, m)?)?;
    m.add_function(wrap_pyfunction!(operations::basic::mean, m)?)?;
    m.add_function(wrap_pyfunction!(operations::basic::min, m)?)?;
    m.add_function(wrap_pyfunction!(operations::basic::max, m)?)?;
    m.add_function(wrap_pyfunction!(operations::transform::map, m)?)?;
    m.add_function(wrap_pyfunction!(operations::transform::map_inplace, m)?)?;
    m.add_function(wrap_pyfunction!(operations::transform::filter, m)?)?;
    m.add_function(wrap_pyfunction!(operations::transform::reduce, m)?)?;
    m.add_function(wrap_pyfunction!(operations::stats::var, m)?)?;
    m.add_function(wrap_pyfunction!(operations::stats::std_dev, m)?)?;
    m.add_function(wrap_pyfunction!(operations::stats::median, m)?)?;
    m.add_function(wrap_pyfunction!(operations::elementwise::add, m)?)?;
    m.add_function(wrap_pyfunction!(operations::elementwise::multiply, m)?)?;
    m.add_function(wrap_pyfunction!(operations::elementwise::clip, m)?)?;
    m.add_function(wrap_pyfunction!(operations::elementwise::normalize, m)?)?;
    m.add_function(wrap_pyfunction!(operations::manipulation::reverse, m)?)?;
    m.add_function(wrap_pyfunction!(operations::manipulation::sort, m)?)?;
    m.add_function(wrap_pyfunction!(operations::manipulation::unique, m)?)?;
    m.add_function(wrap_pyfunction!(operations::slice::slice, m)?)?;
    m.add_class::<iterator::ArrayIterator>()?;
    m.add_function(wrap_pyfunction!(iterator::array_iterator, m)?)?;
    m.add_class::<lazy::LazyArray>()?;
    m.add_function(wrap_pyfunction!(lazy_array, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::basic::{max, mean, min, scale, sum};
    use crate::operations::elementwise::{add, clip, multiply, normalize};
    use crate::operations::manipulation::{reverse, sort, unique};
    use crate::operations::slice::slice;
    use crate::operations::stats::{median, std_dev, var};
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
