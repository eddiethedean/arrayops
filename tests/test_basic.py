"""Basic tests for arrayops package."""

import array
import pytest


@pytest.fixture
def int_array():
    """Create a test integer array."""
    return array.array('i', [1, 2, 3, 4, 5])


@pytest.fixture
def float_array():
    """Create a test float array."""
    return array.array('d', [1.5, 2.5, 3.5, 4.5])


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
        arr = array.array('i', [])
        result = arrayops.sum(arr)
        assert result == 0

    def test_sum_single_element(self):
        """Test sum of single element array."""
        import arrayops
        arr = array.array('i', [42])
        result = arrayops.sum(arr)
        assert result == 42

    def test_sum_all_numeric_types(self):
        """Test sum works with all supported numeric types."""
        import arrayops
        
        # Test all supported integer types
        test_cases = [
            ('b', [-1, 0, 1], 0),  # signed char
            ('B', [1, 2, 3], 6),   # unsigned char
            ('h', [-100, 0, 100], 0),  # signed short
            ('H', [100, 200], 300),    # unsigned short
            ('i', [1, 2, 3], 6),       # signed int
            ('I', [1, 2, 3], 6),       # unsigned int
            ('l', [1000, 2000], 3000), # signed long
            ('L', [1000, 2000], 3000), # unsigned long
            ('f', [1.5, 2.5], 4.0),    # float
            ('d', [1.5, 2.5], 4.0),    # double
        ]
        
        for typecode, values, expected in test_cases:
            arr = array.array(typecode, values)
            result = arrayops.sum(arr)
            assert result == expected, f"Failed for type {typecode}"

    def test_sum_large_array(self):
        """Test sum with large array."""
        import arrayops
        arr = array.array('i', list(range(10000)))
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
        # 'c' is char type, not supported
        arr = array.array('c', b'abc')
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
        arr = array.array('i', [])
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
        arr = array.array('i', [1, 2, 3])
        arrayops.scale(arr, -1.0)
        assert list(arr) == [-1, -2, -3]

    def test_scale_all_numeric_types(self):
        """Test scale works with all supported numeric types."""
        import arrayops
        
        test_cases = [
            ('i', [1, 2, 3], 2.0, [2, 4, 6]),
            ('f', [1.0, 2.0], 1.5, [1.5, 3.0]),
            ('d', [1.0, 2.0], 2.5, [2.5, 5.0]),
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
        arr = array.array('c', b'abc')
        with pytest.raises(TypeError, match="Unsupported typecode"):
            arrayops.scale(arr, 2.0)


class TestParity:
    """Tests comparing arrayops results to Python equivalents."""

    def test_sum_parity(self):
        """Test sum results match Python built-in sum."""
        import arrayops
        test_data = [1, 2, 3, 4, 5, 10, 20, 30]
        arr = array.array('i', test_data)
        python_sum = sum(test_data)
        arrayops_sum = arrayops.sum(arr)
        assert arrayops_sum == python_sum

    def test_sum_parity_float(self):
        """Test float sum results match Python built-in sum."""
        import arrayops
        test_data = [1.5, 2.5, 3.5, 4.5]
        arr = array.array('d', test_data)
        python_sum = sum(test_data)
        arrayops_sum = arrayops.sum(arr)
        assert abs(arrayops_sum - python_sum) < 1e-10

