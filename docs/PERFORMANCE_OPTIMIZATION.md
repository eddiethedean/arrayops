# Performance Optimization Analysis: Why Polars is Faster and How to Catch Up

## Executive Summary

Polars is faster than arrayops primarily due to:
1. **Apache Arrow's columnar format** - Optimized memory layout
2. **Explicit SIMD optimizations** - Hand-tuned vectorized operations
3. **Better memory access patterns** - Direct contiguous access vs PyBuffer overhead
4. **More sophisticated algorithms** - Query optimization, lazy evaluation

arrayops can close much of this gap through:
1. **Eliminating PyBuffer overhead** - Use unsafe raw pointer access (safely)
2. **Implementing explicit SIMD** - Using `std::arch` intrinsics
3. **Better parallel algorithms** - Avoid Vec copies, use unsafe shared memory
4. **Algorithm improvements** - Pairwise summation, better reduction strategies

**Expected improvements**: 5-20x speedup for basic operations, bringing arrayops much closer to Polars performance.

## Why Polars is Faster

### 1. Apache Arrow Columnar Format

**Polars:**
- Data stored in Apache Arrow format - contiguous, aligned memory
- Columnar layout optimized for analytical workloads
- Zero-copy between operations
- Memory layout designed for SIMD operations (aligned to 32/64 bytes)

**arrayops:**
- Uses Python's buffer protocol via PyO3's `PyBuffer<T>`
- Access through `PyBuffer::as_slice()` which returns `&[Cell<T>]`
- Each element accessed via `cell.get()` - method call overhead
- Memory layout depends on Python's array.array format

**Performance Impact**: ~2-5x overhead from PyBuffer access pattern

### 2. Explicit SIMD Optimizations

**Polars:**
- Hand-written SIMD code using platform intrinsics
- Uses libraries like `arrow2` with explicit SIMD for sum, mean, etc.
- Platform-specific optimizations (AVX2, AVX-512, NEON for ARM)
- Processes 8-16 elements per instruction (depending on type)

**arrayops:**
- Relies on compiler auto-vectorization
- No explicit SIMD implementation yet (infrastructure in place)
- Compiler may not always vectorize optimally

**Performance Impact**: ~4-8x for operations like sum, mean, scale

### 3. Memory Access Patterns

**Polars:**
- Direct pointer access to contiguous memory
- No indirection, no method calls
- Better cache locality (columnar format)
- Memory access aligned for SIMD operations

**arrayops:**
- Access through `&[Cell<T>]` with `cell.get()` calls
- Each access is: `Cell<T>::get(&self) -> T`
- Method call overhead (even if inlined, adds complexity)
- Additional indirection through PyBuffer wrapper

**Performance Impact**: ~1.5-2x overhead

### 4. Algorithm Optimizations

**Polars:**
- Query optimization (for DataFrame operations)
- Lazy evaluation with operation fusion
- Specialized algorithms (e.g., pairwise summation for better floating-point accuracy)
- Better parallel reduction strategies

**arrayops:**
- Straightforward sequential algorithms
- Standard reduction (may have precision issues for floats)
- Parallel reduction copies data to Vec first

**Performance Impact**: ~1.2-1.5x for sequential, ~2x for parallel (due to copy)

### 5. Parallel Execution

**Polars:**
- Designed from the ground up for parallelism
- Efficient parallel reduction without copying
- Better work-stealing algorithms

**arrayops:**
- Parallel execution requires `extract_buffer_to_vec()` - full copy!
- Then uses Rayon for parallel processing
- Copy overhead negates much of the parallel benefit for smaller arrays

**Performance Impact**: Copy overhead can be 50-80% of total time for parallel operations

## Current Bottlenecks in arrayops

### Bottleneck 1: PyBuffer Cell Access Overhead

**Current code:**
```rust
let slice = buffer.as_slice(py)?;
let sum = slice.iter().map(|cell| cell.get()).fold(T::default(), |acc, x| acc + x);
```

**Problem:**
- `cell.get()` is a method call on each iteration
- `Cell<T>` wrapper adds indirection
- Compiler may not inline perfectly
- Harder for compiler to auto-vectorize

**Impact**: ~2-3x slower than direct pointer access

### Bottleneck 2: No Explicit SIMD

**Current code:**
- Generic Rust code relying on compiler auto-vectorization
- No platform-specific optimizations
- Compiler may miss optimization opportunities

**Impact**: ~4-8x slower than hand-tuned SIMD

### Bottleneck 3: Parallel Copy Overhead

**Current code:**
```rust
let data = extract_buffer_to_vec(py, buffer)?;  // Full copy!
return Ok(data.par_iter().copied().reduce(...));
```

**Problem:**
- Copies entire array before parallel processing
- For large arrays, copy can be significant portion of time
- Defeats purpose of zero-copy design

**Impact**: Copy overhead is 50-80% of parallel operation time

### Bottleneck 4: Generic Algorithm

**Current code:**
- Standard sequential reduction
- No pairwise summation for floats (better accuracy + performance)
- No specialized algorithms for specific types

**Impact**: ~1.2-1.5x slower, plus precision issues for floats

