<div align="center">
    <span align="center"><img src="https://raw.githubusercontent.com/ahmednooor/lino/main/lino-logo.png" alt="lino" width="240" height="143" class="center" /></span>
    <h3 align="center">A command line text editor with notepad like key bindings. [WIP]</h3>
    <hr />
</div>

### TODO
- [x] input text
- [x] render line numbers
- [x] render text
- [x] render status bar
- [x] speed up rendering
- [x] move left
- [x] move right
- [x] move up
- [x] move down
- [x] ctrl+arrow to move by words
- [x] edit text
- [x] scroll (adjust frames) on move
- [x] home key to move at line start
- [x] end key to move at line end
- [x] page up to scroll up
- [x] page down to scroll down
- [x] ctrl+q to quit
- [x] text selection with shift+ arrow, page up/down, home/end
- [x] text selection by words with ctrl+shift+arrow
- [x] ctrl+a to select all
- [x] ctrl+x to cut
- [x] ctrl+c to copy
- [x] ctrl+v to paste
- [x] ctrl+z to undo
- [x] ctrl+y to redo
- [x] ctrl+shift+z to redo
- [x] file i/o
- [x] ctrl+s to save
- [x] enhance status bar (filename etc.)

(hopefully soon)
- [ ] find/search
- [ ] settings
- [ ] syntax highlighting
- [ ] enhance tab character rendering (right now it enters num of spaces for current tab width)
- [ ] better error handling
- [ ] logging
- [ ] tests

### Dependencies
- [**crossterm**](https://crates.io/crates/crossterm) (to interact with the terminal)
- [**ctrlc**](https://crates.io/crates/ctrlc) (to prevent close on Ctrl+c)
- [**copypasta**](https://crates.io/crates/copypasta) (for clipboard access)
