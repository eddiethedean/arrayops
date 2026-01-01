"""Basic tests for arrayops package."""

import array
import pytest
import sys

try:
    import numpy as np

    NUMPY_AVAILABLE = True
except ImportError:
    NUMPY_AVAILABLE = False


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

        # Check existing functions
        assert hasattr(arrayops, "sum")
        assert hasattr(arrayops, "scale")
        assert hasattr(arrayops, "map")
        assert hasattr(arrayops, "map_inplace")
        assert hasattr(arrayops, "filter")
        assert hasattr(arrayops, "reduce")
        # Check new statistical functions
        assert hasattr(arrayops, "mean")
        assert hasattr(arrayops, "min")
        assert hasattr(arrayops, "max")
        assert hasattr(arrayops, "std")
        assert hasattr(arrayops, "var")
        assert hasattr(arrayops, "median")
        # Check new element-wise functions
        assert hasattr(arrayops, "add")
        assert hasattr(arrayops, "multiply")
        assert hasattr(arrayops, "clip")
        assert hasattr(arrayops, "normalize")
        # Check new array manipulation functions
        assert hasattr(arrayops, "reverse")
        assert hasattr(arrayops, "sort")
        assert hasattr(arrayops, "unique")

        # Check __all__
        assert "sum" in arrayops.__all__
        assert "scale" in arrayops.__all__
        assert "map" in arrayops.__all__
        assert "map_inplace" in arrayops.__all__
        assert "filter" in arrayops.__all__
        assert "reduce" in arrayops.__all__
        assert "mean" in arrayops.__all__
        assert "min" in arrayops.__all__
        assert "max" in arrayops.__all__
        assert "std" in arrayops.__all__
        assert "var" in arrayops.__all__
        assert "median" in arrayops.__all__
        assert "add" in arrayops.__all__
        assert "multiply" in arrayops.__all__
        assert "clip" in arrayops.__all__
        assert "normalize" in arrayops.__all__
        assert "reverse" in arrayops.__all__
        assert "sort" in arrayops.__all__
        assert "unique" in arrayops.__all__
        assert arrayops.__version__ == "1.0.0"

    def test_module_import_error_with_arrayops_in_message(self):
        """Test helpful error message when _arrayops module is missing."""
        # Save original state and clear all arrayops submodules
        original_modules = {}
        modules_to_clear = [
            "arrayops",
            "arrayops._arrayops",
            "arrayops.basic",
            "arrayops.transform",
            "arrayops.stats",
            "arrayops.elementwise",
            "arrayops.manipulation",
            "arrayops.slice",
            "arrayops.iterator",
            "arrayops.lazy",
        ]
        for module_name in modules_to_clear:
            if module_name in sys.modules:
                original_modules[module_name] = sys.modules.pop(module_name)

        try:
            # Read and execute the __init__.py file directly to trigger error path
            import os

            # Try arrayops first, then arrayops_src (for CI where directory is renamed)
            base_dir = os.path.dirname(os.path.dirname(__file__))
            init_path = os.path.join(base_dir, "arrayops", "__init__.py")
            if not os.path.exists(init_path):
                init_path = os.path.join(base_dir, "arrayops_src", "__init__.py")

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
            for module_name in list(sys.modules.keys()):
                if module_name.startswith("arrayops"):
                    del sys.modules[module_name]
            for module_name, module in original_modules.items():
                sys.modules[module_name] = module

    def test_module_import_error_other_error(self):
        """Test that other ImportErrors are re-raised unchanged."""
        # Save original state and clear all arrayops submodules
        original_modules = {}
        modules_to_clear = [
            "arrayops",
            "arrayops._arrayops",
            "arrayops.basic",
            "arrayops.transform",
            "arrayops.stats",
            "arrayops.elementwise",
            "arrayops.manipulation",
            "arrayops.slice",
            "arrayops.iterator",
            "arrayops.lazy",
        ]
        for module_name in modules_to_clear:
            if module_name in sys.modules:
                original_modules[module_name] = sys.modules.pop(module_name)

        try:
            # Read and execute the __init__.py file with a different ImportError
            import os
            import builtins

            # Try arrayops first, then arrayops_src (for CI where directory is renamed)
            base_dir = os.path.dirname(os.path.dirname(__file__))
            init_path = os.path.join(base_dir, "arrayops", "__init__.py")
            if not os.path.exists(init_path):
                init_path = os.path.join(base_dir, "arrayops_src", "__init__.py")

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
            for module_name in list(sys.modules.keys()):
                if module_name.startswith("arrayops"):
                    del sys.modules[module_name]
            for module_name, module in original_modules.items():
                sys.modules[module_name] = module


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


@pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
class TestNumPyInterop:
    """Tests for NumPy array interoperability."""

    def test_sum_numpy_int32(self):
        """Test sum with numpy int32 array."""
        import arrayops

        arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
        result = arrayops.sum(arr)
        assert result == 15
        assert isinstance(result, (int, np.integer))

    def test_sum_numpy_float64(self):
        """Test sum with numpy float64 array."""
        import arrayops

        arr = np.array([1.5, 2.5, 3.5], dtype=np.float64)
        result = arrayops.sum(arr)
        assert abs(result - 7.5) < 1e-10

    def test_sum_numpy_all_types(self):
        """Test sum with all numpy dtypes."""
        import arrayops

        test_cases = [
            (np.int8, [1, 2, 3]),
            (np.int16, [1, 2, 3]),
            (np.int32, [1, 2, 3]),
            (np.int64, [1, 2, 3]),
            (np.uint8, [1, 2, 3]),
            (np.uint16, [1, 2, 3]),
            (np.uint32, [1, 2, 3]),
            (np.uint64, [1, 2, 3]),
            (np.float32, [1.5, 2.5, 3.5]),
            (np.float64, [1.5, 2.5, 3.5]),
        ]

        for dtype, values in test_cases:
            arr = np.array(values, dtype=dtype)
            result = arrayops.sum(arr)
            expected = sum(values)
            assert abs(result - expected) < 1e-6, f"Failed for dtype {dtype}"

    def test_sum_numpy_empty(self):
        """Test sum with empty numpy array."""
        import arrayops

        arr = np.array([], dtype=np.int32)
        result = arrayops.sum(arr)
        assert result == 0

    def test_scale_numpy_array(self):
        """Test scale with numpy array."""
        import arrayops

        arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
        arrayops.scale(arr, 2.0)
        expected = np.array([2, 4, 6, 8, 10], dtype=np.int32)
        np.testing.assert_array_equal(arr, expected)

    def test_map_numpy_returns_numpy(self):
        """Test map with numpy array returns numpy array."""
        import arrayops

        arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
        result = arrayops.map(arr, lambda x: x * 2)
        assert isinstance(result, np.ndarray)
        assert result.dtype == np.int32
        np.testing.assert_array_equal(
            result, np.array([2, 4, 6, 8, 10], dtype=np.int32)
        )

    def test_filter_numpy_returns_numpy(self):
        """Test filter with numpy array returns numpy array."""
        import arrayops

        arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
        result = arrayops.filter(arr, lambda x: x % 2 == 0)
        assert isinstance(result, np.ndarray)
        assert result.dtype == np.int32
        np.testing.assert_array_equal(result, np.array([2, 4], dtype=np.int32))

    def test_reduce_numpy(self):
        """Test reduce with numpy array."""
        import arrayops

        arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
        result = arrayops.reduce(arr, lambda acc, x: acc + x)
        assert result == 15

    def test_numpy_multidimensional_error(self):
        """Test that multi-dimensional numpy arrays raise error."""
        import arrayops

        arr = np.array([[1, 2], [3, 4]], dtype=np.int32)
        with pytest.raises(TypeError, match="1-dimensional"):
            arrayops.sum(arr)

    def test_numpy_non_contiguous_error(self):
        """Test that non-contiguous numpy arrays raise error."""
        import arrayops

        # Create a non-contiguous array by taking every other element
        arr = np.array([1, 2, 3, 4, 5, 6, 7, 8], dtype=np.int32)
        arr_non_contig = arr[::2]  # Strided view, non-contiguous
        with pytest.raises(TypeError, match="contiguous"):
            arrayops.sum(arr_non_contig)


