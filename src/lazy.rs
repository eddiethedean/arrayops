// Lazy evaluation system for arrayops
// This module provides lazy evaluation capabilities for chaining array operations

#![allow(non_local_definitions)] // PyO3 macro generates non-local impl

use pyo3::prelude::*;

/// Lazy operation types
pub enum LazyOp {
    Map { function: PyObject },
    Filter { predicate: PyObject },
    // Add other operations as needed
}

// PyObject doesn't implement Clone directly, but we can store it in an enum
// and clone it when we have Python GIL available. For now, we'll avoid deriving Clone
// and handle cloning manually in the methods that need it.

/// Lazy array wrapper for deferred evaluation
#[pyclass]
pub struct LazyArray {
    source: PyObject, // Original array or another LazyArray
    operations: Vec<LazyOp>,
    cached_result: Option<PyObject>, // Cached if evaluated
}

#[pymethods]
#[allow(non_local_definitions)] // PyO3 macro generates non-local impl
impl LazyArray {
    /// Create a new LazyArray from a source array
    #[new]
    pub fn new(source: PyObject) -> Self {
        LazyArray {
            source,
            operations: Vec::new(),
            cached_result: None,
        }
    }

    /// Apply a map operation (returns new LazyArray)
    fn map(&self, py: Python<'_>, function: PyObject) -> PyResult<Py<Self>> {
        let mut new_ops = Vec::with_capacity(self.operations.len() + 1);
        for op in &self.operations {
            match op {
                LazyOp::Map { function: f } => new_ops.push(LazyOp::Map {
                    function: f.clone_ref(py),
                }),
                LazyOp::Filter { predicate: p } => new_ops.push(LazyOp::Filter {
                    predicate: p.clone_ref(py),
                }),
            }
        }
        new_ops.push(LazyOp::Map {
            function: function.clone_ref(py),
        });

        Py::new(
            py,
            LazyArray {
                source: self.source.clone_ref(py),
                operations: new_ops,
                cached_result: None,
            },
        )
    }

    /// Apply a filter operation (returns new LazyArray)
    fn filter(&self, py: Python<'_>, predicate: PyObject) -> PyResult<Py<Self>> {
        let mut new_ops = Vec::with_capacity(self.operations.len() + 1);
        for op in &self.operations {
            match op {
                LazyOp::Map { function: f } => new_ops.push(LazyOp::Map {
                    function: f.clone_ref(py),
                }),
                LazyOp::Filter { predicate: p } => new_ops.push(LazyOp::Filter {
                    predicate: p.clone_ref(py),
                }),
            }
        }
        new_ops.push(LazyOp::Filter {
            predicate: predicate.clone_ref(py),
        });

        Py::new(
            py,
            LazyArray {
                source: self.source.clone_ref(py),
                operations: new_ops,
                cached_result: None,
            },
        )
    }

    /// Evaluate the lazy operations and return the result
    fn collect(&mut self, py: Python) -> PyResult<PyObject> {
        // If already cached, return cached result
        if let Some(ref cached) = self.cached_result {
            return Ok(cached.clone_ref(py));
        }

        // Import arrayops module to use existing functions
        let arrayops_module = PyModule::import(py, "arrayops._arrayops")?;

        // Start with the source array
        let mut current = self.source.clone_ref(py);

        // Apply each operation in sequence
        for op in &self.operations {
            let result = match op {
                LazyOp::Map { function } => {
                    let map_func = arrayops_module.getattr("map")?;
                    map_func.call1((current.clone_ref(py), function.clone_ref(py)))?
                }
                LazyOp::Filter { predicate } => {
                    let filter_func = arrayops_module.getattr("filter")?;
                    filter_func.call1((current.clone_ref(py), predicate.clone_ref(py)))?
                }
            };
            current = result.unbind();
        }

        // Cache the result
        self.cached_result = Some(current.clone_ref(py));
        Ok(current)
    }

    /// Get the source array
    fn source(&self, py: Python) -> PyObject {
        self.source.clone_ref(py)
    }

    /// Get the number of operations in the chain
    fn len(&self) -> usize {
        self.operations.len()
    }

    /// Iterator protocol: evaluate lazy chain and return an iterator
    fn __iter__(mut slf: PyRefMut<'_, Self>, py: Python) -> PyResult<PyObject> {
        // Evaluate the lazy chain first
        let result = slf.collect(py)?;

        // Import arrayops module to create ArrayIterator
        let arrayops_module = PyModule::import(py, "arrayops._arrayops")?;
        let array_iterator_func = arrayops_module.getattr("array_iterator")?;
        let iterator = array_iterator_func.call1((result,))?;
        Ok(iterator.unbind())
    }
}
