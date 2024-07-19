# Python Rust Parallelism

1. Create a virtual environment.

    ```bash
    python -m venv venv
    ```

1. Activate the virtual environment.

    ```bash
    source ./venv/bin/activate
    ```

1. Install maturin package.

    ```bash
    python -m pip install maturin==1.5.1 pytest pytest-benchmark
    ```

1.  Build the project.

    ```bash
    maturin dev
    ```

1. Run the PyTest tests.

    ```bash
    pytest ./tests/test_word_count.py
    ```

1. Uninstall the `py_parallelism` development package.

    ```bash
    python -m pip uninstall py_parallelism
    ```

1. Build a production release of the `py_parallelism` extension.

    ```bash
    maturin build --release
    ```

    Look for the path to the wheel file "*.whl" to install production build. Copy it.

1. Install the production build of the `py_parallelism` package.

    ```bash
    python -m pip install <COPIED WHEEL PATH>
    ```

1. Run the PyTest tests.

    ```bash
    pytest ./tests/test_word_count.py
    ```

1. When you are done working with the demo, deactivate the virtual environment.

    ```bash
    deactivate
    ```
