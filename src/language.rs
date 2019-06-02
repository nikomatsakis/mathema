use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub(crate) enum Language {
    English,
    Greek,
}

impl Language {
    crate fn abbreviation(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Greek => "gr",
        }
    }

    crate fn full_name(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Greek => "Ελληνικά",
        }
    }
}

impl FromStr for Language {
    type Err = MathemaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gr" => Ok(Language::Greek),
            "en" => Ok(Language::English),
            _ => Err(MathemaErrorKind::UnrecognizedLanguage {
                text: s.to_string(),
            }
            .into()),
        }
    }
}

impl Language {
    crate fn transliterate(self, input: &str) -> String {
        let mut result = String::new();
        for c in input.chars() {
            self.push_char(c, &mut result);
        }
        result
    }

    crate fn push_char(self, c: char, s: &mut String) {
        match self {
            Language::English => s.push(c),
            Language::Greek => push_gr_char(c, s),
        }
    }
}

fn push_gr_char(c: char, s: &mut String) {
    // Adjustments:
    // q = ;
    // Q = :
    // W = ; + : accent

    let mut push = |if_neither: char, if_semi: char, if_colon: char, if_both: char| {
        let mut semi = false;
        let mut colon = false;
        if s.ends_with(":;") {
            semi = true;
            colon = true;
        } else if s.ends_with(";:") {
            semi = true;
            colon = true;
        } else if s.ends_with(":") {
            colon = true;
        } else if s.ends_with(";") {
            semi = true;
        }

        let (modifiers, modified) = if semi && colon && if_neither != if_both {
            (2, if_both)
        } else if semi && if_neither != if_semi {
            (1, if_semi)
        } else if colon && if_neither != if_colon {
            (1, if_colon)
        } else {
            (0, if_neither)
        };

        s.truncate(s.len() - modifiers);
        s.push(modified);
    };

    match c {
        'a' => push('α', 'ά', 'α', 'α'),
        'b' => push('β', 'β', 'β', 'β'),
        'g' => push('γ', 'γ', 'γ', 'γ'),
        'd' => push('δ', 'δ', 'δ', 'δ'),
        'e' => push('ε', 'έ', 'ε', 'ε'),
        'z' => push('ζ', 'ζ', 'ζ', 'ζ'),
        'h' => push('η', 'ή', 'η', 'η'),
        'u' => push('θ', 'θ', 'θ', 'θ'),
        'i' => push('ι', 'ί', 'ϊ', 'ΐ'),
        'k' => push('κ', 'κ', 'κ', 'κ'),
        'l' => push('λ', 'λ', 'λ', 'λ'),
        'm' => push('μ', 'μ', 'μ', 'μ'),
        'n' => push('ν', 'ν', 'ν', 'ν'),
        'j' => push('ξ', 'ξ', 'ξ', 'ξ'),
        'o' => push('ο', 'ό', 'ο', 'ο'),
        'p' => push('π', 'π', 'π', 'π'),
        'r' => push('ρ', 'ρ', 'ρ', 'ρ'),
        's' => push('σ', 'σ', 'σ', 'σ'),
        't' => push('τ', 'τ', 'τ', 'τ'),
        'y' => push('υ', 'ύ', 'υ', 'υ'),
        'f' => push('φ', 'φ', 'φ', 'φ'),
        'x' => push('χ', 'χ', 'χ', 'χ'),
        'c' => push('ψ', 'ψ', 'ψ', 'ψ'),
        'v' => push('ω', 'ώ', 'ω', 'ω'),
        'w' => push('ς', 'ς', 'ς', 'ς'),
        'q' => push(';', ';', ';', ';'),

        'A' => push('Α', 'Ά', 'Α', 'Α'),
        'B' => push('Β', 'Β', 'Β', 'Β'),
        'G' => push('Γ', 'Γ', 'Γ', 'Γ'),
        'D' => push('Δ', 'Δ', 'Δ', 'Δ'),
        'E' => push('Ε', 'Έ', 'Ε', 'Ε'),
        'Z' => push('Ζ', 'Ζ', 'Ζ', 'Ζ'),
        'H' => push('Η', 'Ή', 'Η', 'Η'),
        'U' => push('Θ', 'Θ', 'Θ', 'Θ'),
        'I' => push('Ι', 'Ί', 'Ϊ', 'Ι'),
        'K' => push('Κ', 'Κ', 'Κ', 'Κ'),
        'L' => push('Λ', 'Λ', 'Λ', 'Λ'),
        'M' => push('Μ', 'Μ', 'Μ', 'Μ'),
        'N' => push('Ν', 'Ν', 'Ν', 'Ν'),
        'J' => push('Ξ', 'Ξ', 'Ξ', 'Ξ'),
        'O' => push('Ο', 'Ό', 'Ο', 'Ο'),
        'P' => push('Π', 'Π', 'Π', 'Π'),
        'R' => push('Ρ', 'Ρ', 'Ρ', 'Ρ'),
        'S' => push('Σ', 'Σ', 'Σ', 'Σ'),
        'T' => push('Τ', 'Τ', 'Τ', 'Τ'),
        'Y' => push('Υ', 'Ύ', 'Υ', 'Υ'),
        'F' => push('Φ', 'Φ', 'Φ', 'Φ'),
        'X' => push('Χ', 'Χ', 'Χ', 'Χ'),
        'C' => push('Ψ', 'Ψ', 'Ψ', 'Ψ'),
        'V' => push('Ω', 'Ώ', 'Ω', 'Ω'),
        'Q' => push(':', ':', ':', ':'),

        _ => push(c, c, c, c),
    }
}

#[test]
fn push_giasou() {
    let string = &mut String::new();
    for c in "g;iasoy".chars() {
        Language::Greek.push_char(c, string);
    }

    assert_eq!(&string[..], "γίασου");
}

#[test]
fn push_throizw() {
    let string = &mut String::new();
    for c in "uro:;izv".chars() {
        Language::Greek.push_char(c, string);
    }

    assert_eq!(&string[..], "θροΐζω");
}

#[test]
fn push_throizw2() {
    let string = &mut String::new();
    for c in "uro;:izv".chars() {
        Language::Greek.push_char(c, string);
    }

    assert_eq!(&string[..], "θροΐζω");
}

#[test]
fn push_throizw3() {
    let string = &mut String::new();
    for c in "uro:;azv".chars() {
        Language::Greek.push_char(c, string);
    }

    assert_eq!(&string[..], "θρο:άζω");
}

#[test]
fn transliterate_giasou() {
    assert_eq!(Language::Greek.transliterate("g;iasoy"), "γίασου");
    assert_eq!(
        Language::Greek.transliterate("ftervt;ow"),
        "φτερωτός"
    );
}
