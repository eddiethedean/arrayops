# Contributing Guide

Thank you for your interest in contributing to `arrayops`! This guide will help you get started.

## Getting Started

### Prerequisites

- Python 3.8 or higher
- Rust 1.75 or higher (required for SIMD optimizations)
- `maturin` (for building the extension)
- `git` for version control

### Setting Up Development Environment

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/your-username/arrayops.git
   cd arrayops
   ```

2. **Install development dependencies:**
   ```bash
   pip install -r requirements-dev.txt
   ```

3. **Build the package in development mode:**
   ```bash
   maturin develop
   ```

4. **Verify the installation:**
   ```bash
   python -c "import arrayops; print(arrayops.__version__)"
   ```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

Use descriptive branch names:
- `feature/add-map-operation`
- `fix/sum-overflow-handling`
- `docs/improve-api-docs`

### 2. Make Your Changes

- Write code following the style guidelines below
- Add tests for all new functionality
- Update documentation as needed
- Ensure 100% test coverage is maintained

### 3. Test Your Changes

```bash
# Run Python tests
pytest tests/ -v

# Run with coverage
pytest tests/ --cov=arrayops --cov-report=html

# Run Rust tests
cargo test --lib

# Check code quality
ruff format .
ruff check .
mypy arrayops tests
```

### 4. Commit Your Changes

Follow conventional commit messages:
```
feat: add map operation for array transformation
fix: handle empty arrays in sum operation
docs: update API reference with new examples
test: add tests for edge cases in scale operation
```

### 5. Submit a Pull Request

- Push your branch to your fork
- Open a pull request on GitHub
- Fill out the PR template
- Link any related issues

## Code Style Guidelines

### Python Code

- Follow PEP 8 style guide
- Use `ruff` for formatting and linting
- Maximum line length: 100 characters
- Use type hints where applicable
- Document all public functions with docstrings

**Example:**
```python
def new_function(arr: array.array, param: float) -> None:
    """Brief description of the function.
    
    Args:
        arr: Description of arr parameter
        param: Description of param parameter
    
    Raises:
        TypeError: When arr is not an array.array
    """
    # Implementation
```

### Rust Code

- Follow Rust standard formatting (`rustfmt`)
- Use `cargo clippy` for linting
- Document all public functions
- Use meaningful variable names
- Handle errors explicitly (no panics in library code)

**Example:**
```rust
/// Brief description of the function.
///
/// # Arguments
/// * `py` - Python interpreter instance
/// * `array` - Input array to process
///
/// # Returns
/// Result containing the processed value or error
fn new_function(py: Python, array: &PyAny) -> PyResult<PyObject> {
    // Implementation
}
```

## Testing Requirements

### Test Coverage

- **100% code coverage must be maintained**
- All code paths must be tested
- Include edge cases (empty arrays, single elements, large arrays)
- Test error conditions

### Writing Tests

**Python Tests:**
```python
import pytest
import array
import arrayops

def test_new_function_basic():
    """Test basic functionality."""
    arr = array.array('i', [1, 2, 3])
    result = arrayops.new_function(arr, 2.0)
    assert result == expected_value

def test_new_function_edge_cases():
    """Test edge cases."""
    empty = array.array('i', [])
    result = arrayops.new_function(empty, 1.0)
    assert result == 0
```

**Rust Tests:**
```rust
#[test]
fn test_new_function_int32() {
    Python::with_gil(|py| {
        let array_module = PyModule::import(py, "array").unwrap();
        let array_type = array_module.getattr("array").unwrap();
        let arr = array_type
            .call1(("i", PyList::new(py, &[1, 2, 3])))
            .unwrap();
        let result = new_function(py, arr).unwrap();
        // Assertions
    });
}
```

### Running Tests

```bash
# Python tests
pytest tests/ -v

# Rust tests
cargo test --lib

# Both with coverage
pytest tests/ --cov=arrayops --cov-report=term-missing
cargo tarpaulin --tests --lib
```

## Adding New Operations

### Step-by-Step Guide

1. **Design the API:**
   - Determine function signature
   - Consider in-place vs. new array
   - Plan error handling

2. **Implement in Rust (`src/lib.rs`):**
   - Add type dispatch for all supported types
   - Implement generic function
   - Add error handling
   - Write Rust unit tests

3. **Expose to Python:**
   - Add `#[pyfunction]` attribute
   - Register in `#[pymodule]` function
   - Update `__all__` in `arrayops/__init__.py`

4. **Add Type Stubs (`arrayops/_arrayops.pyi`):**
   - Add function signature with type hints
   - Include docstring

5. **Write Python Tests (`tests/`):**
   - Test all numeric types
   - Test edge cases
   - Test error conditions

6. **Update Documentation:**
   - Add to `docs/api.md`
   - Add examples to `docs/examples.md`
   - Update README if needed

### Example: Adding a New Operation

See the existing `sum` and `scale` implementations as reference:
- `src/lib.rs` - Rust implementation
- `arrayops/__init__.py` - Python wrapper
- `arrayops/_arrayops.pyi` - Type stubs
- `tests/test_basic.py` - Test examples

## Code Review Process

### What Reviewers Look For

1. **Correctness:**
   - Code works as intended
   - All tests pass
   - Edge cases handled

2. **Code Quality:**
   - Follows style guidelines
   - Well-documented
   - No unnecessary complexity

3. **Performance:**
   - Efficient implementation
   - No performance regressions
   - Appropriate use of zero-copy

4. **Testing:**
   - Comprehensive test coverage
   - Tests are clear and maintainable

### Responding to Feedback

- Be open to suggestions
- Ask questions if feedback is unclear
- Make requested changes promptly
- Update your PR when changes are made

## Documentation

### When to Update Documentation

- Adding new functions → Update `docs/api.md`
- New examples → Add to `docs/examples.md`
- Architecture changes → Update `docs/design.md`
- Breaking changes → Update `docs/CHANGELOG.md`

### Documentation Style

- Use clear, concise language
- Include code examples
- Cross-reference related docs
- Keep examples up-to-date

## Release Process

Contributors don't need to handle releases, but understanding the process helps:

1. Update version in `pyproject.toml` and `arrayops/__init__.py`
2. Update `docs/CHANGELOG.md`
3. Create git tag
4. Build and publish to PyPI

## Getting Help

- **Questions?** Open a GitHub discussion
- **Found a bug?** Open an issue
- **Need clarification?** Ask in PR comments
- **Stuck on something?** Check existing code for examples

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help others learn and grow

## Recognition

Contributors will be:
- Listed in the project README
- Credited in release notes
- Acknowledged in the project

Thank you for contributing to `arrayops`!

## Related Documentation

- [Development Guide](development.md) - Detailed development setup
- [API Reference](api.md) - Complete API documentation
- [Design Document](design.md) - Architecture details

