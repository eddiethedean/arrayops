"""Basic tests for arrayops package."""

import array
import pytest
import sys


@pytest.fixture
def int_array():
    """Create a test integer array."""
    return array.array("i", [1, 2, 3, 4, 5])


@pytest.fixture
def float_array():
    """Create a test float array."""
    return array.array("d", [1.5, 2.5, 3.5, 4.5])


class TestSum:
    """Tests for sum operation."""

    def test_sum_int32(self, int_array):
        """Test sum of int32 array."""
        import arrayops

        result = arrayops.sum(int_array)
        assert result == 15
        assert isinstance(result, int)

    def test_sum_float64(self, float_array):
        """Test sum of float64 array."""
        import arrayops

        result = arrayops.sum(float_array)
        assert result == 12.0

    def test_sum_empty(self):
        """Test sum of empty array."""
        import arrayops

        arr = array.array("i", [])
        result = arrayops.sum(arr)
        assert result == 0

    def test_sum_single_element(self):
        """Test sum of single element array."""
        import arrayops

        arr = array.array("i", [42])
        result = arrayops.sum(arr)
        assert result == 42

    def test_sum_all_numeric_types(self):
        """Test sum works with all supported numeric types."""
        import arrayops

        # Test all supported integer types
        test_cases = [
            ("b", [-1, 0, 1], 0),  # signed char
            ("B", [1, 2, 3], 6),  # unsigned char
            ("h", [-100, 0, 100], 0),  # signed short
            ("H", [100, 200], 300),  # unsigned short
            ("i", [1, 2, 3], 6),  # signed int
            ("I", [1, 2, 3], 6),  # unsigned int
            ("l", [1000, 2000], 3000),  # signed long
            ("L", [1000, 2000], 3000),  # unsigned long
            ("f", [1.5, 2.5], 4.0),  # float
            ("d", [1.5, 2.5], 4.0),  # double
        ]

        for typecode, values, expected in test_cases:
            arr = array.array(typecode, values)
            result = arrayops.sum(arr)
            assert result == expected, f"Failed for type {typecode}"

    def test_sum_large_array(self):
        """Test sum with large array."""
        import arrayops

        arr = array.array("i", list(range(10000)))
        result = arrayops.sum(arr)
        expected = sum(range(10000))
        assert result == expected

    def test_sum_not_array(self):
        """Test sum raises error for non-array input."""
        import arrayops

        with pytest.raises(TypeError, match="Expected array.array"):
            arrayops.sum([1, 2, 3])

    def test_sum_unsupported_type(self):
        """Test sum raises error for unsupported typecode."""
        import arrayops

        # Try to use an invalid typecode - Python will reject it first
        # For testing, we'll create a valid array but test our validation
        # by creating an array with 'q' (which we don't support)
        try:
            arr = array.array("q", [1, 2, 3])
        except ValueError:
            # 'q' not available on this platform, skip test
            pytest.skip("Platform doesn't support 'q' typecode")
        with pytest.raises(TypeError, match="Unsupported typecode"):
            arrayops.sum(arr)