class TestMemoryViewInterop:
    """Tests for memoryview interoperability."""

    def test_sum_memoryview_from_array(self):
        """Test sum with memoryview from array.array."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        mv = memoryview(arr)
        result = arrayops.sum(mv)
        assert result == 15

    def test_sum_memoryview_from_bytes(self):
        """Test sum with memoryview from bytes."""
        import arrayops

        # Create bytes with int32 values
        arr = array.array("i", [1, 2, 3])
        data = arr.tobytes()
        mv = memoryview(data).cast("i")
        result = arrayops.sum(mv)
        assert result == 6

    def test_scale_memoryview_writable(self):
        """Test scale with writable memoryview."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        mv = memoryview(arr)  # Writable memoryview
        arrayops.scale(mv, 2.0)
        assert list(arr) == [2, 4, 6, 8, 10]

    def test_scale_memoryview_readonly_error(self):
        """Test scale with read-only memoryview raises error."""
        import arrayops

        data = array.array("i", [1, 2, 3]).tobytes()
        mv = memoryview(data)  # Read-only memoryview
        with pytest.raises(ValueError, match="read-only"):
            arrayops.scale(mv, 2.0)

    def test_map_memoryview_returns_array(self):
        """Test map with memoryview returns array.array."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        mv = memoryview(arr)
        result = arrayops.map(mv, lambda x: x * 2)
        assert isinstance(result, array.array)
        assert list(result) == [2, 4, 6, 8, 10]

    def test_filter_memoryview_returns_array(self):
        """Test filter with memoryview returns array.array."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        mv = memoryview(arr)
        result = arrayops.filter(mv, lambda x: x % 2 == 0)
        assert isinstance(result, array.array)
        assert list(result) == [2, 4]

    def test_reduce_memoryview(self):
        """Test reduce with memoryview."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        mv = memoryview(arr)
        result = arrayops.reduce(mv, lambda acc, x: acc + x)
        assert result == 15

    def test_memoryview_all_types(self):
        """Test memoryview with different format types."""
        import arrayops

        test_cases = [
            ("b", array.array("b", [1, 2, 3])),
            ("B", array.array("B", [1, 2, 3])),
            ("h", array.array("h", [1, 2, 3])),
            ("H", array.array("H", [1, 2, 3])),
            ("i", array.array("i", [1, 2, 3])),
            ("I", array.array("I", [1, 2, 3])),
            ("l", array.array("l", [1, 2, 3])),
            ("L", array.array("L", [1, 2, 3])),
            ("f", array.array("f", [1.5, 2.5, 3.5])),
            ("d", array.array("d", [1.5, 2.5, 3.5])),
        ]

        for typecode, arr in test_cases:
            mv = memoryview(arr)
            result = arrayops.sum(mv)
            expected = sum(arr)
            assert abs(result - expected) < 1e-6, f"Failed for typecode {typecode}"

    def test_map_inplace_memoryview_writable(self):
        """Test map_inplace with writable memoryview."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        mv = memoryview(arr)
        arrayops.map_inplace(mv, lambda x: x * 2)
        assert list(arr) == [2, 4, 6, 8, 10]

    def test_map_inplace_memoryview_readonly_error(self):
        """Test map_inplace with read-only memoryview raises error."""
        import arrayops

        data = array.array("i", [1, 2, 3]).tobytes()
        mv = memoryview(data)  # Read-only
        with pytest.raises(ValueError, match="read-only"):
            arrayops.map_inplace(mv, lambda x: x * 2)


