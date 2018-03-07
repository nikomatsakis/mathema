use crate::prelude::*;

/*

Flow chart of presentation methods


+---> start_prompt
|         |
|         v
|     read_response
|         |   ^
|  +------+---+
|  |      |
|  |    errors
| ok      |
|  |      v
|  |  read_response --No--> read_response
|  |      |                      |
|  |   Yes or Almost             |
|  |      v                      |
|  +> cleanup <------------------+
|         |
|         +-->quiz_expired--->[done]
|         |       |
+---------+-------+

*/

crate trait Presentation {
    /// Invoked when we want to start a new question.
    fn start_prompt(&mut self, prompt: Prompt<'_>) -> Fallible<()>;

    /// Invoked repeatedly to read answers from the user.
    fn read_response(&mut self, prompt: Prompt<'_>, index: usize) -> Fallible<Option<String>>;

    /// Invoked once user has stopped supplying answers, if they did
    /// not get everything right; `missing_answers` is the list of
    /// answers we did not see from them.
    fn read_result(
        &mut self,
        prompt: Prompt<'_>,
        missing_answers: &[&str],
    ) -> Fallible<QuestionResult>;

    /// Invoked repeatedly if user says they got it wrong.
    fn repeat_back(
        &mut self,
        prompt: Prompt<'_>,
        expected_answer: &str,
    ) -> Fallible<Option<String>>;

    /// Invoked when user got it wrong *again*.
    fn try_again(&mut self, prompt: Prompt<'_>, expected_answer: &str) -> Fallible<()>;

    /// Invoked if user says they got it wrong.
    fn cleanup(&mut self);

    /// Prompt user that `quiz_duration` time has been spent. Ask if
    /// they want to spend more time.
    fn quiz_expired(
        &mut self,
        quiz_duration: Duration,
        remaining_cards: usize,
    ) -> Fallible<Option<i64>>;
}

#[derive(Copy, Clone, Debug)]
crate struct Prompt<'p> {
    crate start_time: UtcDateTime,
    crate card: &'p Card,
    crate question_kind: QuestionKind,
    crate num_responses: usize,
}

#[derive(Copy, Clone, Debug)]
crate enum PresentationMode {
    Basic,
    Ncurses,
}

crate mod basic;
crate mod ncurses;
crate mod text;

impl FromStr for PresentationMode {
    type Err = MathemaError;

    fn from_str(s: &str) -> Fallible<PresentationMode> {
        match s {
            "basic" => Ok(PresentationMode::Basic),
            "ncurses" => Ok(PresentationMode::Ncurses),
            _ => throw!(MathemaErrorKind::UnrecognizedPresentationMode {
                text: s.to_string(),
            }),
        }
    }
}

impl dyn Presentation {
    crate fn with_mode(mode: PresentationMode) -> Box<dyn Presentation> {
        match mode {
            PresentationMode::Basic => {
                Box::new(TextPresentation::new(basic::Basic::new())) as Box<dyn Presentation>
            }
            PresentationMode::Ncurses => {
                Box::new(TextPresentation::new(ncurses::Ncurses::new())) as Box<dyn Presentation>
            }
        }
    }
}
