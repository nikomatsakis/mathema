use crate::prelude::*;
use ncurses;

crate struct Ncurses {
    basic: Basic
}

impl Ncurses {
    crate fn new() -> Self {
        ncurses::initscr();
        ncurses::raw();
        ncurses::noecho();
        Ncurses { basic: Basic::new() }
    }
}

impl Presentation for Ncurses {
    fn start_prompt(&mut self, prompt: Prompt<'_>) -> Fallible<()> {
        self.basic.start_prompt(prompt)
    }

    fn read_response(&mut self, prompt: Prompt<'_>, index: usize) -> Fallible<Option<String>> {
        self.basic.read_response(prompt, index)
    }

    fn read_result(
        &mut self,
        prompt: Prompt<'_>,
        missing_answers: &[&str],
    ) -> Fallible<QuestionResult> {
        self.basic.read_result(prompt, missing_answers)
    }

    fn repeat_back(
        &mut self,
        prompt: Prompt<'_>,
        expected_answer: &str,
    ) -> Fallible<Option<String>> {
        self.basic.repeat_back(prompt, expected_answer)
    }

    fn try_again(
        &mut self,
        prompt: Prompt<'_>,
        expected_answer: &str,
    ) -> Fallible<()> {
        self.basic.try_again(prompt, expected_answer)
    }

    fn cleanup(&mut self) {
        ncurses::clear();
    }

    fn quiz_expired(
        &mut self,
        quiz_duration: Duration,
        remaining_cards: usize,
    ) -> Fallible<Option<i64>> {
        self.basic.quiz_expired(quiz_duration, remaining_cards)
    }
}

impl Drop for Ncurses {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}