class TestScale:
    """Tests for scale operation."""

    def test_scale_int32(self, int_array):
        """Test scaling int32 array."""
        import arrayops

        arrayops.scale(int_array, 2.0)
        assert list(int_array) == [2, 4, 6, 8, 10]

    def test_scale_float64(self, float_array):
        """Test scaling float64 array."""
        import arrayops

        arrayops.scale(float_array, 2.0)
        assert list(float_array) == [3.0, 5.0, 7.0, 9.0]

    def test_scale_empty(self):
        """Test scaling empty array."""
        import arrayops

        arr = array.array("i", [])
        arrayops.scale(arr, 5.0)
        assert len(arr) == 0

    def test_scale_zero(self, int_array):
        """Test scaling by zero."""
        import arrayops

        arrayops.scale(int_array, 0.0)
        assert all(x == 0 for x in int_array)

    def test_scale_negative(self):
        """Test scaling by negative factor."""
        import arrayops

        arr = array.array("i", [1, 2, 3])
        arrayops.scale(arr, -1.0)
        assert list(arr) == [-1, -2, -3]

    def test_scale_all_numeric_types(self):
        """Test scale works with all supported numeric types."""
        import arrayops

        test_cases = [
            ("i", [1, 2, 3], 2.0, [2, 4, 6]),
            ("f", [1.0, 2.0], 1.5, [1.5, 3.0]),
            ("d", [1.0, 2.0], 2.5, [2.5, 5.0]),
        ]

        for typecode, initial, factor, expected in test_cases:
            arr = array.array(typecode, initial)
            arrayops.scale(arr, factor)
            result = list(arr)
            for i, (r, e) in enumerate(zip(result, expected)):
                assert abs(r - e) < 1e-6, f"Failed for type {typecode} at index {i}"

    def test_scale_not_array(self):
        """Test scale raises error for non-array input."""
        import arrayops

        with pytest.raises(TypeError, match="Expected array.array"):
            arrayops.scale([1, 2, 3], 2.0)

    def test_scale_unsupported_type(self):
        """Test scale raises error for unsupported typecode."""
        import arrayops

        # Try to use an invalid typecode - Python will reject it first
        # For testing, we'll create a valid array but test our validation
        # by creating an array with 'q' (which we don't support)
        try:
            arr = array.array("q", [1, 2, 3])
        except ValueError:
            # 'q' not available on this platform, skip test
            pytest.skip("Platform doesn't support 'q' typecode")
        with pytest.raises(TypeError, match="Unsupported typecode"):
            arrayops.scale(arr, 2.0)


class TestParity:
    """Tests comparing arrayops results to Python equivalents."""

    def test_sum_parity(self):
        """Test sum results match Python built-in sum."""
        import arrayops

        test_data = [1, 2, 3, 4, 5, 10, 20, 30]
        arr = array.array("i", test_data)
        python_sum = sum(test_data)
        arrayops_sum = arrayops.sum(arr)
        assert arrayops_sum == python_sum

    def test_sum_parity_float(self):
        """Test float sum results match Python built-in sum."""
        import arrayops

        test_data = [1.5, 2.5, 3.5, 4.5]
        arr = array.array("d", test_data)
        python_sum = sum(test_data)
        arrayops_sum = arrayops.sum(arr)
        assert abs(arrayops_sum - python_sum) < 1e-10


