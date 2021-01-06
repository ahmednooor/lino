<div align="center">
    <span align="center"><img src="https://raw.githubusercontent.com/ahmednooor/lino/main/lino-logo.png" alt="lino" width="240" class="center" /></span>
    <h3 align="center">A command line text editor with notepad like key bindings.</h3>
    <span align="center"><img src="https://raw.githubusercontent.com/ahmednooor/lino/main/lino-screenshot.png" alt="screenshot" class="center" /></span>
    <hr />
</div>

[![Crates.io](https://img.shields.io/crates/v/lino)](https://crates.io/crates/lino)
[![Crates.io](https://img.shields.io/crates/l/lino)](https://github.com/ahmednooor/lino/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/lino)](https://crates.io/crates/lino)

## Usage (Key Bindings)
| Key | Function |
| --- | --- |
| <kbd>↑</kbd> , <kbd>↓</kbd> , <kbd>←</kbd> , <kbd>→</kbd> | `Move cursor` |
| <kbd>Ctrl</kbd> + <kbd>←</kbd> , <kbd>→</kbd> | `Move by word` |
| <kbd>Page Up</kbd> , <kbd>Page Down</kbd> | `Move by page (scrolling)` |
| <kbd>Home</kbd> | `Move to line start` |
| <kbd>End</kbd> | `Move to line end` |
| <kbd>Shift</kbd> + <kbd>↑</kbd> , <kbd>↓</kbd> , <kbd>←</kbd> , <kbd>→</kbd> | `Select` |
| <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>←</kbd> , <kbd>→</kbd> | `Select by word` |
| <kbd>Shift</kbd> + <kbd>Page Up</kbd> , <kbd>Page down</kbd> | `Select by page` |
| <kbd>Shift</kbd> + <kbd>Home</kbd> | `Select upto line start` |
| <kbd>Shift</kbd> + <kbd>End</kbd> | `Select upto line end` |
| <kbd>Backspace</kbd> | `Delete backward` |
| <kbd>Delete</kbd> | `Delete forward` |
| <kbd>Ctrl</kbd>/<kbd>Alt</kbd> + <kbd>Backspace</kbd> | `Delete backward by word` |
| <kbd>Ctrl</kbd>/<kbd>Alt</kbd> + <kbd>Delete</kbd> | `Delete forward by word` |
| <kbd>Shift</kbd> + <kbd>Delete</kbd> | `Delete current line` |
| <kbd>Ctrl</kbd>/<kbd>Alt</kbd> + <kbd>↑</kbd> | `Swap current line with the line above` |
| <kbd>Ctrl</kbd>/<kbd>Alt</kbd> + <kbd>↓</kbd> | `Swap current line with the line below` |
| <kbd>Ctrl</kbd>/<kbd>Alt</kbd> + <kbd>Shift</kbd> + <kbd>↑</kbd> | `Duplicate current line upward (Doesn't work on Windows Terminal` |
| <kbd>Ctrl</kbd>/<kbd>Alt</kbd> + <kbd>Shift</kbd> + <kbd>↓</kbd> | `Duplicate current line downward (Doesn't work on Windows Terminal` |
| <kbd>Alt</kbd> + <kbd>]</kbd> | `Increase indentation` |
| <kbd>Alt</kbd> + <kbd>[</kbd> | `Decrease indentation` |
| <kbd>Enter</kbd> | `Enter new line` |
| <kbd>Ctrl</kbd> + <kbd>Enter</kbd> | `Enter auto-indented new line` |
| <kbd>Ctrl</kbd> + <kbd>X</kbd> | `Cut` |
| <kbd>Ctrl</kbd> + <kbd>C</kbd> | `Copy` |
| <kbd>Ctrl</kbd> + <kbd>V</kbd> | `Paste` |
| <kbd>Ctrl</kbd> + <kbd>Z</kbd> | `Undo` |
| <kbd>Ctrl</kbd> + <kbd>Y</kbd> | `Redo` |
| <kbd>Ctrl</kbd> + <kbd>S</kbd> | `Save` |
| <kbd>Ctrl</kbd> + <kbd>Q</kbd> | `Quit` |

## Install
> You will need to have Rust (2018 or higher) installed on your system before proceeding.\
**Install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)**

**1. Install with Cargo**
```sh
cargo install lino
```

**2. Run**
```sh
lino <optional-filename>
```

> Tested on `Windows 10` and `WSL (Ubuntu 18.04)`

## Build
> You will need to have Rust (2018 or higher) installed on your system before proceeding.\
**Install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)**

**1. Clone this repo**
```sh
git clone https://github.com/ahmednooor/lino.git
```
**2. Go into the cloned repo directory**
```sh
cd ./lino/
```
**3. Build & run with Cargo**
```sh
cargo run <optional-filename>
```

On `Linux (Debian based)`, if you head into problems, try installing the following libraries and re-run with cargo.

```sh
sudo apt install xorg-dev libxcb-present-dev libxcb-composite0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

If you get an error about something like couldn't link with `cc`, try installing the following.

```sh
sudo apt install gcc gcc-multilib binutils
```

> NOTE: If you are unsure about above mentioned Linux libraries, do it on a dev system or a vm instead of your daily driver.

## Rust Dependencies
- [**crossterm**](https://crates.io/crates/crossterm) (to interact with the terminal)
- [**ctrlc**](https://crates.io/crates/ctrlc) (to prevent close on Ctrl+c)
- [**copypasta**](https://crates.io/crates/copypasta) (for clipboard access)
> A big Thanks to the authors of these libraries.
