use crate::prelude::*;

crate struct Basic {
    stdin: io::Stdin,
}

impl Basic {
    crate fn new() -> Self {
        Basic { stdin: io::stdin() }
    }
}

impl Presentation for Basic {
    fn start_prompt(&mut self, prompt: Prompt<'_>) -> Fallible<()> {
        println!("--------------------------------------------------");
        println!("Please {}:", prompt.question_kind.prompt_text());
        let prompt_line_kind = prompt.question_kind.prompt_line_kind();
        for line in prompt.card.lines_with_kind(prompt_line_kind) {
            println!("- {}", line);
        }
        Ok(())
    }

    fn read_response(&mut self, prompt: Prompt<'_>, index: usize) -> Fallible<Option<String>> {
        println!("Response {}/{}: ", index, prompt.num_responses);
        let mut buffer = String::new();
        self.stdin.read_line(&mut buffer)?;
        let response_language = prompt.question_kind.response_language();
        let response = response_language.transliterate(buffer.trim());
        if response != buffer.trim() {
            println!("  (transliterated to `{}`)", response);
        }
        if response.is_empty() {
            Ok(None)
        } else {
            Ok(Some(response))
        }
    }

    fn read_result(
        &mut self,
        _prompt: Prompt<'_>,
        missing_answers: &[&str],
    ) -> Fallible<QuestionResult> {
        println!("Missing answers:");
        for answer in missing_answers {
            println!("- {}", answer);
        }

        loop {
            println!("Did you know it (yes/almost/no)? ");
            let mut buffer = String::new();
            self.stdin.read_line(&mut buffer)?;
            let buffer = buffer.trim().to_lowercase();
            match &buffer[..] {
                "yes" | "y" => return Ok(QuestionResult::Yes),
                "almost" | "a" => return Ok(QuestionResult::Almost),
                "no" | "n" => return Ok(QuestionResult::No),
                _ => {}
            }
        }
    }

    fn cleanup(&mut self) {
        println!();
        println!();
        println!();
    }

    fn quiz_expired(
        &mut self,
        quiz_duration: Duration,
        remaining_cards: usize,
    ) -> Fallible<Option<i64>> {
        println!("--------------------------------------------------");
        println!(
            "{} minutes have expired since you started the quiz.",
            quiz_duration.num_minutes()
        );
        println!(
            "There are still {} cards left to go.",
            remaining_cards,
        );
        loop {
            println!("If you want to stop, press enter.");
            println!("Otherwise, type in how many more minutes: ");
            let mut buffer = String::new();
            self.stdin.read_line(&mut buffer)?;
            if buffer.is_empty() {
                return Ok(None);
            }
            match i64::from_str(&buffer) {
                Ok(v) if v >= 0 => {
                    return Ok(Some(v));
                }
                _ => {}
            }
        }
    }
}
