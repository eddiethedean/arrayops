"""Test all docstring examples using doctest and direct execution.

Since the docstrings are in .pyi type stub files (not executable Python),
we can't use doctest directly on them. Instead, we extract and test all
examples manually to ensure they work correctly.
"""

import array
import sys
from pathlib import Path

# Add the project root to the path
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))


def test_module_docstring_examples():
    """Test examples from arrayops/__init__.py module docstring."""
    import arrayops as ao

    # Test the module-level example
    arr = array.array("i", [1, 2, 3, 4, 5])
    result = ao.sum(arr)
    assert result == 15

    ao.scale(arr, 2.0)
    assert list(arr) == [2, 4, 6, 8, 10]


class TestSumExamples:
    """Test examples from sum() docstring."""

    def test_sum_int_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        assert ao.sum(arr) == 15

    def test_sum_float_example(self):
        import arrayops as ao

        farr = array.array("f", [1.5, 2.5, 3.5])
        result = ao.sum(farr)
        assert abs(result - 7.5) < 0.001  # Float comparison


class TestScaleExamples:
    """Test examples from scale() docstring."""

    def test_scale_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        ao.scale(arr, 2.0)
        assert list(arr) == [2, 4, 6, 8, 10]


class TestMapExamples:
    """Test examples from map() docstring."""

    def test_map_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        doubled = ao.map(arr, lambda x: x * 2)
        assert list(doubled) == [2, 4, 6, 8, 10]


class TestMapInplaceExamples:
    """Test examples from map_inplace() docstring."""

    def test_map_inplace_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        ao.map_inplace(arr, lambda x: x * 2)
        assert list(arr) == [2, 4, 6, 8, 10]


class TestFilterExamples:
    """Test examples from filter() docstring."""

    def test_filter_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5, 6])
        evens = ao.filter(arr, lambda x: x % 2 == 0)
        assert list(evens) == [2, 4, 6]


class TestReduceExamples:
    """Test examples from reduce() docstring."""

    def test_reduce_sum_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = ao.reduce(arr, lambda acc, x: acc + x)
        assert result == 15

    def test_reduce_multiply_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        result = ao.reduce(arr, lambda acc, x: acc * x, initial=1)
        assert result == 120


class TestMeanExamples:
    """Test examples from mean() docstring."""

    def test_mean_int_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        assert ao.mean(arr) == 3.0

    def test_mean_float_example(self):
        import arrayops as ao

        farr = array.array("f", [10.0, 20.0, 30.0])
        result = ao.mean(farr)
        assert abs(result - 20.0) < 0.001


class TestMinExamples:
    """Test examples from min() docstring."""

    def test_min_int_example(self):
        import arrayops as ao

        arr = array.array("i", [5, 2, 8, 1, 9])
        assert ao.min(arr) == 1

    def test_min_float_example(self):
        import arrayops as ao

        farr = array.array("f", [3.5, 1.2, 7.8, 0.5])
        assert abs(ao.min(farr) - 0.5) < 0.001


class TestMaxExamples:
    """Test examples from max() docstring."""

    def test_max_int_example(self):
        import arrayops as ao

        arr = array.array("i", [5, 2, 8, 1, 9])
        assert ao.max(arr) == 9

    def test_max_float_example(self):
        import arrayops as ao

        farr = array.array("f", [3.5, 1.2, 7.8, 0.5])
        assert abs(ao.max(farr) - 7.8) < 0.001


class TestStdExamples:
    """Test examples from std() docstring."""

    def test_std_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        std_result = ao.std(arr)
        # Population std for [1,2,3,4,5] with mean=3.0
        # sqrt(sum((x-3)^2)/5) = sqrt(10/5) = sqrt(2) ≈ 1.414
        assert abs(std_result - 1.4142135623730951) < 0.001


class TestVarExamples:
    """Test examples from var() docstring."""

    def test_var_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        assert ao.var(arr) == 2.0


class TestMedianExamples:
    """Test examples from median() docstring."""

    def test_median_odd_length_example(self):
        import arrayops as ao

        arr = array.array("i", [5, 2, 8, 1, 9])
        assert ao.median(arr) == 5

    def test_median_even_length_example(self):
        import arrayops as ao

        arr2 = array.array("i", [1, 2, 3, 4])
        assert ao.median(arr2) == 2  # Even length: lower median


