# Security Documentation

This document describes the security guarantees, threat model, and security considerations for the `arrayops` library.

## Security Guarantees

### Memory Safety

arrayops is built with Rust, which provides compile-time memory safety guarantees:

- **No buffer overflows**: All buffer access is bounds-checked through Rust's safe APIs
- **No use-after-free**: Rust's ownership system prevents dangling pointers
- **No data races**: Rust's type system prevents data races in concurrent code
- **Type safety**: Rust enforces type safety at compile time

### Buffer Protocol Safety

arrayops uses PyO3's buffer protocol for zero-copy access to Python arrays:

- **Safe buffer access**: PyO3's `PyBuffer` type ensures lifetime safety and bounds checking
- **No raw pointer manipulation**: We use safe Rust APIs provided by PyO3
- **Proper error handling**: All buffer operations return `PyResult` and handle errors gracefully

### Unsafe Code Usage

arrayops uses minimal unsafe code:

1. **Allocator module** (`src/allocator.rs`): 
   - Currently **unused** (dead code marked with `#[allow(dead_code)]`)
   - Uses `std::alloc::System` which is Rust's standard, safe allocator
   - All unsafe blocks are properly documented
   - If used in the future, will follow Rust's unsafe code guidelines

All unsafe code blocks are:
- Clearly marked with safety comments explaining invariants
- Minimal and well-scoped
- Using Rust's standard library APIs (not custom unsafe code)
- Protected by Rust's memory safety guarantees

## Threat Model

### In Scope

arrayops is designed to handle:

- **Untrusted input arrays**: Input validation ensures only supported types are processed
- **Malformed arrays**: Type checking and validation prevent processing invalid data
- **Large arrays**: Operations handle arrays of various sizes (with DoS considerations noted below)
- **Edge cases**: Empty arrays, single-element arrays, and boundary conditions are handled safely

### Out of Scope

The following are considered out of scope for arrayops security guarantees:

- **Code execution vulnerabilities**: arrayops does not execute user code beyond calling Python callables (map, filter, reduce callbacks)
- **Network security**: arrayops does not perform network operations
- **File system security**: arrayops does not access the file system
- **Cryptographic security**: arrayops is not a cryptography library
- **Supply chain attacks**: Users should use standard dependency management practices (see Dependency Security below)

### Attack Surface

The main attack surface for arrayops includes:

1. **Input validation**: Malformed or maliciously crafted input arrays
2. **Integer overflow/underflow**: In arithmetic operations
3. **Denial of service**: Extremely large arrays causing resource exhaustion
4. **Type confusion**: Attempting to mix incompatible array types
5. **Callback execution**: User-provided Python callables in map/filter/reduce operations

## Security Considerations

### Input Validation

arrayops performs comprehensive input validation:

- **Type checking**: All inputs are validated to ensure they are supported types (`array.array`, `numpy.ndarray`, `memoryview`, or Arrow buffers)
- **Typecode validation**: Only numeric types are supported (no object arrays)
- **Dimensionality checking**: NumPy arrays must be 1-dimensional and contiguous
- **Buffer validation**: Buffer properties (writability, contiguity) are checked before operations

**Security Impact**: Invalid inputs are rejected with appropriate error messages that do not leak sensitive information.

### Memory Safety

Rust's memory safety guarantees protect against:

- **Buffer overflows**: All array access is bounds-checked
- **Use-after-free**: Rust's ownership system prevents dangling references
- **Double-free**: Automatic memory management prevents double-free errors
- **Uninitialized memory**: Rust prevents use of uninitialized memory

### Denial of Service (DoS) Considerations

arrayops operations can process very large arrays, which may consume significant memory and CPU:

