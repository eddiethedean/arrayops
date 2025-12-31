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
        assert "sum" in arrayops.__all__
        assert "scale" in arrayops.__all__
        assert arrayops.__version__ == "0.1.0"

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
