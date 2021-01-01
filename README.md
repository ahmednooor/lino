<div align="center">
    <span align="center"><img src="https://raw.githubusercontent.com/ahmednooor/lino/main/lino-logo.png" alt="lino" width="240" height="143" class="center" /></span>
    <h3 align="center">A command line text editor with notepad like key bindings. [WIP]</h3>
    <hr />
    <span align="center"><img src="https://raw.githubusercontent.com/ahmednooor/lino/main/lino-screenshot.png" alt="screenshot" class="center" /></span>
    <hr />
</div>

### Usage (Key bindings)
| Key | Function |
| --- | --- |
| <kbd>↑</kbd> , <kbd>↓</kbd> , <kbd>←</kbd> , <kbd>→</kbd> | `Move` |
| <kbd>Ctrl</kbd> + <kbd>←</kbd> , <kbd>→</kbd> | `Move by word` |
| <kbd>Page Up</kbd> , <kbd>Page Down</kbd> | `Move by page (scrolling)` |
| <kbd>Home</kbd> | `Move to line start` |
| <kbd>End</kbd> | `Move to line end` |
| <kbd>Shift</kbd> + <kbd>↑</kbd> , <kbd>↓</kbd> , <kbd>←</kbd> , <kbd>→</kbd> | `Select` |
| <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>←</kbd> , <kbd>→</kbd> | `Select by word` |
| <kbd>Shift</kbd> + <kbd>Page Up</kbd> , <kbd>Page down</kbd> | `Select by page` |
| <kbd>Shift</kbd> + <kbd>Home</kbd> | `Select upto line start` |
| <kbd>Shift</kbd> + <kbd>End</kbd> | `Select upto line end` |
| <kbd>Backspace</kbd> | `Delete backwards` |
| <kbd>Del</kbd> | `Delete forwards` |
| <kbd>Ctrl</kbd> + <kbd>X</kbd> | `Cut` |
| <kbd>Ctrl</kbd> + <kbd>C</kbd> | `Copy` |
| <kbd>Ctrl</kbd> + <kbd>V</kbd> | `Paste` |
| <kbd>Ctrl</kbd> + <kbd>Z</kbd> | `Undo` |
| <kbd>Ctrl</kbd> + <kbd>Y</kbd> | `Redo` |
| <kbd>Ctrl</kbd> + <kbd>S</kbd> | `Save` |
| <kbd>Ctrl</kbd> + <kbd>Q</kbd> | `Quit` |

### Building/Running

**You will need to have Rust (2018 or higher), installed on your system before proceeding. Install it from; [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)**

1. **Clone this repo**
    ```sh
    git clone https://github.com/ahmednooor/lino.git
    ```
2. **Go into the loned repo directory**
    ```sh
    cd ./lino/
    ```
3. **Run with cargo**
    ```sh
    cargo run <optional-filename>
    ```

On `Linux (Debian based)`, if you head into problems, try installing the following libraries and re run with cargo.

```sh
sudo apt install xorg-dev libxcb-present-dev libxcb-composite0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

If you get an error about something like couldn't like with `cc`, try installing the following.

```sh
sudo apt install gcc gcc-multilib binutils
```

### Dependencies
- [**crossterm**](https://crates.io/crates/crossterm) (to interact with the terminal)
- [**ctrlc**](https://crates.io/crates/ctrlc) (to prevent close on Ctrl+c)
- [**copypasta**](https://crates.io/crates/copypasta) (for clipboard access)
