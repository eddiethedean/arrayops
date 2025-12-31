# Design Document

**Project:** arrayops

Rust-backed acceleration for Python array.array

---

## 1. Purpose & Goals

### Problem

Python's built-in `array.array`:
- Is memory-efficient and C-compatible
- Supports zero-copy buffers
- Lacks high-level operations (map, filter, reduce, math ops)
- Is slow when iterated in Python

NumPy solves this but is:
- Heavyweight
- Multi-dimensional (often unnecessary)
- Overkill for scripting, ETL, and systems tooling

### Goal

Create a lightweight Rust extension that:
- Operates directly on `array.array`
- Uses zero-copy buffer access
- Provides fast, safe, numeric operations
- Avoids introducing a new array type

### Non-Goals
- Replacing NumPy
- Multi-dimensional arrays
- Arbitrary Python object arrays
- Dynamic method injection into `array.array`

---

## 2. Target Users
- Systems / ETL Python developers
- Binary protocol authors
- Streaming data pipelines
- Performance-sensitive scripts
- Users who want speed without NumPy

---

## 3. High-Level Architecture

```
┌──────────────┐
│   Python     │
│              │
│ array.array  │  ← unchanged
│ arrayops/_arrayops.py │
└──────┬───────┘
       │ buffer protocol
       ▼
┌──────────────┐
│    Rust      │
│  (PyO3)      │
│              │
│ typed loops  │
│ SIMD / par   │
└──────────────┘
```

---

## 4. Public Python API

### Module layout

```python
import arrayops
```

### Core operations

```
arrayops.map(arr, fn)                -> array
arrayops.map_inplace(arr, fn)        -> None

arrayops.filter(arr, predicate)      -> array

arrayops.reduce(arr, fn, initial)    -> scalar

arrayops.sum(arr)                    -> scalar
arrayops.mean(arr)                   -> float
arrayops.min(arr), arrayops.max(arr)

arrayops.scale(arr, factor)          -> None
arrayops.clip(arr, min, max)         -> None
```

### Type restrictions
- Only numeric `array.array` types:
  - `b`, `B`, `h`, `H`, `i`, `I`, `l`, `L`
  - `f`, `d`
- No object arrays
- No mixed types

Invalid typecodes → `TypeError`

---

## 5. Rust Implementation Strategy

### Tooling
- Rust
- PyO3
- maturin
- Optional: rayon (parallelism), packed_simd or std::simd

### Buffer access

```rust
use pyo3::buffer::PyBuffer;

let buffer = PyBuffer::<i32>::get(py, &py_array)?;
let slice = unsafe { buffer.as_slice()? };
```

- ✔️ Zero-copy
- ✔️ Typed
- ✔️ Safe lifetime enforcement

### Type dispatch

Rust side dispatch by typecode:

```rust
match typecode {
    'i' => process::<i32>(buffer),
    'f' => process::<f32>(buffer),
    'd' => process::<f64>(buffer),
    _ => Err(TypeError)
}
```

Each kernel is monomorphized → fast loops.

---

## 6. Python Callable Handling

### Two execution paths

#### A. Fast path (no Python calls)

```python
arrayops.scale(arr, 1.5)
arrayops.sum(arr)
```

- Pure Rust loop
- No GIL per element
- SIMD-friendly

#### B. Callback path (slower, flexible)

```python
arrayops.map(arr, lambda x: x * x)
```

- GIL held
- Python callable invoked per element
- Still faster than Python iteration due to C-level loop

---

## 7. Safety Guarantees
- Bounds-checked slices
- No unsafe pointer arithmetic exposed
- No reallocation during in-place ops
- Panic boundaries converted to Python exceptions

---

## 8. Performance Characteristics

| Operation | Python list | array.array | arrayops |
|-----------|-------------|-------------|-----------|
| Iteration | Fast | Slower | ❌ (avoid) |
| Sum | Slow | Slow | 🚀 |
| Map | Slow | Slow | 🚀 |
| Binary IO | ❌ | ✅ | ✅ |
| In-place ops | ❌ | ✅ | 🚀 |

---

## 9. Error Handling

| Condition | Behavior |
|-----------|----------|
| Unsupported typecode | TypeError |
| Non-contiguous buffer | BufferError |
| Python callback error | Propagate exception |
| Overflow (ints) | Python semantics |

---

## 10. Packaging & Distribution

### Build
```
maturin build
maturin publish
```

### Targets
- Linux (manylinux)
- macOS (universal2)
- Windows

---

## 11. Testing Strategy

### Rust
- Unit tests per kernel
- Property tests (quickcheck)
- Overflow behavior validation

### Python
- pytest
- Parity tests vs Python list
- Buffer safety tests
- Large array stress tests

---

## 12. Documentation Plan
- README with:
  - Motivation
  - API examples
  - Performance benchmarks
  - Comparison with NumPy
- Cookbook examples:
  - Binary parsing
  - ETL pipelines
  - Streaming stats

---

## 13. Future Extensions
- SIMD auto-vectorization
- Parallel execution (rayon)
- memoryview support
- Optional NumPy interop
- Arrow buffer interop

---

## 14. Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Reinventing NumPy | Scope control |
| Callback overhead | Encourage fast paths |
| API creep | Minimal surface |
| Platform builds | maturin CI |

---

## 15. Summary

arrayops would:
- Fill a real gap between `array.array` and NumPy
- Leverage Rust for safety + speed
- Enable high-performance numeric scripts with zero dependencies

