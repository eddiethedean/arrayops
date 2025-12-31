# Development Guide

Comprehensive guide for setting up and working with the `arrayops` development environment.

## Prerequisites

### Required Software

- **Python 3.8+**: [Download Python](https://www.python.org/downloads/)
- **Rust 1.70+**: [Install Rust](https://www.rust-lang.org/tools/install)
- **maturin**: Python-Rust build tool
  ```bash
  pip install maturin
  ```
- **Git**: Version control

### Optional Tools

- **pytest**: Testing framework (included in requirements-dev.txt)
- **ruff**: Python linter and formatter
- **mypy**: Static type checker
- **cargo-tarpaulin**: Rust code coverage tool
  ```bash
  cargo install cargo-tarpaulin
  ```

## Initial Setup

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/arrayops.git
cd arrayops
```

### 2. Install Development Dependencies

```bash
pip install -r requirements-dev.txt
```

This installs:
- `maturin` - Build tool for Rust extensions
- `pytest` - Testing framework
- `pytest-cov` - Coverage plugin

### 3. Build in Development Mode

```bash
maturin develop
```

This compiles the Rust extension and installs it in development mode. Changes to Rust code require rebuilding.

### 4. Verify Installation

```bash
python -c "import arrayops; print(arrayops.__version__)"
# Should output: 0.1.0
```

## Project Structure

```
arrayops/
├── arrayops/              # Python package
│   ├── __init__.py       # Package initialization
│   └── _arrayops.pyi     # Type stubs for mypy
├── src/                  # Rust source code
│   └── lib.rs           # Main Rust implementation
├── tests/                # Python tests
│   └── test_basic.py     # Test suite
├── docs/                 # Documentation
├── Cargo.toml           # Rust project configuration
├── pyproject.toml       # Python project configuration
└── README.md            # Main documentation
```

## Build Process

### Development Build

```bash
maturin develop
```

- Compiles Rust code in debug mode
- Installs package in editable mode
- Faster compilation, slower runtime

### Release Build

```bash
maturin build --release
```

- Compiles Rust code with optimizations
- Creates wheel distribution
- Slower compilation, faster runtime

### Build for Specific Python Version

```bash
PYO3_PYTHON=/path/to/python maturin build --release
```

## Testing

### Python Tests

```bash
# Run all tests
pytest tests/ -v

# Run with coverage
pytest tests/ --cov=arrayops --cov-report=html

# Run specific test file
pytest tests/test_basic.py -v

# Run specific test
pytest tests/test_basic.py::TestSum::test_sum_int32 -v

# Run in parallel (if pytest-xdist installed)
pytest tests/ -n auto
```

### Rust Tests

```bash
# Run all Rust tests
cargo test --lib

# Run with output
cargo test --lib -- --nocapture

# Run specific test
cargo test --lib test_sum_int32
```

### Test Coverage

**Python Coverage:**
```bash
pytest tests/ --cov=arrayops --cov-report=term-missing
```

**Rust Coverage:**
```bash
cargo tarpaulin --tests --lib --out Html
```

Coverage reports are generated in `htmlcov/` (Python) and `tarpaulin-report.html` (Rust).

### Coverage Requirements

- **100% code coverage must be maintained**
- All code paths must be tested
- Edge cases must be covered
- Error conditions must be tested

## Code Quality

### Python Code Formatting

```bash
# Format code
ruff format .

# Check formatting
ruff format --check .
```

### Python Linting

```bash
# Run linter
ruff check .

# Auto-fix issues
ruff check --fix .
```

### Type Checking

```bash
# Check types with mypy
mypy arrayops tests

# Check specific file
mypy arrayops/__init__.py
```

### Rust Code Formatting

```bash
# Format Rust code
cargo fmt

# Check formatting
cargo fmt --check
```

### Rust Linting

```bash
# Run clippy
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

## Debugging

### Python Debugging

Use standard Python debugging tools:

```python
import pdb; pdb.set_trace()  # Breakpoint
```

Or use your IDE's debugger.

### Rust Debugging

1. **Compile with debug symbols:**
   ```bash
   maturin develop  # Already includes debug symbols
   ```

2. **Use Rust debugger (gdb/lldb):**
   ```bash
   # Set breakpoint in lib.rs
   # Attach debugger to Python process
   ```

3. **Add debug prints:**
   ```rust
   eprintln!("Debug: value = {:?}", value);
   ```

### Common Debugging Scenarios

**Issue: Changes not reflected**
- Solution: Rebuild with `maturin develop`

**Issue: Import errors**
- Solution: Verify installation with `python -c "import arrayops"`

**Issue: Type errors**
- Solution: Check type stubs in `_arrayops.pyi`

## Profiling and Performance Analysis

### Python Profiling

```python
import cProfile
import arrayops
import array

arr = array.array('i', list(range(1_000_000)))
profiler = cProfile.Profile()
profiler.enable()
result = arrayops.sum(arr)
profiler.disable()
profiler.print_stats()
```

### Rust Profiling

Use `perf` on Linux or Instruments on macOS:

```bash
# Linux
perf record python script.py
perf report

# macOS
instruments -t "Time Profiler" python script.py
```

### Benchmarking

Create benchmark scripts:

```python
import time
import array
import arrayops

arr = array.array('i', list(range(1_000_000)))

# Benchmark
start = time.perf_counter()
result = arrayops.sum(arr)
elapsed = time.perf_counter() - start
print(f"Time: {elapsed*1000:.2f}ms")
```

## Release Process

### 1. Update Version

Update version in:
- `pyproject.toml` (dynamic version)
- `arrayops/__init__.py` (`__version__`)

### 2. Update Changelog

Add entry to `docs/CHANGELOG.md`:
- New features
- Bug fixes
- Breaking changes
- Deprecations

### 3. Run Full Test Suite

```bash
# Python tests
pytest tests/ --cov=arrayops

# Rust tests
cargo test --lib

# Code quality
ruff check .
mypy arrayops tests
cargo clippy
```

### 4. Build Distribution

```bash
# Build wheel
maturin build --release

# Verify wheel
python -m pip install dist/arrayops-*.whl --force-reinstall
```

### 5. Create Git Tag

```bash
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0
```

### 6. Publish to PyPI

```bash
maturin publish
```

## CI/CD

The project uses GitHub Actions for continuous integration:

- **Python tests**: Run on multiple Python versions
- **Rust tests**: Run on stable Rust
- **Code quality**: Run ruff, mypy, clippy
- **Coverage**: Track test coverage

See `.github/workflows/` for CI configuration.

## Common Development Tasks

### Adding a New Function

1. Add Rust implementation in `src/lib.rs`
2. Add Python wrapper in `arrayops/__init__.py`
3. Add type stubs in `arrayops/_arrayops.pyi`
4. Add tests in `tests/test_basic.py`
5. Update documentation

### Modifying Existing Function

1. Update Rust code in `src/lib.rs`
2. Rebuild: `maturin develop`
3. Update tests if behavior changes
4. Update documentation

### Debugging Build Issues

```bash
# Clean build
cargo clean
rm -rf target/
maturin develop --release

# Verbose output
maturin develop -v
```

## Platform-Specific Notes

### macOS

- May need to set `DYLD_LIBRARY_PATH` for Rust tests:
  ```bash
  export DYLD_LIBRARY_PATH=$(python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))"):$DYLD_LIBRARY_PATH
  ```

### Linux

- May need development headers:
  ```bash
  sudo apt-get install python3-dev
  ```

### Windows

- Requires Visual Studio Build Tools or MSVC
- Use PowerShell or Git Bash

## Getting Help

- Check [Troubleshooting Guide](troubleshooting.md)
- Review existing code for examples
- Open a GitHub issue
- Ask in discussions

## Related Documentation

- [Contributing Guide](contributing.md) - How to contribute
- [API Reference](api.md) - API documentation
- [Design Document](design.md) - Architecture details

