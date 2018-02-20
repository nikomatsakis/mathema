use crate::prelude::*;

crate trait Presentation {
    fn start_prompt(&mut self, prompt: Prompt<'_>) -> Fallible<()>;
    fn read_response(&mut self, prompt: Prompt<'_>, index: usize) -> Fallible<Option<String>>;
    fn read_result(
        &mut self,
        prompt: Prompt<'_>,
        missing_answers: &[&str],
    ) -> Fallible<QuestionResult>;
    fn cleanup(&mut self);

    /// Prompt user that `quiz_duration` time has been spent. Ask if
    /// they want to spend more time.
    fn quiz_expired(&mut self, quiz_duration: Duration) -> Fallible<Option<i64>>;
}

#[derive(Copy, Clone, Debug)]
crate struct Prompt<'p> {
    crate start_time: UtcDateTime,
    crate card: &'p Card,
    crate question_kind: QuestionKind,
    crate num_responses: usize,
}

mod basic;
mod ncurses;

crate fn basic() -> Box<Presentation> {
    Box::new(basic::Basic::new())
}
