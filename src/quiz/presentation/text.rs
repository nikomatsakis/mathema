use crate::prelude::*;

crate struct TextPresentation<D: TextDelegate> {
    delegate: D
}

impl<D: TextDelegate> TextPresentation<D> {
    crate fn new(delegate: D) -> Self {
        TextPresentation { delegate }
    }
}

crate trait TextDelegate {
    fn read_answer(&mut self, prompt: Prompt<'_>) -> Fallible<Option<String>>;
    fn read_result(&mut self, prompt: Prompt<'_>) -> Fallible<Option<QuestionResult>>;
    fn read_minutes(&mut self) -> Fallible<Option<String>>;
    fn cleanup(&mut self);
    fn println(&mut self, text: &str) -> Fallible<()>;
}

macro println($this:expr, $($args:tt)*) {
    $this.delegate.println(&format!($($args)*))?;
}

const INCORRECT: &str = "\u{1F4A3}";
const CORRECT: &str = "\u{1F389}";
const MISSING: &str = "\u{1F526}";

impl<D: TextDelegate> Presentation for TextPresentation<D> {
    fn start_prompt(&mut self, prompt: Prompt<'_>) -> Fallible<()> {
        println!(self, "Please {}:", prompt.question_kind.prompt_text());
        let prompt_line_kind = prompt.question_kind.prompt_line_kind();
        for line in prompt.card.lines_with_kind(prompt_line_kind) {
            println!(self, "- {}", line);
        }
        Ok(())
    }

    fn read_response(&mut self, prompt: Prompt<'_>, index: usize) -> Fallible<Option<String>> {
        println!(self, "Response {}/{}: ", index, prompt.num_responses);
        self.delegate.read_answer(prompt)
    }

    fn read_result(
        &mut self,
        prompt: Prompt<'_>,
        missing_answers: &[&str],
        correct_answers: &[String],
        incorrect_answers: &[String],
    ) -> Fallible<QuestionResult> {
        if !incorrect_answers.is_empty() {
            println!(self, "Incorrect answers:");
            for answer in incorrect_answers {
                println!(self, "{} {}", INCORRECT, answer);
            }
        }

        if !correct_answers.is_empty() {
            println!(self, "Correct answers:");
            for answer in correct_answers {
                println!(self, "{} {}", CORRECT, answer);
            }
        }

        if !missing_answers.is_empty() {
            println!(self, "Missing answers:");
            for answer in missing_answers {
                println!(self, "{} {}", MISSING, answer);
            }
        }

        loop {
            println!(self, "Did you know it (yes/almost/no)? ");
            if let Some(r) = self.delegate.read_result(prompt)? {
                return Ok(r);
            }
        }
    }

    fn repeat_back(
        &mut self,
        prompt: Prompt<'_>,
        expected_answer: &str,
    ) -> Fallible<Option<String>> {
        println!(self, "Repeat back `{}`:", expected_answer);
        self.delegate.read_answer(prompt)
    }

    fn try_again(
        &mut self,
        _prompt: Prompt<'_>,
        _expected_answer: &str,
    ) -> Fallible<()> {
        println!(self, "Not quite, try again!");
        Ok(())
    }

    fn cleanup(&mut self) {
        self.delegate.cleanup();
    }

    fn quiz_expired(
        &mut self,
        quiz_duration: Duration,
        remaining_cards: usize,
    ) -> Fallible<Option<i64>> {
        println!(
            self,
            "{} minutes have expired since you started the quiz.",
            quiz_duration.num_minutes()
        );
        println!(self, "There are still {} cards left to go.", remaining_cards,);
        loop {
            println!(self, "If you want to stop, press enter.");
            println!(self, "Otherwise, type in how many more minutes: ");
            match self.delegate.read_minutes()? {
                Some(buffer) => {
                    match i64::from_str(&buffer) {
                        Ok(v) if v >= 0 => {
                            return Ok(Some(v));
                        }
                        _ => {}
                    }
                }
                None => return Ok(None),
            }
        }
    }
}