class TestModuleInitialization:
    """Tests for module initialization and error handling."""

    def test_module_imports_successfully(self):
        """Test that module can be imported when extension is available."""
        import arrayops

        assert hasattr(arrayops, "sum")
        assert hasattr(arrayops, "scale")
        assert hasattr(arrayops, "map")
        assert hasattr(arrayops, "map_inplace")
        assert hasattr(arrayops, "filter")
        assert hasattr(arrayops, "reduce")
        assert "sum" in arrayops.__all__
        assert "scale" in arrayops.__all__
        assert "map" in arrayops.__all__
        assert "map_inplace" in arrayops.__all__
        assert "filter" in arrayops.__all__
        assert "reduce" in arrayops.__all__
        assert arrayops.__version__ == "0.2.0"

    def test_module_import_error_with_arrayops_in_message(self):
        """Test helpful error message when _arrayops module is missing."""
        # Save original state
        original_module = sys.modules.pop("arrayops", None)
        original_arrayops = sys.modules.pop("arrayops._arrayops", None)

        try:
            # Read and execute the __init__.py file directly to trigger error path
            import os

            init_path = os.path.join(
                os.path.dirname(os.path.dirname(__file__)), "arrayops", "__init__.py"
            )

            # Load and execute the module source with a mocked import
            # We need to actually execute it with the import failing

            # Read the source
            with open(init_path, "r") as f:
                source = f.read()

            # Create execution namespace with mocked import
            import builtins

            original_import = builtins.__import__

            def mock_import(name, globals=None, locals=None, fromlist=(), level=0):
                if name == "arrayops._arrayops" or (
                    fromlist and "arrayops._arrayops" in str(fromlist)
                ):
                    raise ImportError("No module named 'arrayops._arrayops'")
                return original_import(name, globals, locals, fromlist, level)

            builtins.__import__ = mock_import

            try:
                # Execute in a fresh namespace that coverage can track
                exec_globals = {
                    "__file__": init_path,
                    "__name__": "arrayops",
                    "__package__": "arrayops",
                    "__version__": "0.1.0",
                    "__builtins__": {**vars(builtins), "__import__": mock_import},
                }

                with pytest.raises(ImportError) as exc_info:
                    exec(compile(source, init_path, "exec"), exec_globals)

                assert "arrayops extension module not found" in str(exc_info.value)
                assert "maturin develop" in str(exc_info.value) or "pip install" in str(
                    exc_info.value
                )
            finally:
                builtins.__import__ = original_import
        finally:
            # Restore modules
            if "arrayops" in sys.modules:
                del sys.modules["arrayops"]
            if "arrayops_test" in sys.modules:
                del sys.modules["arrayops_test"]
            if original_module:
                sys.modules["arrayops"] = original_module
            if original_arrayops:
                sys.modules["arrayops._arrayops"] = original_arrayops

    def test_module_import_error_other_error(self):
        """Test that other ImportErrors are re-raised unchanged."""
        # Save original state
        original_module = sys.modules.pop("arrayops", None)
        original_arrayops = sys.modules.pop("arrayops._arrayops", None)

        try:
            # Read and execute the __init__.py file with a different ImportError
            import os
            import builtins

            init_path = os.path.join(
                os.path.dirname(os.path.dirname(__file__)), "arrayops", "__init__.py"
            )

            # Read the source
            with open(init_path, "r") as f:
                source = f.read()

            # Create execution namespace with mocked import that raises different error
            original_import = builtins.__import__

            def mock_import_other(
                name, globals=None, locals=None, fromlist=(), level=0
            ):
                if name == "arrayops._arrayops" or (
                    fromlist and "arrayops._arrayops" in str(fromlist)
                ):
                    raise ImportError("No module named 'some_other_module'")
                return original_import(name, globals, locals, fromlist, level)

            builtins.__import__ = mock_import_other

            try:
                # Execute in a fresh namespace that coverage can track
                exec_globals = {
                    "__file__": init_path,
                    "__name__": "arrayops",
                    "__package__": "arrayops",
                    "__version__": "0.1.0",
                    "__builtins__": {**vars(builtins), "__import__": mock_import_other},
                }

                with pytest.raises(ImportError) as exc_info:
                    exec(compile(source, init_path, "exec"), exec_globals)

                # Should re-raise the original error, not wrap it (line 21: bare raise)
                assert "some_other_module" in str(exc_info.value)
                assert "arrayops extension module not found" not in str(exc_info.value)
            finally:
                builtins.__import__ = original_import
        finally:
            # Restore modules
            if "arrayops" in sys.modules:
                del sys.modules["arrayops"]
            if original_module:
                sys.modules["arrayops"] = original_module
            if original_arrayops:
                sys.modules["arrayops._arrayops"] = original_arrayops


class TestMap:
    """Tests for map operation."""

    def test_map_int32_square(self, int_array):
        """Test map with square function."""
        import arrayops

        result = arrayops.map(int_array, lambda x: x * x)
        assert list(result) == [1, 4, 9, 16, 25]
        assert result.typecode == int_array.typecode

    def test_map_float64(self, float_array):
        """Test map with float array."""
        import arrayops

        result = arrayops.map(float_array, lambda x: x * 2.0)
        assert list(result) == [3.0, 5.0, 7.0, 9.0]
        assert result.typecode == float_array.typecode

    def test_map_empty(self):
        """Test map with empty array."""
        import arrayops

        arr = array.array("i", [])
        result = arrayops.map(arr, lambda x: x * 2)
        assert len(result) == 0
        assert result.typecode == arr.typecode

    def test_map_preserves_type(self):
        """Test that map preserves input array type."""
        import arrayops

        test_cases = [
            ("i", [1, 2, 3], lambda x: x + 1, [2, 3, 4]),
            ("f", [1.0, 2.0], lambda x: x * 2.0, [2.0, 4.0]),
            ("d", [1.5, 2.5], lambda x: x - 0.5, [1.0, 2.0]),
        ]

        for typecode, initial, fn, expected in test_cases:
            arr = array.array(typecode, initial)
            result = arrayops.map(arr, fn)
            assert list(result) == expected
            assert result.typecode == typecode

    def test_map_with_function(self):
        """Test map with a named function."""
        import arrayops

        def double(x):
            return x * 2

        arr = array.array("i", [1, 2, 3])
        result = arrayops.map(arr, double)
        assert list(result) == [2, 4, 6]


