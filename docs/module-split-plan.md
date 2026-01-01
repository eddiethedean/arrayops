# Phase 5: Module Split Plan

This document outlines the plan to split the monolithic `lib.rs` (3773 lines) into logical modules to improve code organization and maintainability.

## Overview

The current `lib.rs` file contains all functionality in a single file. This plan refactors it into a modular structure with clear boundaries and logical grouping while maintaining all existing functionality.

## Module Structure

```
src/
  lib.rs              # Main module, exports, pymodule setup, macros
  types.rs            # TypeCode enum, InputType enum, type conversion functions
  validation.rs       # Input validation, type detection functions
  buffer.rs           # Buffer helper functions (get_array_len, get_itemsize, extract_element_at_index, etc.)
  operations/         # Operation implementations
    mod.rs            # Re-exports, common utilities (create_empty_result_array, etc.)
    basic.rs          # sum, scale, mean, min, max (+ their _impl functions)
    transform.rs      # map, map_inplace, filter, reduce (+ their _impl functions)
    stats.rs          # var, std_dev, median (+ their _impl functions)
    elementwise.rs    # add, multiply, clip, normalize (+ their _impl functions)
    manipulation.rs   # reverse, sort, unique (+ their _impl functions)
    slice.rs          # slice operation
  iterator.rs         # ArrayIterator implementation, array_iterator function
  lazy.rs             # (already exists)
  allocator.rs        # (already exists)
```

## Dependencies and Visibility

### Public Items (needed across modules)

- `TypeCode` enum → `pub enum` in `types.rs`
- `InputType` enum → `pub enum` in `types.rs`
- Helper functions used by macros and operations → `pub(crate)` in respective modules
- Macros → remain in `lib.rs` (macros need to be at crate root or use `#[macro_export]`)

### Key Dependencies

- Macros in `lib.rs` need access to: `TypeCode`, `get_itemsize`, `PyBuffer`
- Operations need: `TypeCode`, `InputType`, validation functions, buffer helpers
- Module registration needs: all `#[pyfunction]` functions and `#[pyclass]` structs

## Implementation Steps

### Step 1: Create types.rs module

- Move `TypeCode` enum and `impl TypeCode` block to `types.rs`
- Move type conversion functions: `get_typecode`, `get_numpy_typecode`, `get_memoryview_typecode`, `get_arrow_typecode`, `get_typecode_unified`
- Make `TypeCode` public: `pub enum TypeCode`
- Keep helper functions `pub(crate)` for internal use
- Add `mod types; pub use types::*;` to `lib.rs`

**Estimated size**: ~250 lines

### Step 2: Create validation.rs module

- Move `InputType` enum to `validation.rs` (make it `pub enum`)
- Move validation functions: `detect_input_type`, `validate_for_operation`, `validate_array_array`, `validate_numpy_array`, `validate_memoryview`, `is_memoryview_writable`, `validate_arrow_buffer`
- Make functions `pub(crate)`
- Add `mod validation; pub use validation::*;` to `lib.rs`

**Estimated size**: ~200 lines

### Step 3: Create buffer.rs module

- Move buffer helper functions: `get_array_len`, `get_itemsize`, `extract_element_at_index`
- Move array creation helpers: `create_empty_result_array`, `create_result_array_from_list`, `create_result_array_from_vec`
- Move parallel helpers: `extract_buffer_to_vec`, `should_parallelize`, `PARALLEL_THRESHOLD_*`, `CACHE_BLOCK_SIZE`
- Make functions `pub(crate)`
- Add `mod buffer; pub use buffer::*;` to `lib.rs`

**Estimated size**: ~300 lines

### Step 4: Create operations/mod.rs

- Create `operations/mod.rs`
- Re-export all operation modules: `pub mod basic; pub mod transform; ...`
- Make the `operations` module in `lib.rs`: `pub mod operations;`

**Estimated size**: ~50 lines

### Step 5: Create operations/basic.rs

- Move `sum` function and `sum_impl`
- Move `scale` function and `scale_impl`
- Move `mean` function, `mean_impl_int`, `mean_impl_float`
- Move `min` function and `min_impl`
- Move `max` function and `max_impl`
- Add necessary imports (use `crate::` paths for types, validation, buffer)
- Add `pub mod basic;` to `operations/mod.rs`

**Estimated size**: ~600 lines

### Step 6: Create operations/transform.rs

- Move `map` function and `map_impl`
- Move `map_inplace` function and `map_inplace_impl`
- Move `filter` function and `filter_impl`
- Move `reduce` function and `reduce_impl`
- Add necessary imports
- Add `pub mod transform;` to `operations/mod.rs`

**Estimated size**: ~400 lines

### Step 7: Create operations/stats.rs

- Move `var` function, `var_impl_int`, `var_impl_float`
- Move `std_dev` function
- Move `median` function, `median_impl_int`, `median_impl_float`
- Add necessary imports
- Add `pub mod stats;` to `operations/mod.rs`

**Estimated size**: ~350 lines

### Step 8: Create operations/elementwise.rs

- Move `add` function and `add_impl`
- Move `multiply` function and `multiply_impl`
- Move `clip` function and `clip_impl`
- Move `normalize` function and `normalize_impl`
- Add necessary imports
- Add `pub mod elementwise;` to `operations/mod.rs`

**Estimated size**: ~500 lines

