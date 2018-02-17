#![cfg(test)]
#![cfg(FALSE)]

use crate::prelude::*;
use super::expiration_duration;

struct CardFactory {
    date: UtcDateTime,
    card: CardRecord,
}

impl CardFactory {
    fn ask(&mut self, days: i64, kind: QuestionKind, result: QuestionResult) {
        let date = self.date;
        card.questions.push(QuestionResult { date, kind, result });
        self.date += Duration::days(days);
    }
}

const QK: QuestionKind = QuestionKind::Translate { from: Language::Greek, to: Langauge::English };

#[test]
fn expiration_no_days() {
    let mut factory = CardFactory::new();
    expiration_duration(&factord.card, None);
}

#[test]
fn expiration_one_day() {
    let mut factory = CardFactory::new();
    factory.ask(0, QK, QuestionResult::Yes);
    expiration_duration(&factord.card, None);
}
