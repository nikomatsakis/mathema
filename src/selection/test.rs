#![cfg(test)]

use crate::prelude::*;
use super::{expiration_duration, DurationExt};

struct CardFactory {
    date: UtcDateTime,
    card: CardRecord,
}

fn start_date() -> UtcDateTime {
    UtcDateTime::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc)
}

impl CardFactory {
    fn new() -> Self {
        CardFactory {
            date: start_date(),
            card: CardRecord::default(),
        }
    }

    fn ask(&mut self, days: i64, result: QuestionResult) {
        self.date = self.date + Duration::days(days);
        self.card.push_question_record(QK, QuestionRecord { date: self.date, result });
    }
}

const QK: QuestionKind = QuestionKind::Translate { from: Language::Greek, to: Language::English };

#[test]
fn expiration_never_asked() {
    let factory = CardFactory::new();
    assert_eq!(expiration_duration(QK, &factory.card), None);
}

#[test]
fn expiration_yes() {
    let mut factory = CardFactory::new();
    factory.ask(0, QuestionResult::Yes);
    assert_eq!(expiration_duration(QK, &factory.card), None);
}

#[test]
fn expiration_yes_yes() {
    let mut factory = CardFactory::new();
    factory.ask(1, QuestionResult::Yes);
    factory.ask(2, QuestionResult::Yes);
    assert_eq!(expiration_duration(QK, &factory.card), Some(Duration::days(2).increase()));
}

#[test]
fn expiration_no_yes_yes() {
    let mut factory = CardFactory::new();
    factory.ask(1, QuestionResult::No);
    factory.ask(1, QuestionResult::Yes);
    factory.ask(2, QuestionResult::Yes);
    assert_eq!(expiration_duration(QK, &factory.card), Some(Duration::days(2).increase()));
}

#[test]
fn expiration_no_yes_yes_maybe() {
    let mut factory = CardFactory::new();
    factory.ask(1, QuestionResult::No);
    factory.ask(1, QuestionResult::Yes);
    factory.ask(2, QuestionResult::Yes);
    factory.ask(3, QuestionResult::Almost);
    assert_eq!(expiration_duration(QK, &factory.card), Some(Duration::days(3)));
}

#[test]
fn expiration_no_yes_yes_maybe_yes() {
    let mut factory = CardFactory::new();
    factory.ask(1, QuestionResult::No);
    factory.ask(1, QuestionResult::Yes);
    factory.ask(2, QuestionResult::Yes);
    factory.ask(3, QuestionResult::Almost);
    factory.ask(3, QuestionResult::Yes);
    assert_eq!(expiration_duration(QK, &factory.card), Some(Duration::days(3).increase()));
}

#[test]
fn expiration_no_yes_yes_maybe_no() {
    let mut factory = CardFactory::new();
    factory.ask(1, QuestionResult::No);
    factory.ask(1, QuestionResult::Yes);
    factory.ask(2, QuestionResult::Yes);
    factory.ask(3, QuestionResult::Almost);
    factory.ask(3, QuestionResult::No);
    assert_eq!(expiration_duration(QK, &factory.card), Some(Duration::days(3).decrease()));
}
