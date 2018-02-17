//! The "database" tracks which words have been asked and when. It
//! also tracks which card files have been added to chrono. It does
//! not track the cards themselves.

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub(crate) struct Database {
    /// Path to each card file, relative to our directory.
    pub(crate) card_files: Vec<PathBuf>,

    /// Records specific to a given user (for now, we only support one
    /// user per directory).
    pub(crate) user: User,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) records: HashMap<Uuid, CardRecord>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CardRecord {
    /// Sorted by date, always.
    pub(crate) questions: HashMap<QuestionKind, Vec<QuestionRecord>>,
}

/// A question we asked the user...
#[derive(Serialize, Deserialize)]
pub(crate) struct QuestionRecord {
    /// ...when did we ask?
    pub(crate) date: UtcDateTime,

    /// ...did they know the answer?
    pub(crate) result: QuestionResult,
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub(crate) enum QuestionKind {
    Translate { from: Language, to: Language },
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub(crate) enum QuestionResult {
    /// User knew it.
    Yes,

    /// User got it wrong, but reported that they "almost" knew it.
    Almost,

    /// User didn't know it.
    No,
}

impl Database {
    crate fn empty() -> Database {
        Database {
            card_files: vec![],
            user: User { records: HashMap::new() },
        }
    }

    crate fn write_to(&self, writer: impl io::Write) -> Fallible<()> {
        extern::serde_json::ser::to_writer(writer, self)?;
        Ok(())
    }

    crate fn load_from(reader: impl io::Read) -> Fallible<Self> {
        let db = extern::serde_json::de::from_reader(reader)?;
        Ok(db)
    }

    crate fn contains_card_file(&self, path: &Path) -> bool {
        self.card_files.iter().any(|c| c == path)
    }

    crate fn card_record(&self, uuid: Uuid) -> Option<&CardRecord> {
        self.user.records.get(&uuid)
    }
}

impl CardRecord {
    crate fn questions(
        &self,
        kind: QuestionKind,
    ) -> &[QuestionRecord] {
        self.questions.get(&kind).map(|v| &v[..]).unwrap_or(&[])
    }

    crate fn most_recent_question_with_result(
        &self,
        kind: QuestionKind,
        result: QuestionResult,
    ) -> Option<usize> {
        self.questions(kind)
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(index, q)| if q.result == result { Some(index) } else { None })
            .next()
    }

    /// Returns a reverse iterater over subsequent pairs of questions.
    /// E.g., if we've asked the word 5 times, and we call those
    /// Q0..Q5, then this iterator would yield up [(Q3, Q4), (Q2, Q3),
    /// (Q1, Q2), (Q0, Q1)]. If we haven't asked the question at least
    /// twice, no tuples are returned.
    crate fn question_pairs(
        &self,
        kind: QuestionKind,
    ) -> impl Iterator<Item = (&QuestionRecord, &QuestionRecord)> {
        let questions = self.questions(kind);
        let len = questions.len();
        (1..len).map(move |i| (&questions[i - 1], &questions[i])).rev()
    }
}
