# Migration Guide: Upgrading to arrayops 1.0.0

This guide helps you upgrade from arrayops 0.x versions to 1.0.0.

## Overview

**Good news**: There are **no breaking changes** from 0.4.x to 1.0.0! 

All code written for arrayops 0.4.x should work unchanged with 1.0.0. This release marks the transition from alpha to production-ready status, with improved documentation, stability guarantees, and enhanced security.

## Upgrading from 0.4.x

### Installation

Simply upgrade using pip:

```bash
pip install --upgrade arrayops
```

Or if using a specific version:

```bash
pip install arrayops>=1.0.0
```

### Code Changes Required

**None!** Your existing code should work without any changes.

### What's New in 1.0.0

While there are no breaking changes, 1.0.0 includes:

1. **Stability Guarantees**: API stability is now guaranteed (see [API_STABILITY.md](docs/API_STABILITY.md))
2. **Enhanced Documentation**: Comprehensive documentation updates
3. **Security Improvements**: Enhanced security documentation and review
4. **Deprecation Tracking**: Better tracking of deprecations and migration paths

## Upgrading from 0.3.x or Earlier

If you're upgrading from 0.3.x or earlier versions, review the changelog entries for intermediate versions:

- [0.4.0 Changelog](docs/CHANGELOG.md#040---2024-01-xx) (if documented)
- [0.3.0 Changelog](docs/CHANGELOG.md#030---2024-01-xx)
- [0.2.0 Changelog](docs/CHANGELOG.md#020---2024-01-xx)
- [0.1.0 Changelog](docs/CHANGELOG.md#010---2024-01-xx)

## Breaking Changes

### None in 1.0.0

There are no breaking changes in version 1.0.0. All APIs remain stable and backward compatible with 0.4.x.

## Deprecated APIs

### Currently None

As of 1.0.0, there are no deprecated public APIs. All functions and classes are stable.

### Internal Deprecations

- **PyO3 `IntoPy` trait**: Internal use only - no impact on users
  - See [DEPRECATION_NOTES.md](docs/DEPRECATION_NOTES.md) for details
  - Migration planned for future version (1.1.0+)

## API Changes

### Function Signatures

All function signatures remain unchanged. No parameters were added, removed, or modified.

### Behavior Changes

No intentional behavior changes. If you notice any behavioral differences, please report them as bugs.

### Error Messages

Error messages may be improved (more descriptive), but error types and conditions remain the same.

## Migration Examples

### Example 1: Basic Usage (No Changes)

```python
# Works in 0.4.x and 1.0.0 - no changes needed
import array
import arrayops

arr = array.array('i', [1, 2, 3, 4, 5])
result = arrayops.sum(arr)  # Still works!
arrayops.scale(arr, 2.0)     # Still works!
```

### Example 2: Advanced Operations (No Changes)

```python
# Works in 0.4.x and 1.0.0 - no changes needed
import array
import arrayops

arr = array.array('f', [1.0, 2.0, 3.0, 4.0, 5.0])
mean_val = arrayops.mean(arr)        # Still works!
std_val = arrayops.std(arr)          # Still works!
arrayops.normalize(arr)               # Still works!
```

### Example 3: NumPy Interop (No Changes)

```python
# Works in 0.4.x and 1.0.0 - no changes needed
import numpy as np
import arrayops

arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
result = arrayops.sum(arr)  # Still works!
```

## Testing Your Migration

After upgrading, run your test suite to ensure everything works:

```bash
# Run your tests
pytest your_tests/

# Or run arrayops tests to verify installation
pip install pytest
pytest tests/
```

## Troubleshooting

### Import Errors

If you see import errors after upgrading:

1. **Verify installation**:
   ```bash
   pip show arrayops
   ```

2. **Reinstall if needed**:
   ```bash
   pip uninstall arrayops
   pip install arrayops
   ```

3. **Check Python version**: arrayops 1.0.0 requires Python 3.8+
   ```bash
   python --version
   ```

### Performance Issues

If you notice performance changes:

1. **Check your Python/Rust versions**: Ensure compatible versions
2. **Verify feature flags**: If using parallel features, ensure they're enabled
3. **Report issues**: Performance regressions are bugs - please report them

### Type Checking Issues

If mypy or other type checkers report issues:

1. **Update type stubs**: Type stubs are included in the package
2. **Clear cache**: Clear mypy cache if needed
3. **Report issues**: Type checker issues are bugs - please report them

## Getting Help

If you encounter issues during migration:

1. **Check Documentation**:
   - [API Reference](docs/api.md)
   - [API Stability](docs/API_STABILITY.md)
   - [Changelog](docs/CHANGELOG.md)

2. **Report Issues**:
   - GitHub Issues: https://github.com/eddiethedean/arrayops/issues
   - Include version information and error messages

3. **Ask Questions**:
   - GitHub Discussions: https://github.com/eddiethedean/arrayops/discussions

## Future Compatibility

### Version 1.1.0+

- New features may be added
- Performance improvements may occur
- Documentation may be enhanced
- **No breaking changes** expected

### Version 2.0.0+

- Breaking changes would require a major version bump
- Migration guide will be provided
- Deprecation warnings will be issued in advance

## Summary

✅ **Upgrading to 1.0.0 is straightforward**:
1. Run `pip install --upgrade arrayops`
2. Run your tests
3. Enjoy improved stability and documentation!

No code changes are required, and all existing code should work without modification.

