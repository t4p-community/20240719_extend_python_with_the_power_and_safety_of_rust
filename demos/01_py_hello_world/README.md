# Python Rust Extension Hello World

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
    python -m pip install maturin==1.5.1
    ```

1.  Build the project.

    ```bash
    maturin dev
    ```

1. Run the Python REPL environment.

    ```bash
    python
    ```

1. Import the Python Extension.

    ```python
    from py_hello_world import sum_as_string
    ```

1. Call the function `sum_as_string` function.

    ```python
    sum_as_string(1, 2)
    ```

    The result should be "3".

1. When you are done working with the demo, deactivate the virtual environment.

    ```bash
    deactivate
    ```