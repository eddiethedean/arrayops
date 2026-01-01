use pyo3::buffer::{Element, PyBuffer};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;

use crate::buffer::{create_empty_result_array, create_result_array_from_list, get_array_len};
use crate::types::TypeCode;
use crate::validation::{
    detect_input_type, get_typecode_unified, validate_for_operation, InputType,
};

// Generic map implementation (returns new array)
fn map_impl<T>(
    py: Python<'_>,
    buffer: &PyBuffer<T>,
    callable: &Bound<'_, PyAny>,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + IntoPy<PyObject>,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    let result_list = PyList::empty(py);

    for cell in slice.iter() {
        let value = cell.get();
        let value_obj = value.into_py(py);
        let result = callable.call1((value_obj,))?;
        result_list.append(result).unwrap();
    }

    create_result_array_from_list(py, typecode, input_type, &result_list)
}

/// Map operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn map(py: Python<'_>, array: &Bound<'_, PyAny>, r#fn: PyObject) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;
    let callable = r#fn.bind(py);

    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        return create_empty_result_array(py, typecode, input_type);
    }

    crate::dispatch_by_typecode!(typecode, array, |buffer| {
        map_impl(py, &buffer, callable, typecode, input_type)
    })
}

// Generic map_inplace implementation
fn map_inplace_impl<T>(
    py: Python<'_>,
    buffer: &mut PyBuffer<T>,
    callable: &Bound<'_, PyAny>,
) -> PyResult<()>
where
    T: Element + Copy + IntoPy<PyObject> + for<'a> pyo3::FromPyObject<'a>,
{
    let slice = buffer
        .as_mut_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get mutable buffer slice"))?;

    for item in slice.iter() {
        let value = item.get();
        let value_obj = value.into_py(py);
        let result = callable.call1((value_obj,))?;
        let result_value: T = result.extract()?;
        item.set(result_value);
    }

    Ok(())
}

/// Map in-place operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn map_inplace(py: Python<'_>, array: &Bound<'_, PyAny>, r#fn: PyObject) -> PyResult<()> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, true)?;
    let typecode = get_typecode_unified(array, input_type)?;
    let callable = r#fn.bind(py);

    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        return Ok(());
    }

    crate::dispatch_by_typecode_mut!(typecode, array, |buffer| {
        map_inplace_impl(py, &mut buffer, callable)
    })
}

// Generic filter implementation
fn filter_impl<T>(
    py: Python<'_>,
    buffer: &PyBuffer<T>,
    predicate: &Bound<'_, PyAny>,
    typecode: TypeCode,
    input_type: InputType,
) -> PyResult<PyObject>
where
    T: Element + Copy + IntoPy<PyObject>,
{
    let slice = buffer
        .as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;

    let result_list = PyList::empty(py);

    for cell in slice.iter() {
        let value = cell.get();
        let value_obj = value.into_py(py);
        let result = predicate.call1((value_obj.clone_ref(py),))?;
        let should_include: bool = result.extract()?;
        if should_include {
            result_list.append(value_obj).unwrap();
        }
    }

    create_result_array_from_list(py, typecode, input_type, &result_list)
}

/// Filter operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
pub fn filter(py: Python<'_>, array: &Bound<'_, PyAny>, predicate: PyObject) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;
    let callable = predicate.bind(py);

    // Handle empty arrays early to avoid buffer alignment issues on macOS
    if get_array_len(array)? == 0 {
        return create_empty_result_array(py, typecode, input_type);
    }

    crate::dispatch_by_typecode!(typecode, array, |buffer| {
        filter_impl(py, &buffer, callable, typecode, input_type)
    })
}

// Generic reduce implementation
fn reduce_impl<T>(
    py: Python<'_>,
    buffer: &PyBuffer<T>,
    r#fn: &Bound<'_, PyAny>,
    initial: Option<PyObject>,
) -> PyResult<PyObject>
where
    T: Element + Copy + IntoPy<PyObject>,
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
            (first.into_py(py), 1)
        }
    };
    for cell in slice.iter().skip(start_idx) {
        let value = cell.get();
        let value_obj = value.into_py(py);
        let result = r#fn.call1((acc, value_obj))?;
        acc = result.into_py(py);
    }

    Ok(acc)
}

/// Reduce operation for array.array, numpy.ndarray, or memoryview
#[pyfunction]
#[pyo3(signature = (array, r#fn, *, initial = None))]
pub fn reduce(
    py: Python<'_>,
    array: &Bound<'_, PyAny>,
    r#fn: PyObject,
    initial: Option<PyObject>,
) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    validate_for_operation(array, input_type, false)?;
    let typecode = get_typecode_unified(array, input_type)?;
    let callable = r#fn.bind(py);

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

    crate::dispatch_by_typecode!(typecode, array, |buffer| {
        reduce_impl(py, &buffer, callable, initial)
    })
}