class TestMapInplace:
    """Tests for map_inplace operation."""

    def test_map_inplace_int32(self):
        """Test map_inplace modifies array in-place."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        arrayops.map_inplace(arr, lambda x: x * 2)
        assert list(arr) == [2, 4, 6, 8, 10]

    def test_map_inplace_float64(self, float_array):
        """Test map_inplace with float array."""
        import arrayops

        arrayops.map_inplace(float_array, lambda x: x * 2.0)
        assert list(float_array) == [3.0, 5.0, 7.0, 9.0]

    def test_map_inplace_empty(self):
        """Test map_inplace with empty array."""
        import arrayops

        arr = array.array("i", [])
        arrayops.map_inplace(arr, lambda x: x * 2)
        assert len(arr) == 0

    def test_map_inplace_all_types(self):
        """Test map_inplace works with all numeric types."""
        import arrayops

        test_cases = [
            ("i", [1, 2, 3], lambda x: x + 1, [2, 3, 4]),
            ("f", [1.0, 2.0], lambda x: x * 2.0, [2.0, 4.0]),
            ("d", [1.5, 2.5], lambda x: x - 0.5, [1.0, 2.0]),
        ]

        for typecode, initial, fn, expected in test_cases:
            arr = array.array(typecode, initial)
            arrayops.map_inplace(arr, fn)
            result = list(arr)
            for i, (r, e) in enumerate(zip(result, expected)):
                assert abs(r - e) < 1e-6, f"Failed for type {typecode} at index {i}"


class TestFilter:
    """Tests for filter operation."""

    def test_filter_even(self):
        """Test filter keeps even numbers."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5, 6])
        result = arrayops.filter(arr, lambda x: x % 2 == 0)
        assert list(result) == [2, 4, 6]
        assert result.typecode == arr.typecode

    def test_filter_empty_result(self):
        """Test filter with predicate that excludes all elements."""
        import arrayops

        arr = array.array("i", [1, 3, 5])
        result = arrayops.filter(arr, lambda x: x % 2 == 0)
        assert len(result) == 0
        assert result.typecode == arr.typecode

    def test_filter_empty_input(self):
        """Test filter with empty array."""
        import arrayops

        arr = array.array("i", [])
        result = arrayops.filter(arr, lambda x: x > 0)
        assert len(result) == 0
        assert result.typecode == arr.typecode

    def test_filter_all_pass(self):
        """Test filter when all elements pass."""
        import arrayops

        arr = array.array("i", [1, 2, 3])
        result = arrayops.filter(arr, lambda x: x > 0)
        assert list(result) == [1, 2, 3]
        assert result.typecode == arr.typecode

    def test_filter_preserves_type(self):
        """Test that filter preserves input array type."""
        import arrayops

        test_cases = [
            ("i", [1, 2, 3, 4, 5], lambda x: x > 2, [3, 4, 5]),
            ("f", [1.0, 2.0, 3.0], lambda x: x > 1.5, [2.0, 3.0]),
            ("d", [1.5, 2.5, 3.5], lambda x: x < 3.0, [1.5, 2.5]),
        ]

        for typecode, initial, predicate, expected in test_cases:
            arr = array.array(typecode, initial)
            result = arrayops.filter(arr, predicate)
            assert list(result) == expected
            assert result.typecode == typecode

    def test_filter_with_function(self):
        """Test filter with a named function."""
        import arrayops

        def is_positive(x):
            return x > 0

        arr = array.array("i", [-1, 0, 1, 2])
        result = arrayops.filter(arr, is_positive)
        assert list(result) == [1, 2]


