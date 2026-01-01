# Deprecation Notes

This document tracks deprecation warnings and migration plans for arrayops.

## Current Deprecations

### PyO3 `IntoPy` Trait (Deprecated in PyO3 0.23+)

**Status**: Documented, migration planned for future version

**Issue**: PyO3 has deprecated the `IntoPy` trait in favor of `IntoPyObject`. 

**Affected Files**:
- `src/buffer.rs` (2 occurrences)
- `src/operations/basic.rs` (1 occurrence)
- `src/operations/elementwise.rs` (2 occurrences)
- `src/operations/manipulation.rs` (2 occurrences)
- `src/operations/transform.rs` (1 occurrence)

**Current Handling**:
- Deprecation warnings are suppressed in CI with `-A deprecated` flag in clippy
- All code continues to function correctly with PyO3 0.24
- No breaking changes to the public API

**Migration Plan**:
1. **Target Version**: 1.1.0 or later (post-1.0.0 release)
2. **Migration Steps**:
   - Replace `IntoPy<PyObject>` with `IntoPyObject` in trait bounds
   - Test thoroughly to ensure compatibility
   - Update PyO3 dependency if needed
3. **Rationale for Delay**:
   - Changing trait bounds requires careful testing
   - PyO3 0.24 still supports `IntoPy` (backward compatible)
   - Priority is stability for 1.0.0 release
   - Migration can be done in a minor version update

**References**:
- PyO3 Migration Guide: https://pyo3.rs/v0.23.0/migration
- PyO3 Issue: See PyO3 changelog for deprecation details

## Future Considerations

### Rust Edition Updates
- Currently using Rust 2021 edition
- Monitor Rust 2024 edition when available

### Dependency Updates
- Monitor PyO3 releases for new deprecations
- Monitor maturin for compatibility updates
- Regular dependency audits via `cargo audit`

## Notes

- All deprecations are non-breaking for end users
- Internal API changes only
- Migration will be tested thoroughly before implementation
- Breaking changes (if any) will follow semantic versioning