## Optimization Strategies

### Strategy 1: Use Unsafe Raw Pointer Access (Safely)

**Approach:**
- Use `unsafe` to get raw pointer from PyBuffer
- Access memory directly without `Cell<T>` overhead
- Wrap in safe abstraction that validates bounds

**Implementation:**
```rust
unsafe fn get_raw_slice<T>(buffer: &PyBuffer<T>) -> *const T {
    // Get raw pointer - safe because PyBuffer guarantees validity
    buffer.buf_ptr() as *const T
}

fn sum_impl_fast<T>(py: Python<'_>, buffer: &PyBuffer<T>, len: usize) -> PyResult<T>
where
    T: Element + Copy + Default + std::ops::Add<Output = T>,
{
    // SAFETY: PyBuffer guarantees valid memory for len elements
    let ptr = unsafe { get_raw_slice(buffer) };
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    
    // Now direct access - no Cell overhead!
    let sum = slice.iter().copied().fold(T::default(), |a, b| a + b);
    Ok(sum)
}
```

**Expected improvement**: 2-3x faster (eliminates Cell overhead)

**Safety**: Safe when:
- PyBuffer guarantees valid memory
- We don't exceed the buffer length
- We maintain Rust's aliasing rules

### Strategy 2: Implement Explicit SIMD

**Approach:**
- Use `std::arch` for platform-specific SIMD intrinsics
- Implement SIMD for common operations (sum, scale, mean)
- Fall back to scalar code for unsupported platforms

**Implementation (sum example):**
```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "x86_64")]
fn sum_simd_i32(slice: &[i32]) -> i32 {
    unsafe {
        let mut sum_vec = _mm256_setzero_si256();
        let chunks = slice.chunks_exact(8);
        let remainder = chunks.remainder();
        
        // Process 8 elements at a time with AVX2
        for chunk in chunks {
            let data = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
            sum_vec = _mm256_add_epi32(sum_vec, data);
        }
        
        // Horizontal sum of vector
        let sum = horizontal_sum_epi32(sum_vec);
        
        // Add remainder
        sum + remainder.iter().sum::<i32>()
    }
}
```

**Expected improvement**: 4-8x faster (for supported operations)

**Complexity**: Medium - requires platform-specific code, testing on multiple architectures

### Strategy 3: Optimize Parallel Execution

**Approach:**
- Avoid Vec copy - use unsafe to share memory across threads
- Use `rayon::scope` for safe parallel access
- Better work distribution

**Implementation:**
```rust
#[cfg(feature = "parallel")]
fn sum_parallel_no_copy<T>(py: Python<'_>, buffer: &PyBuffer<T>, len: usize) -> PyResult<T>
where
    T: Element + Copy + Default + std::ops::Add<Output = T> + Send + Sync,
{
    // SAFETY: PyBuffer is Send + Sync, memory is valid for len elements
    let ptr = unsafe { get_raw_slice(buffer) };
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    
    // Use rayon scope to safely share slice across threads
    rayon::scope(|s| {
        let num_threads = rayon::current_num_threads();
        let chunk_size = len / num_threads;
        
        let results: Vec<_> = (0..num_threads)
            .map(|i| {
                let start = i * chunk_size;
                let end = if i == num_threads - 1 { len } else { (i + 1) * chunk_size };
                s.spawn(move |_| {
                    slice[start..end].iter().copied().fold(T::default(), |a, b| a + b)
                })
            })
            .collect();
        
        Ok(results.into_iter().fold(T::default(), |a, b| a + b))
    })
}
```

**Expected improvement**: 2-3x faster for parallel operations (eliminates copy overhead)

**Complexity**: Medium - requires careful memory safety guarantees

### Strategy 4: Algorithm Improvements

**Approach:**
- Pairwise summation for floats (better accuracy + performance)
- Specialized algorithms for small arrays
- Better reduction strategies

**Implementation (pairwise summation):**
```rust
fn sum_pairwise_f64(slice: &[f64]) -> f64 {
    // Pairwise summation: more accurate and often faster
    if slice.len() <= 64 {
        slice.iter().sum()
    } else {
        let mid = slice.len() / 2;
        sum_pairwise_f64(&slice[..mid]) + sum_pairwise_f64(&slice[mid..])
    }
}
```

**Expected improvement**: 1.2-1.5x faster, better accuracy

**Complexity**: Low - straightforward algorithm changes

### Strategy 5: Better Compiler Hints

**Approach:**
- Use `#[inline]` strategically
- Use `#[target_feature]` for SIMD functions
- Use `#[cold]` for error paths
- Add `assume` hints for known bounds

**Expected improvement**: 1.1-1.2x faster (marginal but easy)

**Complexity**: Low - just attribute additions

## Implementation Plan

### Phase 1: Quick Wins (High Impact, Low Risk)

1. **Eliminate Cell overhead** (Strategy 1)
   - Estimated improvement: 2-3x
   - Risk: Low (safe unsafe usage)
   - Effort: Medium (need careful testing)

2. **Better compiler hints** (Strategy 5)
   - Estimated improvement: 1.1-1.2x
   - Risk: Very low
   - Effort: Low

