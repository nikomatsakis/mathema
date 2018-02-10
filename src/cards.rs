use crate::uuid::Uuid;
use std::path::PathBuf;

crate struct Cards {
    cards: Vec<Card>,
}

crate struct Card {
    source_file: PathBuf,
    uuid: Option<Uuid>,
    start_line: usize,
    lines: Vec<CardLine>,
}

crate struct CardLine {
    kind: LineKind,
    text: String,
}

crate enum LineKind {
    Comment,
    Meaning(Language),
}

crate enum Language {
    English,
    Greek,
}

impl Card {
    crate fn meanings(&self, language: Language) -> impl Iterator<Item = &str> + '_ {
        let kind = LineKind::Meaning(language);
        self.lines_with_kind(kind)
    }

    fn lines_with_kind(&self, kind: LineKind) -> impl Iterator<Item = &str> + '_ {
        self.lines.iter().filter(move |line| line.kind == kind)
    }
}
