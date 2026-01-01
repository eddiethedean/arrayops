# Troubleshooting Guide

Common issues and solutions for `arrayops`.

## Installation Issues

### Issue: Module not found after installation

**Error:**
```
ImportError: No module named 'ao._arrayops'
```

**Solutions:**
1. **Rebuild the extension:**
   ```bash
   maturin develop
   ```

2. **Verify installation:**
   ```bash
   python -c "import arrayops; print(ao.__version__)"
   ```

3. **Check Python version:**
   ```bash
   python --version  # Should be 3.8+
   ```

4. **Reinstall from source:**
   ```bash
   pip uninstall arrayops
   pip install -e .
   ```

### Issue: Build fails with Rust errors

**Error:**
```
error: failed to compile `arrayops`
```

**Solutions:**
1. **Update Rust:**
   ```bash
   rustup update
   rustc --version  # Should be 1.75+
   ```

2. **Clean and rebuild:**
   ```bash
   cargo clean
   maturin develop
   ```

3. **Check Rust toolchain:**
   ```bash
   rustup show
   ```

### Issue: maturin not found

**Error:**
```
command not found: maturin
```

**Solutions:**
1. **Install maturin:**
   ```bash
   pip install maturin
   ```

2. **Verify installation:**
   ```bash
   maturin --version
   ```

## Runtime Errors

### Issue: TypeError for unsupported typecode

**Error:**
```
TypeError: Unsupported typecode: 'c'
```

**Solution:**
Only numeric types are supported. Use one of:
- `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L` (integers)
- `f`, `d` (floats)

```python
# Wrong
arr = array.array('c', b'abc')  # Character array

# Correct
arr = array.array('i', [1, 2, 3])  # Integer array
```

### Issue: TypeError for non-array input

**Error:**
```
TypeError: Expected array.array, got different type
```

**Solution:**
Ensure you're passing an `array.array` instance:

```python
# Wrong
ao.sum([1, 2, 3])  # Python list

# Correct
import array
arr = array.array('i', [1, 2, 3])
ao.sum(arr)
```

### Issue: Results don't match Python's sum

**Possible causes:**
1. **Integer overflow**: Python handles overflow differently
2. **Float precision**: Floating point operations may have slight differences
3. **Empty arrays**: Both return 0, but type may differ

**Solution:**
For integer arrays, Python may promote to larger types on overflow. `arrayops` follows the array's type:

```python
import array
import arrayops as ao

# Large values may overflow in smaller types
arr = array.array('b', [127, 1])  # int8, max value is 127
# Sum will wrap around: 127 + 1 = -128 (two's complement)
```

## Build Issues

### Issue: Python development headers not found

**Error (Linux):**
```
fatal error: Python.h: No such file or directory
```

**Solution:**
Install Python development headers:

```bash
# Ubuntu/Debian
sudo apt-get install python3-dev

# Fedora/RHEL
sudo dnf install python3-devel

# macOS (usually included with Python)
# Windows: Usually included with Python installer
```

### Issue: Linker errors on macOS

**Error:**
```
ld: library not found for -lpython3.x
```

**Solution:**
Set library path:

```bash
export DYLD_LIBRARY_PATH=$(python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))"):$DYLD_LIBRARY_PATH
maturin develop
```

### Issue: Windows build fails

**Error:**
```
error: linker `link.exe` not found
```

**Solution:**
Install Visual Studio Build Tools:
1. Download from [Microsoft](https://visualstudio.microsoft.com/downloads/)
2. Install "Desktop development with C++" workload
3. Rebuild: `maturin develop`

## Performance Issues

### Issue: Slower than expected

**Possible causes:**
1. **Development build**: Using debug build instead of release
2. **Small arrays**: Overhead may dominate for very small arrays
3. **Type conversions**: Unnecessary conversions add overhead

**Solutions:**
1. **Use release build:**
   ```bash
   maturin develop --release
   ```

2. **Check array size:**
   - For arrays < 100 elements, Python overhead may be negligible
   - `arrayops` shines with larger arrays (1000+ elements)

3. **Avoid conversions:**
   ```python
   # Slow: Converting to list
   arr = array.array('i', list(range(1000)))
   
   # Fast: Direct creation
   arr = array.array('i', range(1000))
   ```

### Issue: High memory usage

**Possible causes:**
1. **Multiple copies**: Creating unnecessary array copies
2. **Large arrays**: Very large arrays consume significant memory

**Solutions:**
1. **Use in-place operations:**
   ```python
   # Good: In-place
   ao.scale(arr, 2.0)
   
   # Avoid: Creating copies when possible
   ```

2. **Process in batches:**
   ```python
   # Process large files in chunks
   batch_size = 10000
   # ... batch processing code
   ```

## Platform-Specific Issues

### macOS

**Issue: Rust tests fail**

**Solution:**
Set library path:
```bash
export DYLD_LIBRARY_PATH=$(python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))"):$DYLD_LIBRARY_PATH
cargo test --lib
```

### Linux

**Issue: Permission denied errors**

**Solution:**
Install to user directory or use virtual environment:
```bash
pip install --user maturin
# or
python -m venv venv
source venv/bin/activate
```

### Windows

**Issue: Path issues with spaces**

**Solution:**
Use short paths or quotes:
```bash
cd "C:\Program Files\Python"
# or use short path names
```

## Debugging Tips

### Enable Verbose Output

```bash
# Verbose maturin output
maturin develop -v

# Verbose cargo output
cargo build --verbose
```

### Check Installation

```python
import arrayops as ao
print(ao.__version__)
print(ao.__file__)  # Location of package
```

### Test Import

```python
# Test basic import
import arrayops as ao

# Test function import
from arrayops import sum, scale

# Test with array
import array
arr = array.array('i', [1, 2, 3])
result = ao.sum(arr)
print(result)  # Should print: 6
```

### Verify Rust Extension

```bash
# Check if extension module exists
python -c "import ao._arrayops; print('OK')"
```

## Getting Help

### Before Asking for Help

1. **Check this guide**: Your issue may be covered here
2. **Search existing issues**: Check GitHub issues for similar problems
3. **Verify versions**: Check Python, Rust, and maturin versions
4. **Try clean rebuild**: `cargo clean && maturin develop`

### When Reporting Issues

Include:
- Python version: `python --version`
- Rust version: `rustc --version`
- maturin version: `maturin --version`
- Operating system and version
- Full error message and traceback
- Steps to reproduce
- Expected vs. actual behavior

### Resources

- **GitHub Issues**: [Report bugs](https://github.com/your-username/arrayops/issues)
- **GitHub Discussions**: [Ask questions](https://github.com/your-username/arrayops/discussions)
- **Documentation**: See [docs/README.md](../README.md)

## Common Patterns

### Pattern: Rebuilding after changes

```bash
# After changing Rust code
maturin develop

# After changing Python code
# No rebuild needed (editable install)
```

### Pattern: Testing changes

```bash
# Run tests after changes
pytest tests/ -v

# Run specific test
pytest tests/test_basic.py::TestSum::test_sum_int32 -v
```

### Pattern: Clean rebuild

```bash
# When things go wrong
cargo clean
rm -rf target/
maturin develop
```

## Related Documentation

- [Development Guide](../development.md) - Development setup
- [API Reference](api.md) - API documentation
- [Examples](examples.md) - Usage examples

