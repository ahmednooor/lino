use super::*;
use keybindings::keys;

pub(crate) static GUIDE_TEXT: &str = 
"# Lino Usage (Key Bindings)

> NOTE: Some key-bindings might not work on different terminals. But the most 
>       common ones should work on all.

## MOVING

* Move:
| Up | Down | Left | Right |

* Move by word:
| Ctrl + Left | Ctrl + Right |

* Move by page (scroll):
| Page Up | Page Down |

* Move to line start:
| Home |

* Move to line end:
| End |

## SELECTING

* Select:
| Shift + Up | Shift + Down | Shift + Left | Shift + Right |

* Select by word:
| Ctrl + Shift + Left | Ctrl + Shift + Right |

* Select by page (scroll):
| Shift + Page Up | Shift + Page Down |

* Select till line start:
| Shift + Home |

* Select till line end:
| Shift + End |

* Select all:
| Ctrl + A |

## Editing

* Input:
| Alphabetic keys, tab, space etc. |

* New Line (auto-indented):
| Enter |

* New Line (non-indented):
| Ctrl  + Enter |
| Shift + Enter |

## DELETING

* Delete by character:
| Backspace | Delete |

* Delete by word:
| Ctrl + Backspace | Ctrl + Delete |
| Alt  + Backspace | Alt  + Delete |

* Delete current line:
| Shift + Delete |

## LINE OPERATIONS

* Move current line up:
| Ctrl + Up |
| Alt  + Up |

* Move current line down:
| Ctrl + Down |
| Alt  + Down |

* Duplicate current line up:
| Ctrl + Shift + Up |
| Alt  + Shift + Up |

* Duplicate current line down:
| Ctrl + Shift + Down |
| Alt  + Shift + Down |
| Ctrl + D |

## INDENTATION
> If there is a multi-line selection, then indentation will be applied to all
the selected lines.

* Increase Indentation:
| Alt + ]     |
| Alt + Right |
| Tab         | (only works with selection, otherwise adds normal tab spaces.)

* Decrease Indentation:
| Alt + [     |
| Alt + Left  |
| Shift + Tab |

## OTHER OPERATIONS

* Find / Replace:
| Ctrl + F |

* Cut:
| Ctrl + X |

* Copy:
| Ctrl + C |

* Paste:
| Ctrl + V |

* Undo:
| Ctrl + Z |

* Redo:
| Ctrl + Y |

* Save:
| Ctrl + S |

* Save as:
| Alt + S |

* Close editor:
| Ctrl + Q |
| Ctrl + W |

~ End of document. ~";

impl Lino {
    pub(crate) fn show_guide(&mut self) {
        self.enter_alt_screen_and_enable_raw_mode();

        {
            let mut new_editor = Lino::from_string(&GUIDE_TEXT.to_string());
            new_editor.file.path = "GUIDE.md".to_string();
            new_editor.task_feedback.default_text = "[Esc] Go Back".to_string();
            new_editor.settings.read_only = true;
            new_editor.settings.show_line_nums_frame = false;
            new_editor.clear_all_keybindings();
            new_editor.add_read_only_mode_keybindings();
            new_editor.keybindings.insert(format!("{}+{}", keys::ALT, 'g'), Lino::command_quit);
            new_editor.keybindings.insert(format!("{}", keys::ESC), Lino::command_quit);
            new_editor.start();
        }

        self.enter_alt_screen_and_enable_raw_mode();
    }
}
