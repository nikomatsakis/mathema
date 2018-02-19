use crate::prelude::*;

mod test;

crate fn expired_cards(
    repo: &MathemaRepository,
) -> Vec<(Uuid, QuestionKind)> {
    let mut expired: Vec<(Duration, Uuid, QuestionKind)> = vec![];
    let mut never_asked: Vec<(Uuid, QuestionKind)> = vec![];

    let db = repo.database();
    let now = UtcDateTime::now();
    for (&uuid, card) in repo.cards() {
        let record = db.card_record(uuid);
        for qk in card.suitable_questions() {
            match expiration_duration(qk, record) {
                None => never_asked.push((uuid, qk)),
                Some(duration) => {
                    let expiration_date = record.last_asked(qk) + duration;
                    if now < expiration_date {
                        let expired_by = now.signed_duration_since(expiration_date);
                        expired.push((expired_by, uuid, qk));
                    }
                }
            }
        }
    }

    // Put the ones that are expired by the most in front of the line:
    expired.sort();
    expired.rev();

    // Order the cards never asked randomly.
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut never_asked);

    // Intersperse new things amongst the expired things randomly.
    let mut final_list = vec![];
    while !expired.is_empty() && !never_asked.is_empty() {
        if rng.gen::<bool>() {
            final_list.extend(expired.pop());
        } else {
            final_list.extend(never_asked.pop());
        }
    }
    final_list.extend(expired);
    final_list.extend(never_asked);

    final_list
}

// Here are the patterns:
//
// _ = any answer
// M = maybe answer
// Y = yes answer
// N = no answer
// YN = yes or no answer
//
// "All M":
// - M+
//   ^^ return minimum duration
//
// "Trailing M":
// - .... _ M+
//        ^^^^ return minimum of these durations
//
// "Trailing Y":
// - .... _ Y+
//        ^^^^ increase maximum of these durations
//
// "Trailing N":
// - .... _ N+
//        ^^^^ decrease minimum of these durations
//
//
//

fn expiration_duration(
    question_kind: QuestionKind,
    record: &CardRecord,
) -> Option<Duration> {
    let last_question = record.questions(question_kind).last()?;
    let durations = record
        .question_pairs(question_kind)
        .take_while(|(_, q1)| q1.result == last_question.result)
        .map(|(q0, q1)| q1.date.signed_duration_since(q0.date));
    let next_duration = match last_question.result {
        QuestionResult::Yes => durations.max()?.increase(),
        QuestionResult::Almost => durations.min()?,
        QuestionResult::No => durations.min()?.decrease(),
    };

    Some(next_duration)
}

trait DurationExt {
    fn increase(self) -> Self;
    fn decrease(self) -> Self;
}

impl DurationExt for Duration {
    fn increase(self) -> Self {
        self * 3 / 2
    }

    fn decrease(self) -> Self {
        self / 2
    }
}
