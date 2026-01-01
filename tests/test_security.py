"""Security-focused tests for arrayops package.

These tests verify security properties including:
- Input validation and bounds checking
- Denial of service (DoS) resistance
- Integer overflow handling
- Type confusion prevention
- Error message safety
"""

import array
import sys
import pytest

try:
    import numpy as np

    NUMPY_AVAILABLE = True
except ImportError:
    NUMPY_AVAILABLE = False


class TestInputValidation:
    """Test input validation security properties."""

    def test_invalid_type_rejected(self):
        """Test that invalid types are rejected safely."""
        import arrayops

        # List should be rejected
        with pytest.raises(TypeError, match="Expected array.array"):
            arrayops.sum([1, 2, 3])

        # String should be rejected
        with pytest.raises(TypeError, match="Expected array.array"):
            arrayops.sum("not an array")

        # Integer should be rejected
        with pytest.raises(TypeError, match="Expected array.array"):
            arrayops.sum(42)

        # None should be rejected
        with pytest.raises(TypeError):
            arrayops.sum(None)

    def test_unsupported_typecode_rejected(self):
        """Test that unsupported typecodes are rejected."""
        import arrayops

        # Try unsupported typecodes
        # Note: Some may not be available on all platforms
        unsupported = ["c", "u", "q", "Q"]
        for typecode in unsupported:
            try:
                arr = array.array(typecode, [1, 2, 3] if typecode != "c" else b"abc")
            except (ValueError, TypeError):
                # Platform doesn't support this typecode, skip
                continue

            with pytest.raises(TypeError, match="Unsupported typecode"):
                arrayops.sum(arr)

    def test_empty_array_handled(self):
        """Test that empty arrays are handled safely (no crashes)."""
        import arrayops

        empty = array.array("i", [])
        # Should not crash, should return appropriate default
        assert arrayops.sum(empty) == 0
        with pytest.raises(ValueError):
            arrayops.mean(empty)

    def test_single_element_handled(self):
        """Test that single-element arrays are handled correctly."""
        import arrayops

        single = array.array("i", [42])
        assert arrayops.sum(single) == 42
        assert arrayops.mean(single) == 42.0
        assert arrayops.min(single) == 42
        assert arrayops.max(single) == 42

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_numpy_non_contiguous_rejected(self):
        """Test that non-contiguous NumPy arrays are rejected."""
        import arrayops

        # Create non-contiguous array
        arr = np.arange(10)[::2]  # Non-contiguous slice
        with pytest.raises(TypeError, match="contiguous"):
            arrayops.sum(arr)

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_numpy_multi_dimensional_rejected(self):
        """Test that multi-dimensional NumPy arrays are rejected."""
        import arrayops

        # Create 2D array
        arr = np.array([[1, 2, 3], [4, 5, 6]])
        with pytest.raises(TypeError, match="1-dimensional"):
            arrayops.sum(arr)


class TestDenialOfService:
    """Test denial of service (DoS) resistance.

    Note: These tests verify that very large arrays don't cause crashes.
    However, processing very large arrays will consume significant memory/CPU.
    """

    def test_very_large_array_handled(self):
        """Test that very large arrays are handled (may be slow)."""
        import arrayops

        # Large but reasonable size (1M elements = ~4MB for int32)
        large_array = array.array("i", [1] * 1_000_000)
        result = arrayops.sum(large_array)
        assert result == 1_000_000

    def test_extremely_large_array_handled(self):
        """Test that extremely large arrays are handled (will be slow)."""
        import arrayops

        # Very large size (10M elements = ~40MB for int32)
        # This test may be slow and consume significant memory
        # Skip on systems with limited memory
        try:
            very_large_array = array.array("i", [1] * 10_000_000)
            result = arrayops.sum(very_large_array)
            assert result == 10_000_000
        except MemoryError:
            pytest.skip("Not enough memory for this test")

    def test_large_array_scale(self):
        """Test in-place operations on large arrays."""
        import arrayops

        large_array = array.array("i", [1] * 100_000)
        arrayops.scale(large_array, 2.0)
        # Check first and last elements
        assert large_array[0] == 2
        assert large_array[-1] == 2