### Step 9: Create operations/manipulation.rs

- Move `reverse` function and `reverse_impl`
- Move `sort` function and `sort_impl`
- Move `unique` function and `unique_impl`
- Add necessary imports
- Add `pub mod manipulation;` to `operations/mod.rs`

**Estimated size**: ~400 lines

### Step 10: Create operations/slice.rs

- Move `slice` function and `slice_impl`
- Add necessary imports
- Add `pub mod slice;` to `operations/mod.rs`

**Estimated size**: ~200 lines

### Step 11: Create iterator.rs module

- Move `ArrayIterator` struct (keep `#[pyclass]`)
- Move `array_iterator` function (keep `#[pyfunction]`)
- Add necessary imports
- Add `mod iterator; pub use iterator::*;` to `lib.rs`

**Estimated size**: ~150 lines

### Step 12: Update lib.rs

- Remove all moved code
- Keep macros (`dispatch_by_typecode!`, `dispatch_by_typecode_mut!`) in `lib.rs`
- Update macro definitions to use `crate::types::TypeCode` and `crate::buffer::get_itemsize`
- Keep `#[pymodule] fn _arrayops` in `lib.rs`
- Update function imports in `_arrayops` to use module paths: `operations::basic::sum`, `operations::basic::scale`, etc.
- Update class registration: `m.add_class::<iterator::ArrayIterator>()?`
- Keep `mod tests` in `lib.rs` (or consider moving to separate test files)
- Update test imports to use new module paths

**Target size**: <500 lines (macros, module declarations, pymodule registration, tests)

### Step 13: Update macro visibility

- Macros reference `TypeCode` and `get_itemsize` - these need to be accessible
- Options:
  a) Use `crate::types::TypeCode` in macros (requires updating macro definitions)
  b) Re-export types at crate root so macros can find them
- **Recommendation**: Option (b) - Keep `pub use types::*;` so macros continue working

### Step 14: Fix imports and compilation

- Add `use` statements in each module file:
  - `use pyo3::prelude::*;`
  - `use pyo3::buffer::{Element, PyBuffer};`
  - `use crate::types::{TypeCode, InputType};`
  - `use crate::validation::*;`
  - `use crate::buffer::*;`
  - Add `#[cfg(feature = "parallel")]` imports where needed
- Run `cargo check` after each step
- Fix any import errors or visibility issues

### Step 15: Update tests

- Update test imports in `lib.rs` mod tests
- Use `crate::` paths for internal functions/types
- Ensure all tests still compile and pass

## Critical Considerations

1. **Macro accessibility**: Macros must be able to reference `TypeCode` and helper functions. Solution: Re-export types at crate root with `pub use types::*;`

2. **Function visibility**: All `#[pyfunction]` functions must be accessible to `_arrayops` module registration. Use `pub` visibility for these.

3. **Incremental approach**: Move one module at a time, test compilation after each move

4. **Parallel feature gates**: Ensure `#[cfg(feature = "parallel")]` code is properly handled in each module

5. **Test module**: Consider keeping `mod tests` in `lib.rs` or move to separate integration test files. For now, keep in `lib.rs` with updated imports.

## Files to Create

- `src/types.rs` (~250 lines estimated)
- `src/validation.rs` (~200 lines estimated)
- `src/buffer.rs` (~300 lines estimated)
- `src/operations/mod.rs` (~50 lines estimated)
- `src/operations/basic.rs` (~600 lines estimated)
- `src/operations/transform.rs` (~400 lines estimated)
- `src/operations/stats.rs` (~350 lines estimated)
- `src/operations/elementwise.rs` (~500 lines estimated)
- `src/operations/manipulation.rs` (~400 lines estimated)
- `src/operations/slice.rs` (~200 lines estimated)
- `src/iterator.rs` (~150 lines estimated)

**Total new code**: ~3400 lines (same as current, just reorganized)

## Success Criteria

- ✅ All code compiles successfully
- ✅ All existing tests pass (240 Python tests)
- ✅ `lib.rs` reduced to <500 lines (macros, module declarations, pymodule registration, tests)
- ✅ Clear module boundaries with logical grouping
- ✅ No functionality changes (refactoring only)
- ✅ No performance regressions

## Risk Mitigation

- Move modules incrementally (one at a time)
- Run `cargo check` after each step
- Keep git commits after each successful module move
- Test Python import and basic functionality after major moves
- Macros are the highest risk - test thoroughly after macro visibility changes
- Run full test suite after completing each major step

## Testing Strategy

After completing the refactoring:

1. Run full test suite: `python3.12 -m pytest tests/ -v`
2. Verify all 240 tests pass
3. Run Rust unit tests: `cargo test --lib`
4. Test Python imports: `python3.12 -c "import arrayops; print('OK')"`
5. Verify all functions are accessible from Python
6. Run performance benchmarks to ensure no regressions

## Progress Tracking

This refactoring builds on the work completed in Phases 1-4:

- ✅ Phase 1: Macro System for Typecode Dispatch
- ✅ Phase 2: Buffer Operation Helpers
- ✅ Phase 3: Itemsize Handling (integrated into macros)
- ✅ Phase 4: ArrayIterator Refactoring
- 🔄 Phase 5: Module Split (this plan)

The macro system and helper functions created in previous phases make this module split cleaner and more maintainable.

