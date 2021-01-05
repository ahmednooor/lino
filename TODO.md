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
- [x] enhance tab character rendering (right now it enters num of spaces for current tab width). (iter 1)> made it to behave partially like natural tab behaviour e.g. only add the remaining spaces of tab width based on current cursor position. but it still uses actual spaces instead of the tab character. will probably stay that way.
- [x] better error handling. (in case of error, save data to a temp file, prepare err feedback, panic and let the user know the err msg and recovery file path.)


**Planned**
- [x] Alt+[, Alt+] for indentation
- [x] Ctrl+Up/Alt+Up, Ctrl+Down/Alt+Down to move current line up and down
- [ ] Alt+Shift+Up, Alt+Shift+Down to duplicate current line up and down
- [ ] Shift+Del to delete whole line
- [ ] Ctrl+Backspace to delete previous word
- [ ] Ctrl+Del to delete next word
- [ ] auto indent on new line based on previous line
- [ ] find/search
- [ ] word wrap
- [ ] syntax highlighting
- [ ] settings
- [ ] logging*
- [ ] tests*

\* Maybe
