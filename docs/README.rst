Documentation for arrayops
===========================

This directory contains the source files for the arrayops documentation.

Building the Documentation
---------------------------

To build the documentation locally:

1. Install documentation dependencies:

   .. code-block:: bash

      pip install -r requirements-docs.txt

2. Build the documentation:

   .. code-block:: bash

      cd docs
      make html

   Or on Windows:

   .. code-block:: batch

      cd docs
      make.bat html

3. View the documentation:

   Open ``docs/_build/html/index.html`` in your web browser.

Documentation Structure
-----------------------

The documentation is organized following the Diátaxis framework:

- **Getting Started** (``getting-started/``): Tutorials for new users
- **User Guide** (``user-guide/``): How-to guides and reference for users
- **Developer Guide** (``developer-guide/``): Information for contributors
- **Reference** (``reference/``): Reference materials (changelog, roadmap)

Source Files
------------

The documentation source files are primarily in Markdown (``.md``) format, processed by Sphinx using the MyST parser. The main entry point is ``index.rst``.

Documentation files are organized as:

- ``api.md`` - Complete API reference
- ``examples.md`` - Usage examples and cookbook
- ``performance.md`` - Performance guide
- ``troubleshooting.md`` - Common issues and solutions
- ``contributing.md`` - Contributing guide
- ``development.md`` - Development setup and workflow
- ``design.md`` - Architecture and design document
- ``coverage.md`` - Code coverage methodology
- ``CHANGELOG.md`` - Version history
- ``ROADMAP.md`` - Project roadmap

Read the Docs
-------------

The documentation is configured for Read the Docs. The configuration is in ``.readthedocs.yaml`` in the project root.

To publish to Read the Docs:

1. Connect your GitHub repository to Read the Docs
2. The documentation will be built automatically on commits
3. Documentation will be available at ``https://arrayops.readthedocs.io/``

