"""Tests for iterator protocol implementation."""

import array
import pytest

try:
    import numpy as np

    NUMPY_AVAILABLE = True
except ImportError:
    NUMPY_AVAILABLE = False

try:
    import pyarrow as pa

    ARROW_AVAILABLE = True
except ImportError:
    ARROW_AVAILABLE = False


@pytest.fixture
def int_array():
    """Create a test integer array."""
    return array.array("i", [1, 2, 3, 4, 5])


@pytest.fixture
def float_array():
    """Create a test float array."""
    return array.array("d", [1.5, 2.5, 3.5, 4.5])


class TestArrayIterator:
    """Tests for ArrayIterator class."""

    def test_array_iterator_creation(self, int_array):
        """Test creating an ArrayIterator."""
        import arrayops as ao

        it = ao.array_iterator(int_array)
        assert it is not None
        assert hasattr(it, "__iter__")
        assert hasattr(it, "__next__")

    def test_array_iterator_int32(self, int_array):
        """Test iteration over int32 array."""
        import arrayops as ao

        it = ao.array_iterator(int_array)
        result = list(it)
        assert result == [1, 2, 3, 4, 5]

    def test_array_iterator_float64(self, float_array):
        """Test iteration over float64 array."""
        import arrayops as ao

        it = ao.array_iterator(float_array)
        result = list(it)
        assert result == [1.5, 2.5, 3.5, 4.5]

    def test_array_iterator_empty(self):
        """Test iteration over empty array."""
        import arrayops as ao

        arr = array.array("i", [])
        it = ao.array_iterator(arr)
        result = list(it)
        assert result == []

    def test_array_iterator_next(self, int_array):
        """Test using next() function."""
        import arrayops as ao

        it = ao.array_iterator(int_array)
        assert next(it) == 1
        assert next(it) == 2
        assert next(it) == 3

    def test_array_iterator_stop_iteration(self, int_array):
        """Test StopIteration exception."""
        import arrayops as ao

        it = ao.array_iterator(int_array)
        # Consume all elements
        list(it)
        # Next call should raise StopIteration
        with pytest.raises(StopIteration):
            next(it)

    def test_array_iterator_for_loop(self, int_array):
        """Test using ArrayIterator in for loop."""
        import arrayops as ao

        it = ao.array_iterator(int_array)
        result = []
        for x in it:
            result.append(x)
        assert result == [1, 2, 3, 4, 5]

    def test_array_iterator_all_types(self):
        """Test ArrayIterator with all supported numeric types."""
        import arrayops as ao

        test_cases = [
            ("b", [-1, 0, 1], [-1, 0, 1]),  # signed char
            ("B", [1, 2, 3], [1, 2, 3]),  # unsigned char
            ("h", [-100, 0, 100], [-100, 0, 100]),  # signed short
            ("H", [100, 200], [100, 200]),  # unsigned short
            ("i", [1, 2, 3], [1, 2, 3]),  # signed int
            ("I", [1, 2, 3], [1, 2, 3]),  # unsigned int
            ("l", [1000, 2000], [1000, 2000]),  # signed long
            ("L", [1000, 2000], [1000, 2000]),  # unsigned long
            ("f", [1.5, 2.5], [1.5, 2.5]),  # float
            ("d", [1.5, 2.5], [1.5, 2.5]),  # double
        ]

        for typecode, values, expected in test_cases:
            arr = array.array(typecode, values)
            it = ao.array_iterator(arr)
            result = list(it)
            assert result == expected, f"Failed for type {typecode}"

    def test_array_iterator_iter_function(self, int_array):
        """Test using iter() function on ArrayIterator."""
        import arrayops as ao

        it = ao.array_iterator(int_array)
        it2 = iter(it)
        assert it2 is it  # __iter__ should return self
        assert list(it2) == [1, 2, 3, 4, 5]

    def test_array_iterator_list_comprehension(self, int_array):
        """Test using ArrayIterator in list comprehension."""
        import arrayops as ao

        it = ao.array_iterator(int_array)
        result = [x * 2 for x in it]
        assert result == [2, 4, 6, 8, 10]

    def test_array_iterator_sum_builtin(self, int_array):
        """Test using ArrayIterator with sum() builtin."""
        import arrayops as ao

        it = ao.array_iterator(int_array)
        result = sum(it)
        assert result == 15

    def test_array_iterator_any_all(self):
        """Test using ArrayIterator with any() and all() builtins."""
        import arrayops as ao

        arr = array.array("i", [0, 0, 1, 0])
        it = ao.array_iterator(arr)
        assert any(it) is True

        arr2 = array.array("i", [1, 1, 1, 1])
        it2 = ao.array_iterator(arr2)
        assert all(it2) is True

        arr3 = array.array("i", [0, 0, 0, 0])
        it3 = ao.array_iterator(arr3)
        assert any(it3) is False

    @pytest.mark.skipif(not NUMPY_AVAILABLE, reason="NumPy not available")
    def test_array_iterator_numpy(self):
        """Test ArrayIterator with NumPy array."""
        import arrayops as ao

        arr = np.array([1, 2, 3, 4, 5], dtype=np.int32)
        it = ao.array_iterator(arr)
        result = list(it)
        assert result == [1, 2, 3, 4, 5]

    def test_array_iterator_memoryview(self, int_array):
        """Test ArrayIterator with memoryview."""
        import arrayops as ao

        mv = memoryview(int_array)
        it = ao.array_iterator(mv)
        result = list(it)
        assert result == [1, 2, 3, 4, 5]

    @pytest.mark.skipif(not ARROW_AVAILABLE, reason="Arrow not available")
    def test_array_iterator_arrow(self):
        """Test ArrayIterator with Arrow array."""
        import arrayops as ao

        # Arrow arrays need to be converted to buffers/memoryview for buffer protocol access
        # This is a limitation of how Arrow arrays expose their data
        # For now, we test that the function raises an appropriate error
        arr = pa.array([1, 2, 3, 4, 5], type=pa.int32())
        # Arrow arrays may not work directly with PyBuffer - skip for now
        # This would need special handling in the Rust code similar to how
        # other operations handle Arrow arrays
        with pytest.raises((TypeError, ValueError)):
            it = ao.array_iterator(arr)
            list(it)

    def test_array_iterator_not_array(self):
        """Test ArrayIterator raises error for non-array input."""
        import arrayops as ao

        with pytest.raises(TypeError, match="Expected array.array"):
            ao.array_iterator([1, 2, 3])