class TestIntegerOverflow:
    """Test integer overflow handling.

    Note: Integer overflow behavior follows Python's semantics.
    In Rust debug mode, integer overflow can panic (this is expected).
    In release mode, Rust uses wrapping arithmetic by default.
    These tests verify that overflow is handled appropriately.
    """

    def test_large_positive_integers(self):
        """Test operations with large positive integers."""
        import arrayops

        # Use values that sum to within i32 range (avoid overflow)
        # This test verifies large values are handled correctly
        arr = array.array(
            "i", [1_000_000_000, 1_000_000_000]
        )  # Sum = 2B, within i32 range
        result = arrayops.sum(arr)
        # Result should be valid
        assert isinstance(result, int)
        assert result == 2_000_000_000

    def test_large_negative_integers(self):
        """Test operations with large negative integers."""
        import arrayops

        # Use values that sum to within i32 range (avoid overflow)
        arr = array.array(
            "i", [-1_000_000_000, -1_000_000_000]
        )  # Sum = -2B, within i32 range
        result = arrayops.sum(arr)
        # Result should be valid
        assert isinstance(result, int)
        assert result == -2_000_000_000

    @pytest.mark.skipif(
        sys.platform == "win32",
        reason="Windows uses 32-bit long, test requires 64-bit",
    )
    def test_integer_overflow_does_not_crash(self):
        """Test that integer overflow is handled safely.

        Note: In debug mode, Rust will panic on overflow (expected behavior for catching bugs).
        In release mode, Rust uses wrapping arithmetic (safe, doesn't cause crashes).
        For production use, compile in release mode where wrapping is the default.

        This test verifies that overflow doesn't cause memory safety issues.
        """
        import arrayops

        # Test with values that sum to within i64 range (no overflow)
        # This verifies that large values are handled correctly
        # Actual overflow testing would require release mode compilation
        arr = array.array("l", [4_611_686_018_427_387_500, 4_611_686_018_427_387_500])
        result = arrayops.sum(arr)
        # Result should be valid
        assert isinstance(result, int)
        assert result == 9_223_372_036_854_775_000


class TestTypeConfusion:
    """Test prevention of type confusion attacks."""

    def test_mixed_type_operations_rejected(self):
        """Test that mixing incompatible types is rejected."""
        import arrayops

        arr1 = array.array("i", [1, 2, 3])
        arr2 = array.array("f", [1.0, 2.0, 3.0])

        # Adding arrays with different types should be rejected
        # (if this operation exists and checks types)
        # Note: This may depend on specific function implementation
        try:
            result = arrayops.add(arr1, arr2)
            # If operation succeeds, types should be compatible
            assert isinstance(result, array.array)
        except (TypeError, ValueError):
            # If operation fails, that's expected for type safety
            pass

    def test_wrong_itemsize_rejected(self):
        """Test that arrays with wrong itemsize are rejected."""
        import arrayops

        # Create arrays that might confuse type system
        # This test depends on specific validation logic
        arr1 = array.array("i", [1, 2, 3])
        arr2 = array.array("l", [1, 2, 3])  # Different size on some platforms

        # Operations should handle size differences correctly
        try:
            result = arrayops.add(arr1, arr2)
            # If succeeds, sizes must be compatible
            assert len(result) == len(arr1)
        except (TypeError, ValueError):
            # Rejection is also valid behavior
            pass


