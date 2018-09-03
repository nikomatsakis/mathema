use crate::prelude::*;
use std::fmt;

#[derive(Debug)]
crate struct CardSet {
    cards: HashMap<Uuid, Card>,
}

#[derive(Debug)]
crate struct Card {
    crate uuid: Option<Uuid>,
    crate start_line: u64,
    crate lines: Vec<CardLine>,
}

#[derive(Debug)]
crate struct CardLine {
    crate kind: LineKind,
    crate text: String,
}

#[derive(Debug, PartialEq, Eq)]
crate enum LineKind {
    Comment,
    Meaning(Language),
    PartOfSpeech,
    Aoristos,
}

impl Card {
    crate fn meanings(&self, language: Language) -> impl Iterator<Item = &str> + '_ {
        let kind = LineKind::Meaning(language);
        self.lines_with_kind(kind)
    }

    crate fn is_comment_card(&self) -> bool {
        self.lines.iter().all(|l| l.kind == LineKind::Comment)
    }

    crate fn lines_with_kind(&self, kind: LineKind) -> impl Iterator<Item = &str> + '_ {
        self.lines
            .iter()
            .filter(move |line| line.kind == kind)
            .map(|line| &line.text[..])
    }
}

crate fn parse_cards_file(source_file: &Path) -> Fallible<Vec<Card>> {
    let input = File::open(source_file)?;
    parse_cards_file_from(source_file, input)
}

crate fn parse_cards_file_from(source_file: &Path, input: File) -> Fallible<Vec<Card>> {
    // Annoying note:
    // - Should I be adding context here? Do I have to do it on **every** `?`
    // - Feels like I'd like the *caller* to tag with source file but for *me*
    //   to add e.g. line number
    let parser = &mut LineParser::new(input)?;
    let mut cards = vec![];

    while !parser.eof() {
        if parser.current_line_is_blank() {
            parser.read_next_line()?;
        } else {
            let card = parse_card(source_file, parser)?;
            cards.push(card);
        }
    }

    Ok(cards)
}

fn parse_card(source_file: &Path, parser: &mut LineParser) -> Fallible<Card> {
    let mut card = Card {
        uuid: None,
        start_line: parser.line_number(),
        lines: vec![],
    };

    while !parser.current_line_is_blank() {
        let line = parser.current_line();
        if line.starts_with("#") {
            card.lines.push(CardLine {
                kind: LineKind::Comment,
                text: line[1..].trim().to_string(),
            });
        } else {
            let word0 = line.split_whitespace().next().unwrap();
            let remainder = &line[word0.len()..].trim();

            if word0 == "uuid" {
                if card.uuid.is_some() {
                    throw!(MathemaErrorKind::PreexistingUUID {
                        file: source_file.display().to_string(),
                        line: card.start_line,
                    });
                }
                match Uuid::parse_str(remainder) {
                    Ok(u) => card.uuid = Some(u),
                    Err(_) => throw!(MathemaErrorKind::InvalidUUID {
                        file: source_file.display().to_string(),
                        line: card.start_line,
                    }),
                }
            } else if word0 == "pos" {
                card.lines.push(CardLine {
                    kind: LineKind::PartOfSpeech,
                    text: remainder.to_string(),
                });
            } else if word0 == "αόριστος" {
                card.lines.push(CardLine {
                    kind: LineKind::Aoristos,
                    text: remainder.to_string(),
                });
            } else if let Ok(language) = Language::from_str(word0) {
                let kind = LineKind::Meaning(language);
                let text = language.transliterate(remainder);
                card.lines.push(CardLine {
                    kind: kind,
                    text: text,
                });
            } else {
                throw!(MathemaErrorKind::UnrecognizedLineKind {
                    source_line: parser.line_number(),
                    kind: word0.to_string(),
                });
            }
        }

        parser.read_next_line()?;
    }

    Ok(card)
}

crate fn write_cards_file(target_file: &Path, cards: &[Card]) -> Fallible<()> {
    AtomicFile::new(
        target_file.canonicalize()?,
        OverwriteBehavior::AllowOverwrite,
    ).write(|f| write_cards_to(f, cards))?;

    Ok(())
}

crate fn write_cards_to(output: &mut dyn io::Write, cards: &[Card]) -> Fallible<()> {
    let mut sep = "";
    for card in cards {
        write!(output, "{}", sep)?;
        sep = "\n";

        if let Some(u) = card.uuid {
            writeln!(output, "uuid {}", u)?;
        }

        for line in &card.lines {
            writeln!(output, "{} {}", line.kind, line.text)?;
        }
    }
    Ok(())
}

impl fmt::Display for LineKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineKind::Comment => write!(fmt, "#"),
            LineKind::Meaning(lang) => write!(fmt, "{}", lang.abbreviation()),
            LineKind::PartOfSpeech => write!(fmt, "pos"),
            LineKind::Aoristos => write!(fmt, "αόριστος"),
        }
    }
}

