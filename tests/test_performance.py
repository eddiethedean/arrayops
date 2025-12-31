"""Performance regression tests for arrayops.

These tests ensure that performance doesn't regress significantly.
Note: These are timing-based tests and may be flaky on slower systems.
"""

import array
import time
import arrayops as ao


def benchmark(func, *args, iterations=100):
    """Run a function multiple times and return average time in seconds."""
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        func(*args)
        end = time.perf_counter()
        times.append(end - start)
    return sum(times) / len(times)


class TestPerformanceRegression:
    """Performance regression tests to ensure optimizations are working."""

    def test_sum_performance(self):
        """Test that sum is fast for large arrays."""
        # Use smaller values to avoid i32 overflow (sum of 0..50000 fits in i32)
        arr = array.array("i", range(50_000))

        # Should complete in reasonable time (< 0.001s for 100 iterations)
        avg_time = benchmark(ao.sum, arr, iterations=100)
        assert avg_time < 0.001, f"sum too slow: {avg_time:.6f}s per iteration"

    def test_mean_performance(self):
        """Test that mean is fast for large arrays."""
        # Use smaller values to avoid i32 overflow
        arr = array.array("i", range(50_000))

        avg_time = benchmark(ao.mean, arr, iterations=100)
        assert avg_time < 0.001, f"mean too slow: {avg_time:.6f}s per iteration"

    def test_min_max_performance(self):
        """Test that min/max are fast for large arrays."""
        arr = array.array("i", range(50_000))

        avg_time_min = benchmark(ao.min, arr, iterations=100)
        avg_time_max = benchmark(ao.max, arr, iterations=100)

        assert avg_time_min < 0.002, f"min too slow: {avg_time_min:.6f}s per iteration"
        assert avg_time_max < 0.002, f"max too slow: {avg_time_max:.6f}s per iteration"

    def test_add_performance(self):
        """Test that add is fast for large arrays."""
        arr1 = array.array("i", range(50_000))
        arr2 = array.array("i", range(1, 50_001))

        avg_time = benchmark(ao.add, arr1, arr2, iterations=50)
        assert avg_time < 0.01, f"add too slow: {avg_time:.6f}s per iteration"

    def test_sort_performance(self):
        """Test that sort is reasonably fast for large arrays."""
        # Create reverse-sorted array (worst case)
        arr = array.array("i", range(10_000, 0, -1))

        # Sort is in-place, so we need to clone for each iteration
        def sort_bench():
            arr_copy = array.array("i", arr)
            ao.sort(arr_copy)

        avg_time = benchmark(sort_bench, iterations=10)
        assert avg_time < 0.1, f"sort too slow: {avg_time:.6f}s per iteration"

    def test_small_array_fast_path(self):
        """Test that small arrays use fast paths efficiently."""
        # Small arrays should be very fast
        small_arr = array.array("i", [5, 2, 8, 1, 9])

        avg_time_sum = benchmark(ao.sum, small_arr, iterations=1000)
        avg_time_min = benchmark(ao.min, small_arr, iterations=1000)

        # Small arrays should be extremely fast (< 0.00001s per operation)
        assert avg_time_sum < 0.00001, f"small array sum too slow: {avg_time_sum:.8f}s"
        assert avg_time_min < 0.00001, f"small array min too slow: {avg_time_min:.8f}s"
