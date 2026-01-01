use pyo3::prelude::*;

use crate::buffer::{extract_element_at_index, get_array_len, get_itemsize};
use crate::types::TypeCode;
use crate::validation::{detect_input_type, get_typecode_unified, validate_for_operation};

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
    pub fn new(py: Python<'_>, source: PyObject) -> PyResult<Self> {
        let source_ref = source.bind(py);
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
        let source_ref = source.bind(py);
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
pub fn array_iterator(py: Python<'_>, array: PyObject) -> PyResult<Py<ArrayIterator>> {
    Py::new(py, ArrayIterator::new(py, array)?)
}