class TestReduce:
    """Tests for reduce operation."""

    def test_reduce_sum(self):
        """Test reduce with addition."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = arrayops.reduce(arr, lambda acc, x: acc + x)
        assert result == 15

    def test_reduce_multiply(self):
        """Test reduce with multiplication."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4])
        result = arrayops.reduce(arr, lambda acc, x: acc * x)
        assert result == 24

    def test_reduce_with_initial(self):
        """Test reduce with initial value."""
        import arrayops

        arr = array.array("i", [1, 2, 3])
        result = arrayops.reduce(arr, lambda acc, x: acc + x, initial=10)
        assert result == 16

    def test_reduce_empty_with_initial(self):
        """Test reduce on empty array with initial value."""
        import arrayops

        arr = array.array("i", [])
        result = arrayops.reduce(arr, lambda acc, x: acc + x, initial=42)
        assert result == 42

    def test_reduce_empty_no_initial(self):
        """Test reduce on empty array without initial raises error."""
        import arrayops

        arr = array.array("i", [])
        with pytest.raises(
            ValueError, match="reduce\\(\\) of empty array with no initial value"
        ):
            arrayops.reduce(arr, lambda acc, x: acc + x)

    def test_reduce_single_element(self):
        """Test reduce with single element array."""
        import arrayops

        arr = array.array("i", [42])
        result = arrayops.reduce(arr, lambda acc, x: acc + x)
        assert result == 42

    def test_reduce_float(self):
        """Test reduce with float array."""
        import arrayops

        arr = array.array("d", [1.5, 2.5, 3.5])
        result = arrayops.reduce(arr, lambda acc, x: acc + x)
        assert abs(result - 7.5) < 1e-10

    def test_reduce_all_types(self):
        """Test reduce works with all numeric types."""
        import arrayops

        test_cases = [
            ("i", [1, 2, 3], lambda acc, x: acc + x, 6),
            ("f", [1.0, 2.0, 3.0], lambda acc, x: acc + x, 6.0),
            ("d", [1.5, 2.5], lambda acc, x: acc + x, 4.0),
        ]

        for typecode, initial, fn, expected in test_cases:
            arr = array.array(typecode, initial)
            result = arrayops.reduce(arr, fn)
            assert abs(result - expected) < 1e-6, f"Failed for type {typecode}"

    def test_reduce_with_function(self):
        """Test reduce with a named function."""
        import arrayops

        def add(acc, x):
            return acc + x

        arr = array.array("i", [1, 2, 3])
        result = arrayops.reduce(arr, add)
        assert result == 6


