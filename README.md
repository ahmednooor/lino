# lino

A command-line text editor with notepad-like key-bindings. [WIP]

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
- [ ] ctrl+z to undo
- [ ] ctrl+y to redo
- [ ] ctrl+shift+z to redo
- [ ] file i/o
- [ ] ctrl+s to save
- [ ] enhance status bar (filename etc.)
- [ ] syntax highlighting
- [ ] enhance tab character rendering (right now it enters num of spaces for current tab width)

### Dependencies
- crossterm (For interacting with terminal)
- ctrlc (For preventing close on Ctrl+c)
- copypasta (For clipboard functionality)
