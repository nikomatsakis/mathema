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
) -> Option<UtcDateTime> {
    let last_question = record.questions(question_kind).last()?;
    let durations = record
        .question_pairs(question_kind)
        .take_while(|(_, q1)| q1.result == last_question.result)
        .map(|(q0, q1)| q1.date.signed_duration_since(q0.date));
    let next_duration = match last_question.result {
        QuestionResult::Yes => {
            let max_duration = durations.max()?;
            (max_duration * 3) / 2
        }

        QuestionResult::Almost => {
            durations.min()?
        }

        QuestionResult::No => {
            let min_duration = durations.min()?;
            min_duration / 2
        }
    };

    Some(last_question.date + next_duration)
}

