use crate::prelude::*;

mod test;

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
