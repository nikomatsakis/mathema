use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub(crate) enum Language {
    English,
    Greek,
}

impl fmt::Display for Language {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::English => write!(fmt, "en"),
            Language::Greek => write!(fmt, "gr"),
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
            }.into()),
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
    let last_is_semi = s.chars().last().map(|c| c == ';').unwrap_or(false);
    let mut push = |not_semi: char, yes_semi: char| {
        if last_is_semi && not_semi != yes_semi {
            s.truncate(s.len() - 1);
            s.push(yes_semi);
        } else {
            s.push(not_semi);
        }
    };

    match c {
        'a' => push('α', 'ά'),
        'b' => push('β', 'β'),
        'g' => push('γ', 'γ'),
        'd' => push('δ', 'δ'),
        'e' => push('ε', 'έ'),
        'z' => push('ζ', 'ζ'),
        'h' => push('η', 'ή'),
        'u' => push('θ', 'θ'),
        'i' => push('ι', 'ί'),
        'k' => push('κ', 'κ'),
        'l' => push('λ', 'λ'),
        'm' => push('μ', 'μ'),
        'n' => push('ν', 'ν'),
        'j' => push('ξ', 'ξ'),
        'o' => push('ο', 'ό'),
        'p' => push('π', 'π'),
        'r' => push('ρ', 'ρ'),
        's' => push('σ', 'σ'),
        't' => push('τ', 'τ'),
        'y' => push('υ', 'ύ'),
        'f' => push('φ', 'φ'),
        'x' => push('χ', 'χ'),
        'c' => push('ψ', 'ψ'),
        'v' => push('ω', 'ώ'),
        'q' => push(';', ';'),

        'A' => push('Α', 'Ά'),
        'B' => push('Β', 'Β'),
        'G' => push('Γ', 'Γ'),
        'D' => push('Δ', 'Δ'),
        'E' => push('Ε', 'Έ'),
        'Z' => push('Ζ', 'Ζ'),
        'H' => push('Η', 'Ή'),
        'U' => push('Θ', 'Θ'),
        'I' => push('Ι', 'Ί'),
        'K' => push('Κ', 'Κ'),
        'L' => push('Λ', 'Λ'),
        'M' => push('Μ', 'Μ'),
        'N' => push('Ν', 'Ν'),
        'J' => push('Ξ', 'Ξ'),
        'O' => push('Ο', 'Ό'),
        'P' => push('Π', 'Π'),
        'R' => push('Ρ', 'Ρ'),
        'S' => push('Σ', 'Σ'),
        'T' => push('Τ', 'Τ'),
        'Y' => push('Υ', 'Ύ'),
        'F' => push('Φ', 'Φ'),
        'X' => push('Χ', 'Χ'),
        'C' => push('Ψ', 'Ψ'),
        'V' => push('Ω', 'Ώ'),
        'Q' => push(':', ':'),

        _ => push(c, c),
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
fn transliterate_giasou() {
    assert_eq!(Language::Greek.transliterate("g;iasoy"), "γίασου");
}
