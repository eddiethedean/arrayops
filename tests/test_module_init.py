"""Tests for module initialization and imports."""


class TestModuleInitialization:
    """Tests for module initialization."""

    def test_module_imports_successfully(self):
        """Test that the module imports successfully."""
        import arrayops

        assert arrayops is not None
        assert hasattr(arrayops, "__version__")
        assert arrayops.__version__ == "1.0.0"

    def test_module_imports_all_functions(self):
        """Test that all expected functions are imported."""
        import arrayops

        expected_functions = [
            "sum",
            "scale",
            "map",
            "map_inplace",
            "filter",
            "reduce",
            "mean",
            "min",
            "max",
            "std",
            "var",
            "median",
            "add",
            "multiply",
            "clip",
            "normalize",
            "reverse",
            "sort",
            "unique",
            "slice",
            "array_iterator",
            "ArrayIterator",
            "lazy_array",
            "LazyArray",
        ]

        for func_name in expected_functions:
            assert hasattr(arrayops, func_name), f"Missing function: {func_name}"

    def test_module_all_list(self):
        """Test that __all__ contains all expected functions."""
        import arrayops

        expected_functions = [
            "sum",
            "scale",
            "map",
            "map_inplace",
            "filter",
            "reduce",
            "mean",
            "min",
            "max",
            "std",
            "var",
            "median",
            "add",
            "multiply",
            "clip",
            "normalize",
            "reverse",
            "sort",
            "unique",
            "slice",
            "array_iterator",
            "ArrayIterator",
            "lazy_array",
            "LazyArray",
        ]

        assert set(arrayops.__all__) == set(expected_functions)

    def test_module_docstring(self):
        """Test that module has a docstring."""
        import arrayops

        assert arrayops.__doc__ is not None
        assert "arrayops" in arrayops.__doc__.lower()
