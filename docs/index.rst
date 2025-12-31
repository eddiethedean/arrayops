.. arrayops documentation master file

Welcome to arrayops Documentation
==================================

**Rust-backed acceleration for Python's ``array.array`` type**

.. image:: https://img.shields.io/pypi/v/ao.svg
   :target: https://pypi.org/project/arrayops/
   :alt: PyPI version

.. image:: https://img.shields.io/badge/python-3.8+-blue.svg
   :target: https://www.python.org/downloads/
   :alt: Python version

.. image:: https://img.shields.io/badge/rust-1.75+-orange.svg
   :target: https://www.rust-lang.org/
   :alt: Rust version

.. image:: https://img.shields.io/badge/coverage-100%25-brightgreen.svg
   :target: https://github.com/eddiethedean/arrayops
   :alt: Code coverage

Fast, lightweight numeric operations for Python's ``array.array``, ``numpy.ndarray`` (1D), and ``memoryview`` objects. Built with Rust and PyO3 for zero-copy, memory-safe performance.

.. toctree::
   :maxdepth: 2
   :caption: Getting Started

   getting-started/index

.. toctree::
   :maxdepth: 2
   :caption: User Guide

   user-guide/api
   user-guide/examples
   user-guide/performance
   user-guide/troubleshooting

.. toctree::
   :maxdepth: 2
   :caption: Developer Guide

   developer-guide/contributing
   developer-guide/development
   developer-guide/design
   developer-guide/coverage

.. toctree::
   :maxdepth: 2
   :caption: Reference

   reference/changelog
   reference/roadmap


Key Features
------------

* ⚡ **High Performance**: 10-100x faster than pure Python loops
* 🔒 **Memory Safe**: Zero-copy buffer access with Rust's safety guarantees
* 📦 **Lightweight**: No dependencies beyond Rust standard library (optional: parallel execution via ``rayon``)
* 🔌 **Compatible**: Works directly with Python's ``array.array``, ``numpy.ndarray`` (1D), and ``memoryview`` - no new types
* ✅ **Fully Tested**: 100% code coverage (Python and Rust)
* 🎯 **Type Safe**: Full mypy type checking support
* 🚀 **Optional Optimizations**: Parallel execution and SIMD support via feature flags

Quick Start
-----------

.. code-block:: python

   import array
   import arrayops

   # Create an array
   data = array.array('i', [1, 2, 3, 4, 5])

   # Fast sum operation
   total = ao.sum(data)
   print(total)  # 15

   # In-place scaling
   ao.scale(data, 2.0)
   print(list(data))  # [2, 4, 6, 8, 10]

   # Map operation (returns new array)
   doubled = ao.map(data, lambda x: x * 2)
   print(list(doubled))  # [4, 8, 12, 16, 20]

Installation
------------

.. code-block:: bash

   # Install maturin if not already installed
   pip install maturin

   # Install in development mode
   maturin develop

   # Or install from source
   pip install -e .

   # With optional features
   maturin develop --features parallel

Documentation Overview
----------------------

**Getting Started**
   New to arrayops? Start here to learn the basics.

**User Guide**
   Comprehensive guides for using arrayops, including API reference, examples, performance tips, and troubleshooting.

**Developer Guide**
   Information for contributors, including development setup, architecture details, and contribution guidelines.

**Reference**
   Reference materials including changelog and roadmap.

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`

