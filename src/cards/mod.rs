use crate::prelude::*;

#[derive(Debug)]
crate struct Cards {
    cards: Vec<Card>,
}

#[derive(Debug)]
crate struct Card {
    source_file: String,
    uuid: Option<Uuid>,
    start_line: u64,
    lines: Vec<CardLine>,
}

#[derive(Debug)]
crate struct CardLine {
    kind: LineKind,
    text: String,
}

#[derive(Debug, PartialEq, Eq)]
crate enum LineKind {
    Comment,
    Meaning(Language),
}

#[derive(Debug, PartialEq, Eq)]
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
        self.lines
            .iter()
            .filter(move |line| line.kind == kind)
            .map(|line| &line.text[..])
    }
}

crate fn parse_cards_file(source_file: &str) -> Result<Cards, Error> {
    // Annoying note:
    // - Should I be adding context here? Do I have to do it on **every** `?`
    // - Feels like I'd like the *caller* to tag with source file but for *me*
    //   to add e.g. line number
    let input = File::open(source_file)?;
    let parser = &mut LineParser::new(input)?;
    let mut cards = Cards { cards: vec![] };

    while !parser.eof() {
        if parser.current_line_is_blank() {
            parser.read_next_line()?;
        } else {
            let card = parse_card(source_file, parser)?;
            cards.cards.push(card);
        }
    }

    Ok(cards)
}

fn parse_card(
    source_file: &str,
    parser: &mut LineParser,
) -> Result<Card, Error> {
    let mut card = Card {
        source_file: source_file.to_owned(),
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
            let kind = match word0 {
                "en" => LineKind::Meaning(Language::English),
                "gr" => LineKind::Meaning(Language::Greek),
                _ => {
                    bail!(errors::UnrecognizedLineKind {
                        source_line: parser.line_number(),
                        kind: word0.to_string(),
                    });
                }
            };
            card.lines.push(CardLine {
                kind: kind,
                text: line[word0.len()..].trim().to_owned(),
            });
        }

        parser.read_next_line()?;
    }

    Ok(card)
}
