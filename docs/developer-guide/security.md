# Developer Security Guidelines

This guide provides security best practices for contributors to arrayops. All contributors should follow these guidelines when writing code, reviewing pull requests, or making changes to the codebase.

## Overview

arrayops prioritizes security through:

- Rust's memory safety guarantees
- Comprehensive input validation
- Safe error handling
- Minimal unsafe code usage
- Security-focused code review

## Secure Coding Practices

### Rust/PyO3 Security

#### 1. Input Validation

**Always validate inputs** before processing:

```rust
// ✅ Good: Validate input type and properties
pub fn my_function(py: Python, array: &PyAny) -> PyResult<PyObject> {
    let input_type = detect_input_type(array)?;
    let typecode = get_typecode_unified(array, input_type)?;
    validate_for_operation(array, input_type, false)?;
    // ... process array
}

// ❌ Bad: No validation
pub fn my_function(py: Python, array: &PyAny) -> PyResult<PyObject> {
    // Missing validation - unsafe!
    // ... process array
}
```

**Validation requirements**:
- Check input type (array.array, numpy.ndarray, memoryview, Arrow)
- Validate typecode is supported
- Check array properties (dimensionality, contiguity, writability)
- Verify buffer accessibility before use

#### 2. Buffer Safety

**Use PyO3's safe buffer APIs**:

```rust
// ✅ Good: Use PyBuffer with safe APIs
let buffer = PyBuffer::<i32>::get(array)?;
let slice = buffer.as_slice(py).ok_or_else(|| PyTypeError::new_err("..."))?;
// slice is guaranteed to be valid and bounds-checked

// ❌ Bad: Raw pointer manipulation (never do this)
// let ptr = array.as_ptr(); // Unsafe - don't do this!
```

**Buffer safety rules**:
- Always use `PyBuffer` type for buffer access
- Use `.as_slice()` for safe slice access (never use raw pointers)
- Check buffer validity with `.ok_or_else()` before use
- Never store buffer references beyond function scope (lifetime safety)

#### 3. Error Handling

**Return appropriate error types**:

```rust
// ✅ Good: Specific error types
if !is_valid {
    return Err(PyTypeError::new_err("Expected array.array"));
}
if size > MAX_SIZE {
    return Err(PyValueError::new_err("Array too large"));
}

// ❌ Bad: Generic or unsafe errors
// return Err(PyException::new_err("Error")); // Too generic
// panic!("Invalid input"); // Never panic in library code!
```

**Error handling rules**:
- Use specific error types (`PyTypeError`, `PyValueError`, `PyBufferError`)
- Never panic in library code (convert panics to Python exceptions)
- Error messages should be helpful but not leak sensitive information
- Don't expose internal implementation details in error messages

#### 4. Unsafe Code Guidelines

**Minimize unsafe code usage**:

```rust
// ✅ Good: Use safe Rust APIs
let slice = buffer.as_slice(py)?; // Safe API

// ⚠️ If unsafe is necessary, document safety invariants:
// SAFETY: This is safe because:
// 1. Pointer is valid (checked above)
// 2. Size is correct (validated)
// 3. Lifetime is managed by Rust
unsafe {
    // minimal unsafe code with safety comments
}
```

