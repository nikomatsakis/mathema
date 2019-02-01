use crate::prelude::*;

mod test;

crate struct CardAndExpirationDate {
    crate uuid: Uuid,
    crate kind: QuestionKind,
    crate expiration: Option<(Duration, UtcDateTime)>,
}

crate fn expiration_dates(
    repo: &'a MathemaRepository,
    suitable_questions: &'a [QuestionKind],
) -> impl Iterator<Item = CardAndExpirationDate> + 'a {
    let db = repo.database();
    repo.card_uuids()
        .cartesian_product(suitable_questions)
        .map(move |(uuid, &kind)| {
            let expiration = (|| -> Option<_> {
                let record = db.card_record(uuid)?;
                let duration = expiration_duration(kind, record)?;
                Some((duration, record.last_asked(kind).unwrap() + duration))
            })();
            CardAndExpirationDate {
                uuid,
                kind,
                expiration,
            }
        })
}

crate fn expired_cards(
    rng: &mut impl Rng,
    repo: &MathemaRepository,
    suitable_questions: &[QuestionKind],
) -> Vec<(Uuid, QuestionKind)> {
    // Collect the expired cards in these vectors.
    let mut expired: Vec<(Duration, usize, Uuid, QuestionKind)> = vec![];
    let mut never_asked: Vec<(Uuid, QuestionKind)> = vec![];
    let now = Utc::now();
    for card_data in expiration_dates(repo, suitable_questions) {
        match card_data.expiration {
            Some((_, expiration_date)) => {
                if expiration_date < now {
                    let expired_by = now.signed_duration_since(expiration_date);
                    expired.push((expired_by, rng.gen(), card_data.uuid, card_data.kind));
                }
            }
            None => {
                never_asked.push((card_data.uuid, card_data.kind));
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

    // Retain just one question per card, whichever one we happened to
    // pick first I guess.
    let mut just_one_per_card = HashSet::new();
    final_list.retain(|(uuid, _qk)| just_one_per_card.insert(*uuid));

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

crate fn expiration_duration(question_kind: QuestionKind, record: &CardRecord) -> Option<Duration> {
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