class TestMapRobust:
    """Robust tests for map operation - edge cases and error handling."""

    def test_map_non_callable(self):
        """Test map raises error for non-callable function."""
        import arrayops

        arr = array.array("i", [1, 2, 3])
        with pytest.raises(TypeError):
            arrayops.map(arr, "not a function")

    def test_map_function_raises_exception(self):
        """Test map propagates exceptions from Python function."""
        import arrayops

        def failing_func(x):
            raise ValueError("Test error")

        arr = array.array("i", [1, 2, 3])
        with pytest.raises(ValueError, match="Test error"):
            arrayops.map(arr, failing_func)

    def test_map_wrong_return_type(self):
        """Test map with function returning incompatible type."""
        import arrayops

        def wrong_type(x):
            return "string"  # Should fail when extracting back to int

        arr = array.array("i", [1, 2, 3])
        with pytest.raises((TypeError, ValueError)):
            arrayops.map(arr, wrong_type)

    def test_map_large_array(self):
        """Test map with large array."""
        import arrayops

        arr = array.array("i", list(range(10000)))
        result = arrayops.map(arr, lambda x: x * 2)
        assert len(result) == 10000
        assert result[0] == 0
        assert result[5000] == 10000
        assert result[9999] == 19998

    def test_map_boundary_values(self):
        """Test map with boundary values."""
        import arrayops

        # Test with values near boundaries (but that don't overflow when mapped)
        arr = array.array("i", [-2147483647, 0, 2147483646])
        result = arrayops.map(arr, lambda x: x + 1)
        assert list(result) == [-2147483646, 1, 2147483647]

    def test_map_all_numeric_types_comprehensive(self):
        """Test map with all numeric types comprehensively."""
        import arrayops

        test_cases = [
            ("b", [-1, 0, 1], lambda x: x * 2, [-2, 0, 2]),
            ("B", [1, 2, 254], lambda x: x + 1, [2, 3, 255]),  # Don't overflow
            ("h", [-32767, 0, 32766], lambda x: x + 1, [-32766, 1, 32767]),
            ("H", [1, 65534], lambda x: x + 1, [2, 65535]),
            ("i", [-1, 0, 1], lambda x: x * 3, [-3, 0, 3]),
            ("I", [1, 2, 3], lambda x: x * 2, [2, 4, 6]),
            ("l", [-1, 0, 1], lambda x: x * 2, [-2, 0, 2]),
            ("L", [1, 2, 3], lambda x: x * 2, [2, 4, 6]),
            ("f", [1.5, 2.5, 3.5], lambda x: x * 2.0, [3.0, 5.0, 7.0]),
            ("d", [1.5, 2.5, 3.5], lambda x: x * 2.0, [3.0, 5.0, 7.0]),
        ]

        for typecode, initial, fn, expected in test_cases:
            arr = array.array(typecode, initial)
            result = arrayops.map(arr, fn)
            result_list = list(result)
            for i, (r, e) in enumerate(zip(result_list, expected)):
                if typecode in ("f", "d"):
                    assert abs(r - e) < 1e-6, (
                        f"Failed for type {typecode} at index {i}: {r} != {e}"
                    )
                else:
                    assert r == e, (
                        f"Failed for type {typecode} at index {i}: {r} != {e}"
                    )

    def test_map_with_bound_method(self):
        """Test map with bound method."""
        import arrayops

        class Multiplier:
            def __init__(self, factor):
                self.factor = factor

            def multiply(self, x):
                return x * self.factor

        mult = Multiplier(3)
        arr = array.array("i", [1, 2, 3])
        result = arrayops.map(arr, mult.multiply)
        assert list(result) == [3, 6, 9]

    def test_map_not_array(self):
        """Test map raises error for non-array input."""
        import arrayops

        with pytest.raises(TypeError, match="Expected array.array"):
            arrayops.map([1, 2, 3], lambda x: x * 2)


class TestMapInplaceRobust:
    """Robust tests for map_inplace operation - edge cases and error handling."""

    def test_map_inplace_non_callable(self):
        """Test map_inplace raises error for non-callable function."""
        import arrayops

        arr = array.array("i", [1, 2, 3])
        with pytest.raises(TypeError):
            arrayops.map_inplace(arr, "not a function")

    def test_map_inplace_function_raises_exception(self):
        """Test map_inplace propagates exceptions from Python function."""
        import arrayops

        def failing_func(x):
            raise ValueError("Test error")

        arr = array.array("i", [1, 2, 3])
        with pytest.raises(ValueError, match="Test error"):
            arrayops.map_inplace(arr, failing_func)
        # Array may be partially modified if error occurs mid-operation
        # (depends on where error occurs)

    def test_map_inplace_wrong_return_type(self):
        """Test map_inplace with function returning incompatible type."""
        import arrayops

        def wrong_type(x):
            return "string"  # Should fail when extracting back to int

        arr = array.array("i", [1, 2, 3])
        with pytest.raises((TypeError, ValueError)):
            arrayops.map_inplace(arr, wrong_type)

    def test_map_inplace_large_array(self):
        """Test map_inplace with large array."""
        import arrayops

        arr = array.array("i", list(range(10000)))
        arrayops.map_inplace(arr, lambda x: x * 2)
        assert len(arr) == 10000
        assert arr[0] == 0
        assert arr[5000] == 10000
        assert arr[9999] == 19998

    def test_map_inplace_all_numeric_types_comprehensive(self):
        """Test map_inplace with all numeric types comprehensively."""
        import arrayops

        test_cases = [
            ("b", [-1, 0, 1], lambda x: x * 2),
            ("B", [1, 2, 3], lambda x: x * 2),
            ("h", [-1, 0, 1], lambda x: x * 2),
            ("H", [1, 2, 3], lambda x: x * 2),
            ("i", [-1, 0, 1], lambda x: x * 2),
            ("I", [1, 2, 3], lambda x: x * 2),
            ("l", [-1, 0, 1], lambda x: x * 2),
            ("L", [1, 2, 3], lambda x: x * 2),
            ("f", [1.0, 2.0, 3.0], lambda x: x * 2.0),
            ("d", [1.0, 2.0, 3.0], lambda x: x * 2.0),
        ]

        for typecode, initial, fn in test_cases:
            arr = array.array(typecode, initial)
            expected = [fn(x) for x in initial]
            arrayops.map_inplace(arr, fn)
            result = list(arr)
            for i, (r, e) in enumerate(zip(result, expected)):
                if typecode in ("f", "d"):
                    assert abs(r - e) < 1e-6, f"Failed for type {typecode} at index {i}"
                else:
                    assert r == e, (
                        f"Failed for type {typecode} at index {i}: {r} != {e}"
                    )

    def test_map_inplace_not_array(self):
        """Test map_inplace raises error for non-array input."""
        import arrayops

        with pytest.raises(TypeError, match="Expected array.array"):
            arrayops.map_inplace([1, 2, 3], lambda x: x * 2)


