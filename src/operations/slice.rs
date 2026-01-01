use pyo3::prelude::*;

use crate::buffer::{create_slice_view_helper, get_array_len};
use crate::validation::{detect_input_type, validate_for_operation};

/// Slice operation - returns a zero-copy memoryview of a portion of the array
#[pyfunction]
pub fn slice(
    py: Python<'_>,
    array: &Bound<'_, PyAny>,
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
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Invalid slice indices: start={start_idx}, end={end_idx}, length={len}"
        )));
    }

    // Use the helper function to create the slice view
    create_slice_view_helper(py, array, start_idx, end_idx)
}