class TestAddExamples:
    """Test examples from add() docstring."""

    def test_add_example(self):
        import arrayops as ao

        arr1 = array.array("i", [1, 2, 3, 4, 5])
        arr2 = array.array("i", [10, 20, 30, 40, 50])
        result = ao.add(arr1, arr2)
        assert list(result) == [11, 22, 33, 44, 55]


class TestMultiplyExamples:
    """Test examples from multiply() docstring."""

    def test_multiply_example(self):
        import arrayops as ao

        arr1 = array.array("i", [1, 2, 3, 4, 5])
        arr2 = array.array("i", [10, 20, 30, 40, 50])
        result = ao.multiply(arr1, arr2)
        assert list(result) == [10, 40, 90, 160, 250]


class TestClipExamples:
    """Test examples from clip() docstring."""

    def test_clip_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 5, 10, 15, 20])
        ao.clip(arr, 5, 15)
        assert list(arr) == [5, 5, 10, 15, 15]


class TestNormalizeExamples:
    """Test examples from normalize() docstring."""

    def test_normalize_example(self):
        import arrayops as ao

        farr = array.array("f", [10.0, 20.0, 30.0, 40.0, 50.0])
        ao.normalize(farr)
        result = list(farr)
        expected = [0.0, 0.25, 0.5, 0.75, 1.0]
        for i, (r, e) in enumerate(zip(result, expected)):
            assert abs(r - e) < 0.001, f"Index {i}: {r} != {e}"


class TestReverseExamples:
    """Test examples from reverse() docstring."""

    def test_reverse_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        ao.reverse(arr)
        assert list(arr) == [5, 4, 3, 2, 1]


class TestSortExamples:
    """Test examples from sort() docstring."""

    def test_sort_example(self):
        import arrayops as ao

        arr = array.array("i", [5, 2, 8, 1, 9, 3])
        ao.sort(arr)
        assert list(arr) == [1, 2, 3, 5, 8, 9]


class TestUniqueExamples:
    """Test examples from unique() docstring."""

    def test_unique_example(self):
        import arrayops as ao

        arr = array.array("i", [5, 2, 8, 2, 1, 5, 9])
        unique_arr = ao.unique(arr)
        assert list(unique_arr) == [1, 2, 5, 8, 9]


class TestSliceExamples:
    """Test examples from slice() docstring."""

    def test_slice_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        view = ao.slice(arr, 1, 4)
        assert list(view) == [2, 3, 4]
        arr[2] = 99  # Modify original
        assert list(view) == [2, 99, 4]  # View reflects the change


class TestLazyArrayExamples:
    """Test examples from lazy_array() and LazyArray docstrings."""

    def test_lazy_array_basic_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        result = lazy.map(lambda x: x * 2).filter(lambda x: x > 5).collect()
        assert list(result) == [6, 8, 10]

    def test_lazy_array_map_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3])
        lazy = ao.lazy_array(arr)
        lazy = lazy.map(lambda x: x * 2)
        result = lazy.collect()
        assert list(result) == [2, 4, 6]

    def test_lazy_array_filter_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        lazy = lazy.filter(lambda x: x > 2)
        result = lazy.collect()
        assert list(result) == [3, 4, 5]

    def test_lazy_array_collect_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        lazy = lazy.map(lambda x: x * 2).filter(lambda x: x > 5)
        result = lazy.collect()
        assert list(result) == [6, 8, 10]

    def test_lazy_array_source_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3, 4, 5])
        lazy = ao.lazy_array(arr)
        source = lazy.source()
        assert list(source) == [1, 2, 3, 4, 5]

    def test_lazy_array_len_example(self):
        import arrayops as ao

        arr = array.array("i", [1, 2, 3])
        lazy = ao.lazy_array(arr)
        assert lazy.len() == 0
        lazy = lazy.map(lambda x: x * 2)
        assert lazy.len() == 1
        lazy = lazy.filter(lambda x: x > 2)
        assert lazy.len() == 2