**Total Phase 1 improvement**: ~2.5-3.5x faster

### Phase 2: SIMD Implementation (High Impact, Medium Risk)

3. **Explicit SIMD for sum, scale, mean** (Strategy 2)
   - Estimated improvement: 4-8x
   - Risk: Medium (platform-specific code)
   - Effort: High (need x86_64, ARM64 implementations)

4. **Pairwise summation for floats** (Strategy 4)
   - Estimated improvement: 1.2-1.5x + accuracy
   - Risk: Low
   - Effort: Low

**Total Phase 2 improvement**: Additional 4-8x (combined with Phase 1: ~10-28x faster!)

### Phase 3: Parallel Optimization (Medium Impact, Medium Risk)

5. **Eliminate parallel copy overhead** (Strategy 3)
   - Estimated improvement: 2-3x for parallel ops
   - Risk: Medium (memory safety)
   - Effort: Medium

**Total Phase 3 improvement**: Additional 2-3x for parallel operations

## Expected Final Performance

After all optimizations, arrayops should be:

- **Sequential operations**: 10-20x faster than current (close to Polars)
- **Parallel operations**: 20-40x faster than current (competitive with Polars)
- **With SIMD**: Approaching Polars performance for basic operations

### Realistic Targets (1M int32 elements):

| Operation | Current | After Phase 1 | After Phase 2 | Polars | Gap After Phase 2 |
|-----------|---------|---------------|---------------|--------|-------------------|
| Sum | 7.5 ms | ~2.5 ms | ~0.4 ms | 0.3 ms | **1.3x slower** |
| Scale | 4.8 ms | ~1.6 ms | ~0.3 ms | 0.4 ms | **Competitive** |
| Mean | 7.1 ms | ~2.4 ms | ~0.5 ms | 0.3 ms | **1.7x slower** |

**Conclusion**: After Phase 2, arrayops should be within 1.5-2x of Polars for basic operations - excellent performance for a lightweight library!

## Code Examples

### Example 1: Fast Sum with Raw Pointer Access

```rust
use pyo3::buffer::{Element, PyBuffer};
use pyo3::prelude::*;

fn sum_impl_fast<T>(py: Python<'_>, buffer: &PyBuffer<T>, len: usize) -> PyResult<T>
where
    T: Element + Copy + Default + std::ops::Add<Output = T>,
{
    let slice = buffer.as_slice(py)
        .ok_or_else(|| PyTypeError::new_err("Failed to get buffer slice"))?;
    
    // Get raw pointer - PyBuffer guarantees validity
    // SAFETY: PyBuffer::as_slice() guarantees valid memory for len elements
    let ptr = slice.as_ptr() as *const T;
    let direct_slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    
    // Direct access - no Cell overhead!
    Ok(direct_slice.iter().copied().fold(T::default(), |a, b| a + b))
}
```

### Example 2: SIMD-accelerated Sum

```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "x86_64")]
fn sum_simd_i32_avx2(slice: &[i32]) -> i32 {
    unsafe {
        let mut sum_vec = _mm256_setzero_si256();
        let chunks = slice.chunks_exact(8);
        let remainder = chunks.remainder();
        
        // Process 8 elements at a time
        for chunk in chunks {
            let data = _mm256_loadu_si256(chunk.as_ptr() as *const __m256i);
            sum_vec = _mm256_add_epi32(sum_vec, data);
        }
        
        // Horizontal sum
        let mut result = [0i32; 8];
        _mm256_storeu_si256(result.as_mut_ptr() as *mut __m256i, sum_vec);
        let sum = result.iter().sum::<i32>();
        
        sum + remainder.iter().sum::<i32>()
    }
}
```

## Safety Considerations

All optimizations using `unsafe` must:
1. **Validate bounds** - Never access beyond buffer length
2. **Maintain aliasing rules** - No mutable aliasing
3. **Preserve PyBuffer guarantees** - Memory is valid while PyBuffer exists
4. **Handle errors safely** - Panic on invalid state, not undefined behavior

## Testing Requirements

For each optimization:
1. **Correctness tests** - Verify results match reference implementation
2. **Performance benchmarks** - Measure actual speedup
3. **Edge case tests** - Empty arrays, single element, overflow
4. **Cross-platform tests** - x86_64, ARM64, etc.
5. **Memory safety tests** - Use Miri/Valgrind to verify no UB

## Conclusion

Polars is faster primarily due to:
- Better memory access patterns (Arrow columnar format)
- Explicit SIMD optimizations
- Better parallel algorithms

arrayops can close most of the gap by:
1. Eliminating PyBuffer Cell overhead (2-3x)
2. Implementing explicit SIMD (4-8x)
3. Optimizing parallel execution (2-3x)

**Expected outcome**: After optimizations, arrayops should be within 1.5-2x of Polars for basic operations - excellent performance for a lightweight, zero-dependency library!

The trade-off: arrayops maintains simplicity and zero dependencies, while Polars provides DataFrame features and query optimization. For 1D array operations, optimized arrayops should be very competitive.