class TestFilterRobust:
    """Robust tests for filter operation - edge cases and error handling."""

    def test_filter_non_callable(self):
        """Test filter raises error for non-callable predicate."""
        import arrayops

        arr = array.array("i", [1, 2, 3])
        with pytest.raises(TypeError):
            arrayops.filter(arr, "not a function")

    def test_filter_predicate_raises_exception(self):
        """Test filter propagates exceptions from Python predicate."""
        import arrayops

        def failing_pred(x):
            raise ValueError("Test error")

        arr = array.array("i", [1, 2, 3])
        with pytest.raises(ValueError, match="Test error"):
            arrayops.filter(arr, failing_pred)

    def test_filter_non_boolean_return(self):
        """Test filter requires explicit boolean return."""
        import arrayops

        # Filter requires explicit boolean conversion
        arr = array.array("i", [0, 1, 2, 3, 0, 4])
        with pytest.raises(TypeError):
            # Should fail - requires explicit bool conversion
            arrayops.filter(arr, lambda x: x)

        # Explicit boolean conversion works
        result = arrayops.filter(arr, lambda x: bool(x))
        assert list(result) == [1, 2, 3, 4]

    def test_filter_large_array(self):
        """Test filter with large array."""
        import arrayops

        arr = array.array("i", list(range(10000)))
        result = arrayops.filter(arr, lambda x: x % 2 == 0)
        assert len(result) == 5000
        assert result[0] == 0
        assert result[1] == 2
        assert result[4999] == 9998

    def test_filter_all_numeric_types_comprehensive(self):
        """Test filter with all numeric types comprehensively."""
        import arrayops

        test_cases = [
            ("b", [-2, -1, 0, 1, 2], lambda x: x > 0, [1, 2]),
            ("B", [0, 1, 2, 255], lambda x: x > 1, [2, 255]),
            ("h", [-100, 0, 100], lambda x: x != 0, [-100, 100]),
            ("H", [0, 100, 200], lambda x: x > 50, [100, 200]),
            ("i", [-2, -1, 0, 1, 2], lambda x: x >= 0, [0, 1, 2]),
            ("I", [0, 1, 2, 3], lambda x: x % 2 == 0, [0, 2]),
            ("l", [-1000, 0, 1000], lambda x: x > 0, [1000]),
            ("L", [0, 1000, 2000], lambda x: x > 500, [1000, 2000]),
            ("f", [0.0, 1.5, 2.5, -1.0], lambda x: x > 0.0, [1.5, 2.5]),
            ("d", [0.0, 1.5, 2.5, -1.0], lambda x: x > 0.0, [1.5, 2.5]),
        ]

        for typecode, initial, predicate, expected in test_cases:
            arr = array.array(typecode, initial)
            result = arrayops.filter(arr, predicate)
            result_list = list(result)
            assert result_list == expected, (
                f"Failed for type {typecode}: {result_list} != {expected}"
            )

    def test_filter_not_array(self):
        """Test filter raises error for non-array input."""
        import arrayops

        with pytest.raises(TypeError, match="Expected array.array"):
            arrayops.filter([1, 2, 3], lambda x: x > 0)


