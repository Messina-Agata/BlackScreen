This program opens a fullscreen window on every monitor and fills it entirely with black. All screens remain black until the user clicks, at which point the program closes and normal display is restored.

It is available in two versions:
- Python version (`black_screen.py`):
    - Requires `Python 3` with `Tkinter` and `screeninfo` library
- Rust version (inside `black_screen` folder)
    - If you are on Windows 10 or 11 (64-bit), you can directly run the binary file in the release (`black_screen.exe`)
    - Otherwise, build the program for your OS using `Rust` and `Cargo` 