**Unsafe code rules**:
- Avoid unsafe code when possible (use safe Rust/PyO3 APIs)
- Document all unsafe blocks with safety comments explaining why it's safe
- Justify each unsafe block (why safe APIs can't be used)
- Test unsafe code thoroughly
- Review unsafe code carefully in PRs

#### 5. Integer Overflow

**Handle integer operations safely**:

```rust
// ✅ Good: Use checked arithmetic or handle overflow
let result = a.checked_add(b).ok_or_else(|| PyValueError::new_err("Overflow"))?;
// Or use Rust's default behavior (wrapping) if appropriate for the use case

// ❌ Bad: Ignore potential overflow
// let result = a + b; // May overflow silently
```

**Integer overflow rules**:
- Consider overflow behavior for arithmetic operations
- Use checked arithmetic when overflow is an error condition
- Document overflow behavior in function documentation
- Note: Python's integer semantics may differ from Rust's

### Python Security

#### 1. Callable Execution

**When executing user callables**, be aware of security implications:

```rust
// ✅ Good: Execute callable with proper error handling
let result = callback.call1(py, (element,))?;
// GIL is held (required for Python interop)
// Exceptions are propagated correctly

// ❌ Bad: No error handling or unsafe execution
// callback.call1(py, (element,)); // Ignoring errors
```

**Callable execution rules**:
- Always handle exceptions from user callables
- GIL must be held when calling Python code
- Don't assume callables are safe (users can provide any Python code)
- Document that callables execute with full Python interpreter privileges

#### 2. Error Message Security

**Error messages should not leak sensitive information**:

```rust
// ✅ Good: Generic but helpful error
return Err(PyTypeError::new_err("Expected array.array"));

// ❌ Bad: Leaks internal details
// return Err(PyTypeError::new_err(format!("Buffer pointer {:p} invalid", ptr)));
// return Err(PyTypeError::new_err(format!("Internal state: {:?}", internal_state)));
```

**Error message rules**:
- Don't expose internal implementation details
- Don't include memory addresses or pointers
- Don't include stack traces or debug information
- Keep messages helpful for users but generic enough to be safe
- Avoid exposing system-specific information

## Code Review Security Checklist

When reviewing code, check for:

### Input Validation
- [ ] All inputs are validated before use
- [ ] Type checking is performed
- [ ] Array properties are verified (size, dimensionality, contiguity)
- [ ] Edge cases are handled (empty arrays, single elements, very large arrays)

### Memory Safety
- [ ] No raw pointer manipulation
- [ ] Buffer access uses safe PyO3 APIs
- [ ] No use-after-free patterns
- [ ] Lifetime management is correct

### Error Handling
- [ ] Errors are handled appropriately
- [ ] No panics in library code
- [ ] Error messages are safe (no sensitive info)
- [ ] Appropriate error types are used

### Unsafe Code
- [ ] Unsafe code is minimized
- [ ] All unsafe blocks have safety comments
- [ ] Safety invariants are documented
- [ ] Unsafe code is justified

### Testing
- [ ] Security-sensitive code has tests
- [ ] Edge cases are tested
- [ ] Error conditions are tested
- [ ] Large input tests are included

## Testing Security Considerations

### Security-Focused Tests

When writing tests, include:

1. **Input validation tests**: Invalid inputs should be rejected
2. **Edge case tests**: Empty arrays, single elements, boundary values
3. **Large array tests**: Very large arrays (DoS considerations)
4. **Error handling tests**: Ensure errors are handled safely
5. **Type confusion tests**: Attempt to use wrong types

### Example Security Test

```python
def test_security_large_array():
    """Test that very large arrays are handled (DoS consideration)."""
    # This test verifies that large arrays don't cause crashes
    # but note: they may consume significant memory/CPU
    large_array = array.array('i', [0] * 10_000_000)
    result = arrayops.sum(large_array)
    assert result == 0

def test_security_invalid_type():
    """Test that invalid types are rejected safely."""
    with pytest.raises(TypeError, match="Expected array.array"):
        arrayops.sum([1, 2, 3])  # list, not array
```

## Dependency Security

### Adding New Dependencies

Before adding a new dependency:

1. **Check security**: Review the crate/package for known vulnerabilities
2. **Minimize dependencies**: Only add if necessary
3. **Review maintenance**: Check if the dependency is actively maintained
4. **Check license**: Ensure license is compatible
5. **Document rationale**: Explain why the dependency is needed

### Updating Dependencies

- **Regular updates**: Keep dependencies up to date
- **Security updates**: Apply security patches promptly
- **Test updates**: Test thoroughly after updating dependencies
- **Review changelogs**: Check for security-related changes

## Security Incident Response

If you discover a security vulnerability:

1. **Do NOT** open a public issue or PR
2. **Email** the maintainer: odosmatthews@gmail.com
3. **Include**:
   - Description of the vulnerability
   - Potential impact
   - Steps to reproduce
   - Suggested fix (if any)
4. **Wait** for response before public disclosure

See [SECURITY.md](../../SECURITY.md) for the full security policy.

## Security Resources

- [Rust Security Guidelines](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- [PyO3 Safety Documentation](https://pyo3.rs/latest/faq.html#why-is-my-code-giving-safety-errors)
- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [arrayops Security Documentation](../security.md)

## Code Examples

### Secure Function Template

```rust
/// Brief description of the function.
///
/// # Security Considerations
/// - Validates input type and properties
/// - Uses safe buffer APIs
/// - Handles errors appropriately
/// - No unsafe code
pub fn secure_function(py: Python, array: &PyAny) -> PyResult<PyObject> {
    // 1. Validate input
    let input_type = detect_input_type(array)?;
    let typecode = get_typecode_unified(array, input_type)?;
    validate_for_operation(array, input_type, false)?;
    
    // 2. Use safe buffer access
    dispatch_by_typecode!(typecode, array, |buffer| {
        let slice = buffer.as_slice(py)
            .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
        
        // 3. Process with safe operations
        let result = process_slice(slice)?;
        
        Ok(result.to_object(py))
    })
}

fn process_slice<T: Element>(slice: &[T]) -> PyResult<T::Output> {
    // Safe slice operations - bounds-checked by Rust
    // ...
}
```

### Security Anti-Patterns to Avoid

```rust
// ❌ DON'T: Skip validation
pub fn bad_function(py: Python, array: &PyAny) -> PyResult<PyObject> {
    // Missing validation!
    let buffer = PyBuffer::<i32>::get(array)?; // May fail unexpectedly
}

// ❌ DON'T: Use raw pointers
pub fn bad_function(py: Python, array: &PyAny) -> PyResult<PyObject> {
    let ptr = array.as_ptr(); // Unsafe!
    unsafe { /* pointer manipulation */ }
}

// ❌ DON'T: Panic in library code
pub fn bad_function(py: Python, array: &PyAny) -> PyResult<PyObject> {
    if !is_valid { panic!("Invalid!"); } // Never panic!
}

// ❌ DON'T: Leak sensitive info in errors
pub fn bad_function(py: Python, array: &PyAny) -> PyResult<PyObject> {
    return Err(PyTypeError::new_err(format!("Pointer {:p} invalid", ptr)));
}
```

## Questions?

If you have questions about security:

- Check the [Security Documentation](../security.md)
- Review existing code for examples
- Ask in PR comments or discussions
- Contact the maintainer for sensitive questions

---

_Last updated: 2024_

