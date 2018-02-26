use crate::prelude::*;
use ncurses;

crate struct Ncurses {
    window: ncurses::WINDOW,
    row: i32,
}

impl Ncurses {
    crate fn new() -> Self {
        ncurses::setlocale(ncurses::LcCategory::all, "");
        let window = ncurses::initscr();
        ncurses::raw();
        ncurses::noecho();
        ncurses::clear();
        Ncurses { window, row: 0 }
    }

    fn getch(&mut self) -> Fallible<char> {
        let ch = check_ret!(ncurses::getch());
        if ch == 3 {
            throw!(MathemaErrorKind::ControlC);
        }
        Ok(char::from_u32(ch as u32).unwrap())
    }

    fn read_line(
        &mut self,
        mut push_char: impl FnMut(char, &mut String),
    ) -> Fallible<Option<String>> {
        let mut buffer = String::new();
        loop {
            check_ret!(ncurses::mvprintw(self.row, 0, &buffer));
            let ch = self.getch()?;
            if ch == '\n' {
                break;
            }
            push_char(ch, &mut buffer);
        }
        if buffer.is_empty() {
            Ok(None)
        } else {
            Ok(Some(buffer))
        }
    }
}

macro check_ret($e: expr) {
    {
        let result = $e;
        if result < 0 {
            panic!("obscure ncurses error: {}", result);
        }
        result
    }
}

impl TextDelegate for Ncurses {
    fn println(&mut self, text: &str) -> Fallible<()> {
        check_ret!(ncurses::mvprintw(self.row, 0, text));
        self.row += 1;
        Ok(())
    }

    fn read_answer(&mut self, prompt: Prompt<'_>) -> Fallible<Option<String>> {
        let response_language = prompt.question_kind.response_language();
        self.read_line(|c, b| response_language.push_char(c, b))
    }

    fn read_result(&mut self, _prompt: Prompt<'_>) -> Fallible<Option<QuestionResult>> {
        let ch = check_ret!(ncurses::getch());
        match char::from_u32(ch as u32).unwrap() {
            'y' => Ok(Some(QuestionResult::Yes)),
            'a' => Ok(Some(QuestionResult::Almost)),
            'n' => Ok(Some(QuestionResult::No)),
            _ => Ok(None),
        }
    }

    fn read_minutes(&mut self) -> Fallible<Option<String>> {
        self.read_line(|c, b| b.push(c))
    }

    fn cleanup(&mut self) {
        ncurses::clear();
        self.row = 0;
    }
}

impl Drop for Ncurses {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}
