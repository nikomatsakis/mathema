use crate::prelude::*;

crate struct Basic {
    stdin: io::Stdin,
}

impl Basic {
    crate fn new() -> Self {
        Basic { stdin: io::stdin() }
    }

}

impl TextDelegate for Basic {
    fn println(&mut self, text: &str) -> Fallible<()> {
        println!("{}", text);
        Ok(())
    }

    fn read_answer(&mut self, prompt: Prompt<'_>) -> Fallible<Option<String>> {
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

    fn read_result(&mut self, _prompt: Prompt<'_>) -> Fallible<Option<QuestionResult>> {
        let mut buffer = String::new();
        self.stdin.read_line(&mut buffer)?;
        let buffer = buffer.trim().to_lowercase();
        match &buffer[..] {
            "yes" | "y" => Ok(Some(QuestionResult::Yes)),
            "almost" | "a" => Ok(Some(QuestionResult::Almost)),
            "no" | "n" => Ok(Some(QuestionResult::No)),
            _ => Ok(None),
        }
    }

    fn read_minutes(&mut self) -> Fallible<Option<String>> {
        let mut buffer = String::new();
        self.stdin.read_line(&mut buffer)?;
        if buffer.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(buffer))
        }
    }

    fn cleanup(&mut self) {
        println!();
        println!();
        println!();
        println!("--------------------------------------------------");
    }
}
