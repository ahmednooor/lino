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
- [x] Ctrl+arrow to move by words
- [x] edit text
- [x] scroll (adjust frames) on move
- [x] home key to move at line start
- [x] end key to move at line end
- [x] page up to scroll up
- [x] page down to scroll down
- [x] Ctrl+q to quit
- [x] text selection with shift+ arrow, page up/down, home/end
- [x] text selection by words with Ctrl+shift+arrow
- [x] Ctrl+a to select all
- [x] Ctrl+x to cut
- [x] Ctrl+c to copy
- [x] Ctrl+v to paste
- [x] Ctrl+z to undo
- [x] Ctrl+y to redo
- [x] Ctrl+shift+z to redo
- [x] file i/o
- [x] Ctrl+s to save
- [x] filename in status bar

- [x] enhance tab character rendering
    
    (iter 1): made it to behave partially like natural tab behaviour e.g. only add the remaining spaces of tab width based on current cursor position. but it still uses actual spaces instead of the tab character. will probably stay that way.

- [x] better error handling. 
    
    (in case of error, save data to a temp file, prepare err feedback, panic and let the user know the err msg and recovery file path.)

- [x] Alt+[, Alt+] for indentation (Alt+[ doesn't work on linux)
- [x] Ctrl/Alt+Up, Ctrl/Alt+Down to swap current line up and down (Alt with Arrow Keys doesn't work on WT)
- [x] Ctrl/Alt+Shift+Up, Ctrl/Alt+Shift+Down to duplicate current line up and down (doesn't work on WT, works on CMD)
- [x] Shift+Del to delete whole line
- [x] Ctrl+Backspace/Alt+Backspace, to delete previous word
- [x] Ctrl+Del/Alt+Del, to delete next word
- [x] Ctrl+Enter to auto indent new line

- [x] syntax highlighting

    (iter 1): highlights line-wise instead of complete file, for the sake of performance. but this can't parse multi-line tokens like multiline comments & strings etc.
    
    (needs improvement) both performance-wise if possible and (currently there is a bug if you paste over selected text, it doesn't highlight new text unless you move you cursor to those new lines [fixed for now]).

- [x] improve file io err handling
- [x] add clear mapping between keybindings and operations (refactoring required)
- [x] improve rendering (maintain render buffers and only render edited cells)

    (iter 1): turns out that rendering individual cells takes more time than rendering the complete frame. that is because rendering character by character and setting bg and fg colors for each character takes more time.

- [x] show command/task feedback or error below status bar
- [x] Ctrl+w to close as this is the dominant shortcut to close in GUI apps
- [x] Alt+s to save as (Ctrl+Shift+s isn't detected for now so had to pick Alt+s for save as)
- [ ] find, replace
- [ ] word wrap
- [ ] help screen

#### Maybe
- [ ] tests
- [ ] settings
- [ ] logging
- [ ] improve highlighting (speed up and highlight chunks or whole file instead of single line)
- [ ] improve unicode and tab char rendering (width-wise)