class TestReduceRobust:
    """Robust tests for reduce operation - edge cases and error handling."""

    def test_reduce_non_callable(self):
        """Test reduce raises error for non-callable function."""
        import arrayops

        arr = array.array("i", [1, 2, 3])
        with pytest.raises(TypeError):
            arrayops.reduce(arr, "not a function")

    def test_reduce_function_raises_exception(self):
        """Test reduce propagates exceptions from Python function."""
        import arrayops

        def failing_func(acc, x):
            raise ValueError("Test error")

        arr = array.array("i", [1, 2, 3])
        with pytest.raises(ValueError, match="Test error"):
            arrayops.reduce(arr, failing_func)

    def test_reduce_single_element_no_initial(self):
        """Test reduce with single element array and no initial."""
        import arrayops

        arr = array.array("i", [42])
        result = arrayops.reduce(arr, lambda acc, x: acc + x)
        assert result == 42

    def test_reduce_single_element_with_initial(self):
        """Test reduce with single element array and initial."""
        import arrayops

        arr = array.array("i", [42])
        result = arrayops.reduce(arr, lambda acc, x: acc + x, initial=10)
        assert result == 52

    def test_reduce_large_array(self):
        """Test reduce with large array."""
        import arrayops

        arr = array.array("i", list(range(1000)))
        result = arrayops.reduce(arr, lambda acc, x: acc + x)
        expected = sum(range(1000))
        assert result == expected

    def test_reduce_different_accumulator_type(self):
        """Test reduce with function returning different type."""
        import arrayops

        arr = array.array("i", [1, 2, 3])
        result = arrayops.reduce(arr, lambda acc, x: str(acc) + str(x))
        assert isinstance(result, str)
        assert result == "123"  # First element is used as initial, so "1" + "2" + "3"

    def test_reduce_with_initial_string(self):
        """Test reduce with string initial value."""
        import arrayops

        arr = array.array("i", [1, 2, 3])
        result = arrayops.reduce(arr, lambda acc, x: str(acc) + str(x), initial="sum:")
        assert result == "sum:123"

    def test_reduce_all_numeric_types_comprehensive(self):
        """Test reduce with all numeric types comprehensively."""
        import arrayops

        test_cases = [
            ("b", [-1, 0, 1], lambda acc, x: acc + x, 0),
            ("B", [1, 2, 3], lambda acc, x: acc + x, 6),
            ("h", [-100, 0, 100], lambda acc, x: acc + x, 0),
            ("H", [100, 200], lambda acc, x: acc + x, 300),
            ("i", [1, 2, 3], lambda acc, x: acc + x, 6),
            ("I", [1, 2, 3], lambda acc, x: acc + x, 6),
            ("l", [1000, 2000], lambda acc, x: acc + x, 3000),
            ("L", [1000, 2000], lambda acc, x: acc + x, 3000),
            ("f", [1.5, 2.5, 3.5], lambda acc, x: acc + x, 7.5),
            ("d", [1.5, 2.5, 3.5], lambda acc, x: acc + x, 7.5),
        ]

        for typecode, initial, fn, expected in test_cases:
            arr = array.array(typecode, initial)
            result = arrayops.reduce(arr, fn)
            if typecode in ("f", "d"):
                assert abs(result - expected) < 1e-6, (
                    f"Failed for type {typecode}: {result} != {expected}"
                )
            else:
                assert result == expected, (
                    f"Failed for type {typecode}: {result} != {expected}"
                )

    def test_reduce_not_array(self):
        """Test reduce raises error for non-array input."""
        import arrayops

        with pytest.raises(TypeError, match="Expected array.array"):
            arrayops.reduce([1, 2, 3], lambda acc, x: acc + x)

    def test_reduce_multiply_large_numbers(self):
        """Test reduce with multiplication that could overflow."""
        import arrayops

        arr = array.array("i", [100, 200, 300])
        result = arrayops.reduce(arr, lambda acc, x: acc * x)
        # Python handles overflow, so this should work
        assert result == 100 * 200 * 300
