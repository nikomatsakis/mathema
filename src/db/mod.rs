//! The "database" tracks which words have been asked and when. It
//! also tracks which card files have been added to chrono. It does
//! not track the cards themselves.

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub(crate) struct Database {
    /// Path to each card file, relative to our directory.
    card_files: Vec<PathBuf>,

    /// Records specific to a given user (for now, we only support one
    /// user per directory).
    user: User,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) records: Vec<CardRecord>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CardRecord {
    /// Uuid of the card we are asking about.
    pub(crate) uuid: Uuid,

    /// Sorted by date, always.
    pub(crate) questions: Vec<QuestionRecord>,
}

/// A question we asked the user...
#[derive(Serialize, Deserialize)]
pub(crate) struct QuestionRecord {
    /// ...when did we ask?
    pub(crate) date: UtcDateTime,

    /// ...what kind of question was it?
    pub(crate) kind: QuestionKind,

    /// ...did they know the answer?
    pub(crate) result: QuestionResult,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum QuestionKind {
    Translate { from: Language, to: Language },
}

#[derive(Serialize, Deserialize)]
pub(crate) enum QuestionResult {
    /// User knew it.
    Yes,

    /// User got it wrong, but reported that they "almost" knew it.
    Almost,

    /// User didn't know it.
    No,
}
