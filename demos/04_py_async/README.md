# Python Async

1. Install Linux packages.

    ```bash
    apt install -y pkg-config libssl-dev
    ```

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

1. Run the `app.py` Python script.

    ```bash
    python ./app.py
    ```

1. Uninstall the `py_async` development package.

    ```bash
    python -m pip uninstall py_async
    ```

1. Build a production release of the `py_async` extension.

    ```bash
    maturin build --release
    ```

    Look for the path to the wheel file "*.whl" to install production build. Copy it.

1. Install the production build of the `py_async` package.

    ```bash
    python -m pip install <COPIED WHEEL PATH>
    ```

1. Run the `app.py` Python script.

    ```bash
    python ./app.py
    ```

1. When you are done working with the demo, deactivate the virtual environment.

    ```bash
    deactivate
    ```