class TestStatisticalOperations:
    """Tests for statistical operations (mean, min, max, std, var, median)."""

    def test_mean_int32(self):
        """Test mean with int32 array."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = arrayops.mean(arr)
        assert result == 3.0
        assert isinstance(result, float)

    def test_mean_float64(self):
        """Test mean with float64 array."""
        import arrayops

        arr = array.array("d", [1.5, 2.5, 3.5, 4.5])
        result = arrayops.mean(arr)
        assert abs(result - 3.0) < 1e-10

    def test_mean_empty(self):
        """Test mean with empty array raises ValueError."""
        import arrayops

        arr = array.array("i", [])
        with pytest.raises(ValueError, match="mean.*empty"):
            arrayops.mean(arr)

    def test_min_int32(self):
        """Test min with int32 array."""
        import arrayops

        arr = array.array("i", [5, 2, 8, 1, 9])
        result = arrayops.min(arr)
        assert result == 1
        assert isinstance(result, int)

    def test_min_float64(self):
        """Test min with float64 array."""
        import arrayops

        arr = array.array("d", [5.5, 2.2, 8.8, 1.1, 9.9])
        result = arrayops.min(arr)
        assert abs(result - 1.1) < 1e-10

    def test_min_empty(self):
        """Test min with empty array raises ValueError."""
        import arrayops

        arr = array.array("i", [])
        with pytest.raises(ValueError, match="min.*empty"):
            arrayops.min(arr)

    def test_max_int32(self):
        """Test max with int32 array."""
        import arrayops

        arr = array.array("i", [5, 2, 8, 1, 9])
        result = arrayops.max(arr)
        assert result == 9
        assert isinstance(result, int)

    def test_max_float64(self):
        """Test max with float64 array."""
        import arrayops

        arr = array.array("d", [5.5, 2.2, 8.8, 1.1, 9.9])
        result = arrayops.max(arr)
        assert abs(result - 9.9) < 1e-10

    def test_max_empty(self):
        """Test max with empty array raises ValueError."""
        import arrayops

        arr = array.array("i", [])
        with pytest.raises(ValueError, match="max.*empty"):
            arrayops.max(arr)

    def test_std_int32(self):
        """Test std with int32 array."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = arrayops.std(arr)
        # Population std: sqrt(sum((x-mean)^2)/n) = sqrt(10/5) = sqrt(2) ≈ 1.414
        expected = (10.0 / 5.0) ** 0.5
        assert abs(result - expected) < 1e-10

    def test_std_float64(self):
        """Test std with float64 array."""
        import arrayops

        arr = array.array("d", [1.0, 2.0, 3.0, 4.0, 5.0])
        result = arrayops.std(arr)
        expected = (10.0 / 5.0) ** 0.5
        assert abs(result - expected) < 1e-10

    def test_std_empty(self):
        """Test std with empty array raises ValueError."""
        import arrayops

        arr = array.array("i", [])
        with pytest.raises(ValueError, match="var.*empty"):
            arrayops.std(arr)

    def test_var_int32(self):
        """Test var with int32 array."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = arrayops.var(arr)
        # Population var: sum((x-mean)^2)/n = 10/5 = 2.0
        assert abs(result - 2.0) < 1e-10

    def test_var_float64(self):
        """Test var with float64 array."""
        import arrayops

        arr = array.array("d", [1.0, 2.0, 3.0, 4.0, 5.0])
        result = arrayops.var(arr)
        assert abs(result - 2.0) < 1e-10

    def test_var_empty(self):
        """Test var with empty array raises ValueError."""
        import arrayops

        arr = array.array("i", [])
        with pytest.raises(ValueError, match="var.*empty"):
            arrayops.var(arr)

    def test_median_int32_odd(self):
        """Test median with odd-length int32 array."""
        import arrayops

        arr = array.array("i", [5, 2, 8, 1, 9])
        result = arrayops.median(arr)
        assert result == 5
        assert isinstance(result, int)

    def test_median_int32_even(self):
        """Test median with even-length int32 array (returns lower median)."""
        import arrayops

        arr = array.array("i", [5, 2, 8, 1])
        result = arrayops.median(arr)
        # Sorted: [1, 2, 5, 8], lower median = 2
        assert result == 2

    def test_median_float64(self):
        """Test median with float64 array."""
        import arrayops

        arr = array.array("d", [5.5, 2.2, 8.8, 1.1, 9.9])
        result = arrayops.median(arr)
        # Sorted: [1.1, 2.2, 5.5, 8.8, 9.9], median = 5.5
        assert abs(result - 5.5) < 1e-10

    def test_median_empty(self):
        """Test median with empty array raises ValueError."""
        import arrayops

        arr = array.array("i", [])
        with pytest.raises(ValueError, match="median.*empty"):
            arrayops.median(arr)

    def test_statistical_all_types(self):
        """Test statistical operations with all numeric types."""
        import arrayops

        test_cases = [
            ("b", array.array("b", [1, 2, 3, 4, 5])),
            ("B", array.array("B", [1, 2, 3, 4, 5])),
            ("h", array.array("h", [1, 2, 3, 4, 5])),
            ("H", array.array("H", [1, 2, 3, 4, 5])),
            ("i", array.array("i", [1, 2, 3, 4, 5])),
            ("I", array.array("I", [1, 2, 3, 4, 5])),
            ("l", array.array("l", [1, 2, 3, 4, 5])),
            ("L", array.array("L", [1, 2, 3, 4, 5])),
            ("f", array.array("f", [1.0, 2.0, 3.0, 4.0, 5.0])),
            ("d", array.array("d", [1.0, 2.0, 3.0, 4.0, 5.0])),
        ]

        for typecode, arr in test_cases:
            mean_val = arrayops.mean(arr)
            assert abs(mean_val - 3.0) < 1e-6

            min_val = arrayops.min(arr)
            assert min_val == 1 or abs(min_val - 1.0) < 1e-6

            max_val = arrayops.max(arr)
            assert max_val == 5 or abs(max_val - 5.0) < 1e-6

            median_val = arrayops.median(arr)
            assert median_val == 3 or abs(median_val - 3.0) < 1e-6

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_mean_numpy(self):
        """Test mean with NumPy array."""
        import arrayops

        arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
        result = arrayops.mean(arr)
        assert abs(result - 3.0) < 1e-10

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_min_max_numpy(self):
        """Test min/max with NumPy array."""
        import arrayops

        arr = np.array([5, 2, 8, 1, 9], dtype=np.int32)
        assert arrayops.min(arr) == 1
        assert arrayops.max(arr) == 9


class TestElementWiseOperations:
    """Tests for element-wise operations (add, multiply, clip, normalize)."""

    def test_add_int32(self):
        """Test add with int32 arrays."""
        import arrayops

        arr1 = array.array("i", [1, 2, 3, 4, 5])
        arr2 = array.array("i", [10, 20, 30, 40, 50])
        result = arrayops.add(arr1, arr2)
        assert list(result) == [11, 22, 33, 44, 55]
        assert isinstance(result, array.array)

    def test_add_float64(self):
        """Test add with float64 arrays."""
        import arrayops

        arr1 = array.array("d", [1.5, 2.5, 3.5])
        arr2 = array.array("d", [10.0, 20.0, 30.0])
        result = arrayops.add(arr1, arr2)
        assert all(abs(a - b) < 1e-10 for a, b in zip(result, [11.5, 22.5, 33.5]))

    def test_add_mismatched_length(self):
        """Test add with mismatched array lengths raises ValueError."""
        import arrayops

        arr1 = array.array("i", [1, 2, 3])
        arr2 = array.array("i", [1, 2])
        with pytest.raises(ValueError, match="same length"):
            arrayops.add(arr1, arr2)

    def test_add_mismatched_type(self):
        """Test add with mismatched types raises TypeError."""
        import arrayops

        arr1 = array.array("i", [1, 2, 3])
        arr2 = array.array("f", [1.0, 2.0, 3.0])
        with pytest.raises(TypeError, match="same type"):
            arrayops.add(arr1, arr2)

    def test_multiply_int32(self):
        """Test multiply with int32 arrays."""
        import arrayops

        arr1 = array.array("i", [1, 2, 3, 4, 5])
        arr2 = array.array("i", [2, 3, 4, 5, 6])
        result = arrayops.multiply(arr1, arr2)
        assert list(result) == [2, 6, 12, 20, 30]

    def test_multiply_float64(self):
        """Test multiply with float64 arrays."""
        import arrayops

        arr1 = array.array("d", [1.5, 2.5, 3.5])
        arr2 = array.array("d", [2.0, 3.0, 4.0])
        result = arrayops.multiply(arr1, arr2)
        assert all(abs(a - b) < 1e-10 for a, b in zip(result, [3.0, 7.5, 14.0]))

    def test_multiply_empty(self):
        """Test multiply with empty arrays."""
        import arrayops

        arr1 = array.array("i", [])
        arr2 = array.array("i", [])
        result = arrayops.multiply(arr1, arr2)
        assert len(result) == 0

    def test_clip_int32(self):
        """Test clip with int32 array."""
        import arrayops

        arr = array.array("i", [1, 5, 10, 15, 20])
        arrayops.clip(arr, 5.0, 15.0)
        assert list(arr) == [5, 5, 10, 15, 15]

    def test_clip_float64(self):
        """Test clip with float64 array."""
        import arrayops

        arr = array.array("d", [1.5, 5.5, 10.5, 15.5, 20.5])
        arrayops.clip(arr, 5.0, 15.0)
        assert all(
            abs(a - b) < 1e-10 for a, b in zip(arr, [5.0, 5.5, 10.5, 15.0, 15.0])
        )

    def test_clip_invalid_range(self):
        """Test clip with min > max raises ValueError."""
        import arrayops

        arr = array.array("i", [1, 2, 3])
        with pytest.raises(ValueError, match="min_val must be <= max_val"):
            arrayops.clip(arr, 10.0, 5.0)

    def test_clip_empty(self):
        """Test clip with empty array (no error)."""
        import arrayops

        arr = array.array("i", [])
        arrayops.clip(arr, 0.0, 100.0)
        assert len(arr) == 0

    def test_normalize_float64(self):
        """Test normalize with float64 array."""
        import arrayops

        arr = array.array("d", [10.0, 20.0, 30.0, 40.0, 50.0])
        arrayops.normalize(arr)
        # After normalization: (x - 10) / (50 - 10) = (x - 10) / 40
        expected = [0.0, 0.25, 0.5, 0.75, 1.0]
        assert all(abs(a - b) < 1e-10 for a, b in zip(arr, expected))

    def test_normalize_int32_error(self):
        """Test normalize with int32 array raises ValueError."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        with pytest.raises(ValueError, match="float arrays"):
            arrayops.normalize(arr)

    def test_normalize_empty(self):
        """Test normalize with empty array (no error)."""
        import arrayops

        arr = array.array("d", [])
        arrayops.normalize(arr)
        assert len(arr) == 0

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_add_numpy(self):
        """Test add with NumPy arrays returns NumPy array."""
        import arrayops

        arr1 = np.array([1, 2, 3], dtype=np.int32)
        arr2 = np.array([10, 20, 30], dtype=np.int32)
        result = arrayops.add(arr1, arr2)
        assert isinstance(result, np.ndarray)
        np.testing.assert_array_equal(result, np.array([11, 22, 33], dtype=np.int32))

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_clip_numpy(self):
        """Test clip with NumPy array."""
        import arrayops

        arr = np.array([1, 5, 10, 15, 20], dtype=np.int32)
        arrayops.clip(arr, 5.0, 15.0)
        np.testing.assert_array_equal(arr, np.array([5, 5, 10, 15, 15], dtype=np.int32))


