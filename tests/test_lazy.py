"""Tests for lazy evaluation."""

import array
import pytest

try:
    import numpy as np

    NUMPY_AVAILABLE = True
except ImportError:
    NUMPY_AVAILABLE = False


class TestLazyArray:
    """Tests for LazyArray class."""

    def test_lazy_array_creation(self):
        """Test creating a LazyArray."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        assert lazy is not None
        assert hasattr(lazy, "map")
        assert hasattr(lazy, "filter")
        assert hasattr(lazy, "collect")

    def test_lazy_map(self):
        """Test lazy map operation."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        result_lazy = lazy.map(lambda x: x * 2)
        result = result_lazy.collect()
        assert list(result) == [2, 4, 6, 8, 10]

    def test_lazy_filter(self):
        """Test lazy filter operation."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        result_lazy = lazy.filter(lambda x: x > 2)
        result = result_lazy.collect()
        assert list(result) == [3, 4, 5]

    def test_lazy_chain(self):
        """Test chaining lazy operations."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        result_lazy = lazy.map(lambda x: x * 2).filter(lambda x: x > 5)
        result = result_lazy.collect()
        assert list(result) == [6, 8, 10]

    def test_lazy_multiple_maps(self):
        """Test multiple map operations."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3])
        lazy = ao.lazy_array(arr)
        result_lazy = lazy.map(lambda x: x * 2).map(lambda x: x + 1)
        result = result_lazy.collect()
        assert list(result) == [3, 5, 7]

    def test_lazy_multiple_filters(self):
        """Test multiple filter operations."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        lazy = ao.lazy_array(arr)
        result_lazy = lazy.filter(lambda x: x > 2).filter(lambda x: x < 8)
        result = result_lazy.collect()
        assert list(result) == [3, 4, 5, 6, 7]

    def test_lazy_empty_array(self):
        """Test lazy operations on empty array."""
        import arrayops as ao

        arr = array.array("i", [])
        lazy = ao.lazy_array(arr)
        result_lazy = lazy.map(lambda x: x * 2)
        result = result_lazy.collect()
        assert list(result) == []

    def test_lazy_cached_result(self):
        """Test that collect caches the result."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3])
        lazy = ao.lazy_array(arr)
        result_lazy = lazy.map(lambda x: x * 2)

        # First collect
        result1 = result_lazy.collect()
        # Second collect should return cached result
        result2 = result_lazy.collect()
        assert list(result1) == list(result2) == [2, 4, 6]

    def test_lazy_source(self):
        """Test accessing source array."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3])
        lazy = ao.lazy_array(arr)
        source = lazy.source()
        assert list(source) == [1, 2, 3]

    def test_lazy_len(self):
        """Test getting operation chain length."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3])
        lazy = ao.lazy_array(arr)
        assert lazy.len() == 0

        lazy = lazy.map(lambda x: x * 2)
        assert lazy.len() == 1

        lazy = lazy.filter(lambda x: x > 2)
        assert lazy.len() == 2

    def test_lazy_all_types(self):
        """Test lazy operations with all supported types."""
        import arrayops as ao

        test_cases = [
            ("i", [1, 2, 3], lambda x: x * 2, [2, 4, 6]),
            ("d", [1.5, 2.5, 3.5], lambda x: x * 2, [3.0, 5.0, 7.0]),
        ]

        for typecode, values, fn, expected in test_cases:
            arr = array.array(typecode, values)
            lazy = ao.lazy_array(arr)
            result = lazy.map(fn).collect()
            assert list(result) == expected, f"Failed for type {typecode}"

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_lazy_numpy(self):
        """Test lazy operations with NumPy array."""
        import arrayops as ao

        arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
        lazy = ao.lazy_array(arr)
        result = lazy.map(lambda x: x * 2).collect()
        assert list(result) == [2, 4, 6, 8, 10]

    def test_lazy_memoryview(self):
        """Test lazy operations with memoryview."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        mv = memoryview(arr)
        lazy = ao.lazy_array(mv)
        result = lazy.map(lambda x: x * 2).collect()
        assert list(result) == [2, 4, 6, 8, 10]