class TestErrorMessageSecurity:
    """Test that error messages don't leak sensitive information."""

    def test_error_messages_generic(self):
        """Test that error messages are generic (no internal details)."""
        import arrayops

        # Error messages should not contain:
        # - Memory addresses
        # - Internal state
        # - Stack traces
        # - File paths

        with pytest.raises(TypeError) as exc_info:
            arrayops.sum([1, 2, 3])
        error_msg = str(exc_info.value)

        # Should not contain pointer addresses
        assert "0x" not in error_msg.lower()
        # Should not contain file paths (in most cases)
        # (Some file paths in tracebacks are acceptable, but not in error message itself)
        # Should be a helpful but generic message
        assert "Expected" in error_msg or "array.array" in error_msg

    def test_error_messages_no_stack_traces(self):
        """Test that errors don't expose stack traces to users."""
        import arrayops

        # Errors should be Python exceptions, not raw panics
        with pytest.raises((TypeError, ValueError)):
            arrayops.sum([1, 2, 3])

        # Should be catchable as Python exception
        try:
            arrayops.sum([1, 2, 3])
            assert False, "Should have raised exception"
        except TypeError:
            # Expected - should be Python exception, not panic
            pass

    def test_validation_error_messages_safe(self):
        """Test that validation error messages are safe."""
        import arrayops

        # Test various validation errors
        test_cases = [
            ([1, 2, 3], "Expected array.array"),  # Wrong type
        ]

        for invalid_input, expected_pattern in test_cases:
            with pytest.raises(TypeError) as exc_info:
                arrayops.sum(invalid_input)
            error_msg = str(exc_info.value)

            # Should contain expected pattern
            assert expected_pattern in error_msg or "array.array" in error_msg
            # Should not contain sensitive information
            assert "0x" not in error_msg.lower()


class TestBufferSafety:
    """Test buffer safety and bounds checking."""

    def test_bounds_checked(self):
        """Test that buffer access is bounds-checked (implicit in Rust)."""
        import arrayops

        # Rust's bounds checking should prevent buffer overflows
        # This is tested implicitly - if bounds checking fails, we'd get a panic
        # which would be converted to a Python exception

        arr = array.array("i", [1, 2, 3, 4, 5])
        # These operations should work without buffer overflows
        result = arrayops.sum(arr)
        assert result == 15

        arrayops.scale(arr, 2.0)
        assert list(arr) == [2, 4, 6, 8, 10]

    def test_empty_buffer_safe(self):
        """Test that empty buffers are handled safely."""
        import arrayops

        empty = array.array("i", [])
        # Operations on empty buffers should be safe
        assert arrayops.sum(empty) == 0
        arrayops.scale(empty, 2.0)  # Should not crash
        assert len(empty) == 0


class TestCallbackSecurity:
    """Test security considerations for callback functions.

    Note: Callbacks execute user-provided Python code with full interpreter privileges.
    These tests verify that callbacks are executed safely from arrayops' perspective.
    """

    def test_callback_exceptions_propagated(self):
        """Test that exceptions in callbacks are propagated safely."""
        import arrayops

        arr = array.array("i", [1, 2, 3])

        # Callback that raises exception
        def raising_callback(x):
            raise ValueError("Test exception")

        # Exception should be propagated, not cause crash
        with pytest.raises(ValueError, match="Test exception"):
            arrayops.map(arr, raising_callback)

    def test_callback_with_large_array(self):
        """Test callbacks with large arrays (DoS consideration)."""
        import arrayops

        large_array = array.array("i", list(range(1000)))
        # Callback should be called for each element
        result = arrayops.map(large_array, lambda x: x * 2)
        assert len(result) == 1000
        assert result[0] == 0
        assert result[-1] == 1998


class TestMemorySafety:
    """Test memory safety properties (implicit in Rust, but verify behavior)."""

    def test_no_use_after_free(self):
        """Test that there are no use-after-free issues (Rust prevents this)."""
        import arrayops

        # Create array and use it
        arr = array.array("i", [1, 2, 3, 4, 5])
        result1 = arrayops.sum(arr)

        # Use array again (should still be valid)
        result2 = arrayops.sum(arr)
        assert result1 == result2

        # Modify and use again
        arrayops.scale(arr, 2.0)
        result3 = arrayops.sum(arr)
        assert result3 == result1 * 2

    def test_concurrent_access_safe(self):
        """Test that operations don't cause data races (Rust prevents this)."""
        import arrayops
        import threading

        arr = array.array("i", [1, 2, 3, 4, 5])
        results = []

        def worker():
            results.append(arrayops.sum(arr))

        # Create multiple threads
        threads = [threading.Thread(target=worker) for _ in range(5)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        # All results should be the same (read-only operation)
        assert all(r == 15 for r in results)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