class TestArrayManipulation:
    """Tests for array manipulation operations (reverse, sort, unique)."""

    def test_reverse_int32(self):
        """Test reverse with int32 array."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        arrayops.reverse(arr)
        assert list(arr) == [5, 4, 3, 2, 1]

    def test_reverse_float64(self):
        """Test reverse with float64 array."""
        import arrayops

        arr = array.array("d", [1.5, 2.5, 3.5, 4.5])
        arrayops.reverse(arr)
        expected = [4.5, 3.5, 2.5, 1.5]
        assert all(abs(a - b) < 1e-10 for a, b in zip(arr, expected))

    def test_reverse_empty(self):
        """Test reverse with empty array (no error)."""
        import arrayops

        arr = array.array("i", [])
        arrayops.reverse(arr)
        assert len(arr) == 0

    def test_sort_int32(self):
        """Test sort with int32 array."""
        import arrayops

        arr = array.array("i", [5, 2, 8, 1, 9])
        arrayops.sort(arr)
        assert list(arr) == [1, 2, 5, 8, 9]

    def test_sort_float64(self):
        """Test sort with float64 array."""
        import arrayops

        arr = array.array("d", [5.5, 2.2, 8.8, 1.1, 9.9])
        arrayops.sort(arr)
        expected = [1.1, 2.2, 5.5, 8.8, 9.9]
        assert all(abs(a - b) < 1e-10 for a, b in zip(arr, expected))

    def test_sort_already_sorted(self):
        """Test sort with already sorted array."""
        import arrayops

        arr = array.array("i", [1, 2, 3, 4, 5])
        arrayops.sort(arr)
        assert list(arr) == [1, 2, 3, 4, 5]

    def test_sort_empty(self):
        """Test sort with empty array (no error)."""
        import arrayops

        arr = array.array("i", [])
        arrayops.sort(arr)
        assert len(arr) == 0

    def test_unique_int32(self):
        """Test unique with int32 array."""
        import arrayops

        arr = array.array("i", [5, 2, 8, 2, 1, 5, 9])
        result = arrayops.unique(arr)
        assert list(result) == [1, 2, 5, 8, 9]
        assert isinstance(result, array.array)

    def test_unique_float64(self):
        """Test unique with float64 array."""
        import arrayops

        arr = array.array("d", [5.5, 2.2, 8.8, 2.2, 1.1, 5.5, 9.9])
        result = arrayops.unique(arr)
        expected = [1.1, 2.2, 5.5, 8.8, 9.9]
        assert all(abs(a - b) < 1e-10 for a, b in zip(result, expected))

    def test_unique_all_unique(self):
        """Test unique with all unique values."""
        import arrayops

        arr = array.array("i", [5, 2, 8, 1, 9])
        result = arrayops.unique(arr)
        assert list(result) == [1, 2, 5, 8, 9]

    def test_unique_all_same(self):
        """Test unique with all same values."""
        import arrayops

        arr = array.array("i", [5, 5, 5, 5, 5])
        result = arrayops.unique(arr)
        assert list(result) == [5]

    def test_unique_empty(self):
        """Test unique with empty array."""
        import arrayops

        arr = array.array("i", [])
        result = arrayops.unique(arr)
        assert len(result) == 0
        assert isinstance(result, array.array)

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_reverse_numpy(self):
        """Test reverse with NumPy array."""
        import arrayops

        arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
        arrayops.reverse(arr)
        np.testing.assert_array_equal(arr, np.array([5, 4, 3, 2, 1], dtype=np.int32))

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_sort_numpy(self):
        """Test sort with NumPy array."""
        import arrayops

        arr = np.array([5, 2, 8, 1, 9], dtype=np.int32)
        arrayops.sort(arr)
        np.testing.assert_array_equal(arr, np.array([1, 2, 5, 8, 9], dtype=np.int32))

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_unique_numpy(self):
        """Test unique with NumPy array returns NumPy array."""
        import arrayops

        arr = np.array([5, 2, 8, 2, 1, 5, 9], dtype=np.int32)
        result = arrayops.unique(arr)
        assert isinstance(result, np.ndarray)
        np.testing.assert_array_equal(result, np.array([1, 2, 5, 8, 9], dtype=np.int32))
