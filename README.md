
<div align="center">
    <br />
    <span align="center"><img src="https://raw.githubusercontent.com/ahmednooor/lino/main/assets/lino-icon.png" alt="lino" width="156" height="156" class="center" /></span>
    <h2 align="center">A command line text editor with notepad like key bindings. [WIP]</h2>
    <span align="center"><img src="https://raw.githubusercontent.com/ahmednooor/lino/main/assets/screenshot-4.png" alt="screenshot" class="center" /></span>
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
| <kbd>Ctrl</kbd> + <kbd>A</kbd> | `Select all` |
| <kbd>Backspace</kbd> | `Delete backward` |
| <kbd>Delete</kbd> | `Delete forward` |
| <kbd>Ctrl</kbd> / <kbd>Alt</kbd> + <kbd>Backspace</kbd> | `Delete backward by word` |
| <kbd>Ctrl</kbd> / <kbd>Alt</kbd> + <kbd>Delete</kbd> | `Delete forward by word` |
| <kbd>Shift</kbd> + <kbd>Delete</kbd> | `Delete current line` |
| <kbd>Ctrl</kbd> / <kbd>Alt</kbd> + <kbd>↑</kbd> | `Move current line up` |
| <kbd>Ctrl</kbd> / <kbd>Alt</kbd> + <kbd>↓</kbd> | `Move current line down` |
| <kbd>Ctrl</kbd> / <kbd>Alt</kbd> + <kbd>Shift</kbd> + <kbd>↑</kbd> | `Duplicate current line upward (Doesn't work on Windows Terminal)` |
| <kbd>Ctrl</kbd> / <kbd>Alt</kbd> + <kbd>Shift</kbd> + <kbd>↓</kbd> | `Duplicate current line downward (Doesn't work on Windows Terminal)` |
| <kbd>Alt</kbd> + <kbd>]</kbd> / <kbd>→</kbd> | `Increase indentation` |
| <kbd>Alt</kbd> + <kbd>[</kbd> / <kbd>←</kbd> | `Decrease indentation` |
| <kbd>Enter</kbd> | `Enter auto-indented new line` |
| <kbd>Ctrl</kbd> + <kbd>Enter</kbd> | `Enter non-indented new line` |
| <kbd>Ctrl</kbd> + <kbd>X</kbd> | `Cut` |
| <kbd>Ctrl</kbd> + <kbd>C</kbd> | `Copy` |
| <kbd>Ctrl</kbd> + <kbd>V</kbd> | `Paste` |
| <kbd>Ctrl</kbd> + <kbd>Z</kbd> | `Undo` |
| <kbd>Ctrl</kbd> + <kbd>Y</kbd> | `Redo` |
| <kbd>Ctrl</kbd> + <kbd>S</kbd> | `Save` |
| <kbd>Alt</kbd> + <kbd>S</kbd> | `Save as` |
| <kbd>Ctrl</kbd> + <kbd>W</kbd> | `Close` |

> Some key-bindings don't work on Linux.

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
- [**syntect**](https://github.com/trishume/syntect) (for syntax highlighting)
> A big Thanks to the authors/maintainers/contributors of these libraries.

## Motivation
Imagine you're a mortal who is accustomed to VS Code or some other GUI Text Editor/IDE, and suddenly you have to login to some SSH or fire up a Linux Server Distro with no GUI. You find yourself treading in the territory of the gods where beasts like Vim, Emacs and others like them reside. You find a friend called Nano, but it doesn't speak your tongue (i.e. key bindings), so you waste your time trying to learn a new way to communicate instead of getting the job done and get the hell out of there. This editor can be your friend that speaks the same-ish tongue and you can take it there with you. It isn't the only one though, there is another one called [Micro](https://github.com/zyedidia/micro).