- **Memory exhaustion**: Very large arrays will consume memory proportional to array size
- **CPU exhaustion**: Operations on very large arrays may take significant time
- **No built-in limits**: arrayops does not impose arbitrary size limits (Python's memory limits apply)

**Mitigation**: 

- Users should validate array sizes before processing if memory/CPU constraints are a concern
- For very large arrays, consider processing in chunks or using streaming approaches
- Parallel features (when enabled) may increase CPU usage on multi-core systems

**Recommendation**: In environments where DoS is a concern, implement size limits in application code before calling arrayops functions.

### Integer Overflow and Underflow

Integer operations follow Python's semantics:

- **Overflow behavior**: Integer overflow follows Python's behavior (promotion to larger types or wrapping depending on Python version)
- **Underflow behavior**: Integer underflow follows Python's behavior
- **Float precision**: Floating-point operations follow IEEE 754 semantics

**Security Impact**: Integer overflow/underflow does not cause memory safety issues (Rust's type system prevents this), but may cause incorrect results for very large values.

### Error Message Security

Error messages are designed to be informative without leaking sensitive information:

- **No internal details**: Error messages do not expose internal implementation details
- **No stack traces**: Rust panics are caught and converted to Python exceptions without exposing stack traces
- **Generic messages**: Error messages are generic enough to be helpful without revealing system details

**Example of secure error messages**:
- ✅ `"Expected array.array, numpy.ndarray, or memoryview"` (safe)
- ❌ `"Buffer pointer 0x7f8b4c001000 is invalid"` (would be unsafe - not used)

### Callback Security

Functions that accept Python callables (`map`, `filter`, `reduce`) execute user-provided code:

- **GIL safety**: Python callbacks are executed with the GIL held (required for Python interop)
- **Exception propagation**: Exceptions raised in callbacks are propagated to the caller
- **No sandboxing**: Callbacks run with full Python interpreter privileges

**Security Impact**: Users should only call arrayops with trusted callables. Do not use arrayops with untrusted code.

### Buffer Protocol Security

The buffer protocol provides zero-copy access, which is safe when used correctly:

- **Lifetime management**: PyO3 ensures buffers remain valid during use
- **Bounds checking**: All buffer access is bounds-checked
- **Writability checks**: Writable buffers are validated before in-place operations

## Secure Usage Patterns

### Recommended Practices

1. **Validate inputs**: Ensure inputs are of expected types and sizes before calling arrayops functions
2. **Handle errors**: Always handle exceptions raised by arrayops functions
3. **Size limits**: Implement size limits for untrusted input if DoS is a concern
4. **Trust callbacks**: Only use trusted Python callables with map/filter/reduce operations
5. **Keep updated**: Keep arrayops and dependencies up to date for security fixes

### Example: Secure Input Handling

```python
import array
import arrayops as ao

def safe_sum(arr, max_size=10_000_000):
    """Safely compute sum with size limit."""
    if not isinstance(arr, (array.array,)):
        raise TypeError("Expected array.array")
    
    if len(arr) > max_size:
        raise ValueError(f"Array too large (max {max_size} elements)")
    
    try:
        return ao.sum(arr)
    except Exception as e:
        # Log error without exposing internal details
        logger.error(f"Sum operation failed: {type(e).__name__}")
        raise
```

### Example: Secure Callback Usage

```python
import array
import arrayops as ao

# Safe: trusted callback
def trusted_transform(x):
    return x * 2

arr = array.array('i', [1, 2, 3])
result = ao.map(arr, trusted_transform)

# Unsafe: untrusted callback - DO NOT DO THIS
# untrusted_code = eval(user_input)  # Dangerous!
# result = ao.map(arr, untrusted_code)
```

## Dependency Security

### Rust Dependencies

arrayops uses minimal dependencies:

- **pyo3**: Rust bindings for Python (maintained by PyO3 team)
- **rayon**: Parallel processing (optional, feature-gated)

**Security Practices**:

- Dependencies are regularly audited using `cargo-audit`
- CI/CD includes automated vulnerability scanning
- Security updates are applied promptly

**Known Security Issues**:

- **RUSTSEC-2025-0020** (pyo3 0.20.3): Risk of buffer overflow in `PyString::from_object`
  - **Status**: Requires upgrade to pyo3 >=0.24.1
  - **Impact**: Low (arrayops does not use `PyString::from_object` directly)
  - **Action**: Upgrade pyo3 dependency in future release (major version upgrade requires testing)
  - **Mitigation**: Current code does not use the affected function

### Python Dependencies

arrayops has minimal Python dependencies:

- **maturin**: Build tool (dev dependency only)
- **pytest**: Testing (dev dependency only)
- **Optional**: numpy, pyarrow (for extended functionality, not required)

**Security Practices**:

- Python dependencies are scanned with `pip-audit`
- Dev dependencies are kept up to date
- Users should keep Python dependencies up to date

### Updating Dependencies

To check for security vulnerabilities:

```bash
# Rust dependencies
cargo audit

# Python dependencies
pip-audit
```

## Reporting Security Issues

If you discover a security vulnerability, please report it responsibly:

1. **Do NOT** open a public GitHub issue
2. Email the maintainer: odosmatthews@gmail.com
3. Include details about the vulnerability, impact, and steps to reproduce
4. Allow time for the maintainer to address the issue before public disclosure

See [SECURITY.md](https://github.com/eddiethedean/arrayops/blob/main/SECURITY.md) for full details on the security reporting process.

## Security Updates

Security updates are released as:

- **Patch releases**: For vulnerabilities in supported versions
- **Security advisories**: Published on GitHub
- **Release notes**: Mention security fixes (after disclosure period)

Stay informed about security updates by:

- Watching the repository on GitHub
- Checking release notes
- Subscribing to security announcements

## Best Practices for Users

1. **Keep updated**: Regularly update arrayops to receive security fixes
2. **Validate inputs**: Validate array types and sizes before processing
3. **Handle errors**: Always handle exceptions properly
4. **Limit size**: Implement size limits for untrusted input
5. **Trust callbacks**: Only use trusted code in callbacks
6. **Review dependencies**: Keep all dependencies (Rust and Python) up to date
7. **Follow principle of least privilege**: Run code with minimal necessary permissions

## Common Security Pitfalls to Avoid

1. **Processing untrusted arrays without validation**: Always validate array types and sizes
2. **Using untrusted callbacks**: Never execute untrusted code in map/filter/reduce
3. **Ignoring exceptions**: Always handle errors from arrayops functions
4. **Assuming size limits**: Don't assume arrayops will reject large arrays (implement your own limits)
5. **Exposing error details**: Don't expose detailed error messages to untrusted users
6. **Outdated dependencies**: Keep dependencies up to date to receive security fixes

## Additional Resources

- [SECURITY.md](https://github.com/eddiethedean/arrayops/blob/main/SECURITY.md): Security policy and reporting process
- [Developer Security Guide](developer-guide/security.md): Security guidelines for contributors
- [Design Document](design.md#7-safety-guarantees): Architecture and safety guarantees
- [PyO3 Documentation](https://pyo3.rs/): Rust-Python interop security considerations

---

_Last updated: 2024_

