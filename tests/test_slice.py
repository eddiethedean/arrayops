"""Tests for slice operation."""

import array
import pytest

try:
    import numpy as np

    NUMPY_AVAILABLE = True
except ImportError:
    NUMPY_AVAILABLE = False


class TestSlice:
    """Tests for slice operation."""

    def test_slice_basic(self):
        """Test basic slicing."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = ao.slice(arr, 1, 4)
        assert list(result) == [2, 3, 4]
        assert isinstance(result, memoryview)

    def test_slice_start_only(self):
        """Test slicing with only start index."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = ao.slice(arr, 2, None)
        assert list(result) == [3, 4, 5]

    def test_slice_end_only(self):
        """Test slicing with only end index."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = ao.slice(arr, None, 3)
        assert list(result) == [1, 2, 3]

    def test_slice_no_indices(self):
        """Test slicing with no indices (full array)."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = ao.slice(arr, None, None)
        assert list(result) == [1, 2, 3, 4, 5]

    def test_slice_empty(self):
        """Test slicing empty array."""
        import arrayops as ao

        arr = array.array("i", [])
        result = ao.slice(arr, None, None)
        assert list(result) == []

    def test_slice_single_element(self):
        """Test slicing single element."""
        import arrayops as ao

        arr = array.array("i", [42])
        result = ao.slice(arr, 0, 1)
        assert list(result) == [42]

    def test_slice_all_types(self):
        """Test slice with all supported types."""
        import arrayops as ao

        test_cases = [
            ("b", [-1, 0, 1, 2]),
            ("B", [1, 2, 3, 4]),
            ("h", [-100, 0, 100]),
            ("H", [100, 200, 300]),
            ("i", [1, 2, 3, 4]),
            ("I", [1, 2, 3, 4]),
            ("l", [1000, 2000, 3000]),
            ("L", [1000, 2000, 3000]),
            ("f", [1.5, 2.5, 3.5]),
            ("d", [1.5, 2.5, 3.5]),
        ]

        for typecode, values in test_cases:
            arr = array.array(typecode, values)
            result = ao.slice(arr, 1, len(values) - 1)
            expected = values[1 : len(values) - 1]
            assert list(result) == expected, f"Failed for type {typecode}"

    def test_slice_invalid_indices(self):
        """Test slice with invalid indices."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])

        # Start > end
        with pytest.raises(ValueError, match="Invalid slice indices"):
            ao.slice(arr, 3, 2)

        # Start > length
        with pytest.raises(ValueError, match="Invalid slice indices"):
            ao.slice(arr, 10, 15)

        # End > length
        with pytest.raises(ValueError, match="Invalid slice indices"):
            ao.slice(arr, 1, 10)

    def test_slice_zero_copy(self):
        """Test that slice returns a view (zero-copy)."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = ao.slice(arr, 1, 4)

        # Modify original array
        arr[2] = 99

        # View should reflect the change
        assert list(result) == [2, 99, 4]

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_slice_numpy(self):
        """Test slice with NumPy array."""
        import arrayops as ao

        arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
        result = ao.slice(arr, 1, 4)
        assert list(result) == [2, 3, 4]

    def test_slice_memoryview(self):
        """Test slice with memoryview."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        mv = memoryview(arr)
        result = ao.slice(mv, 1, 4)
        assert list(result) == [2, 3, 4]

    def test_slice_not_array(self):
        """Test slice raises error for non-array input."""
        import arrayops as ao

        with pytest.raises(TypeError):
            ao.slice([1, 2, 3], 0, 2)