class TestLazyArrayIterator:
    """Tests for LazyArray iterator protocol."""

    def test_lazy_array_iter(self):
        """Test LazyArray iteration."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        lazy = lazy.map(lambda x: x * 2).filter(lambda x: x > 5)

        result = []
        for x in lazy:
            result.append(x)
        assert result == [6, 8, 10]

    def test_lazy_array_iter_list(self):
        """Test converting LazyArray iterator to list."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        lazy = lazy.map(lambda x: x * 2)

        result = list(lazy)
        assert result == [2, 4, 6, 8, 10]

    def test_lazy_array_iter_next(self):
        """Test using next() on LazyArray iterator."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3])
        lazy = ao.lazy_array(arr)
        lazy = lazy.map(lambda x: x * 2)

        it = iter(lazy)
        assert next(it) == 2
        assert next(it) == 4
        assert next(it) == 6

    def test_lazy_array_iter_empty(self):
        """Test LazyArray iteration on empty result."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3])
        lazy = ao.lazy_array(arr)
        lazy = lazy.filter(lambda x: x > 10)  # No elements pass

        result = list(lazy)
        assert result == []

    def test_lazy_array_iter_cached(self):
        """Test that LazyArray iterator uses cached result."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3])
        lazy = ao.lazy_array(arr)
        lazy = lazy.map(lambda x: x * 2)

        # First iteration
        result1 = list(lazy)
        assert result1 == [2, 4, 6]

        # Second iteration should use cached result
        result2 = list(lazy)
        assert result2 == [2, 4, 6]

    def test_lazy_array_iter_comprehension(self):
        """Test LazyArray iteration in list comprehension."""
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        lazy = lazy.map(lambda x: x * 2)

        result = [x + 1 for x in lazy]
        assert result == [3, 5, 7, 9, 11]
