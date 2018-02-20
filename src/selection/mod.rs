use crate::prelude::*;

mod test;

crate fn expired_cards(
    rng: &mut impl Rng,
    repo: &MathemaRepository,
    suitable_questions: &[QuestionKind],
) -> Vec<(Uuid, QuestionKind)> {
    // Collect the expired cards in this vector.
    let mut expired: Vec<(Duration, usize, Uuid, QuestionKind)> = vec![];
    let mut never_asked: Vec<(Uuid, QuestionKind)> = vec![];

    let db = repo.database();
    let now = Utc::now();
    for uuid in repo.card_uuids() {
        let record = db.card_record(uuid);
        for &qk in suitable_questions {
            if let Some(record) = record {
                if let Some(duration) = expiration_duration(qk, record) {
                    let expiration_date = record.last_asked(qk).unwrap() + duration;
                    if now < expiration_date {
                        let expired_by = now.signed_duration_since(expiration_date);
                        expired.push((expired_by, rng.gen(), uuid, qk));
                    }
                } else {
                    never_asked.push((uuid, qk));
                }
            } else {
                never_asked.push((uuid, qk));
            }
        }
    }

    // Sort the expired cards by when they are expired (and then randomly after that).
    expired.sort();
    let mut expired_remaining = expired.len();
    let mut expired = expired.into_iter().rev().map(|(_, _, uuid, qk)| (uuid, qk));

    // Order the cards never asked randomly.
    let mut never_asked_remaining = never_asked.len();
    rng.shuffle(&mut never_asked);
    let mut never_asked = never_asked.into_iter();

    // Intersperse new things amongst the expired things randomly.
    let mut final_list = vec![];
    while expired_remaining > 0 && never_asked_remaining > 0 {
        if rng.gen::<bool>() {
            final_list.extend(expired.next());
            expired_remaining -= 1;
        } else {
            final_list.extend(never_asked.next());
            never_asked_remaining -= 1;
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

fn expiration_duration(question_kind: QuestionKind, record: &CardRecord) -> Option<Duration> {
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
