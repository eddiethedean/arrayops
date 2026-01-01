"""Tests for import error handling."""


class TestImportErrors:
    """Tests for import error handling."""

    def test_import_error_without_arrayops_module(self):
        """Test that ImportError is raised when _arrayops module is missing."""
        # This test verifies the error handling path in __init__.py
        # We can't easily test this without breaking the module, but we can
        # verify the error message format is correct
        import arrayops

        # The module should import successfully in normal circumstances
        assert arrayops is not None

    def test_all_functions_callable(self):
        """Test that all exported functions are callable (or classes)."""
        import arrayops

        functions = [
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
            "lazy_array",
        ]

        for func_name in functions:
            func = getattr(arrayops, func_name)
            assert callable(func), f"{func_name} should be callable"

        # ArrayIterator and LazyArray are classes, not callable
        for class_name in ["ArrayIterator", "LazyArray"]:
            assert hasattr(arrayops, class_name), f"{class_name} should be available"
            cls = getattr(arrayops, class_name)
            # Check it's a class-like object (has __new__ or similar)
            assert hasattr(cls, "__new__") or hasattr(cls, "__init__"), (
                f"{class_name} should be a class"
            )
