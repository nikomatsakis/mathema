use crate::prelude::*;

const SUITABLE_QUESTIONS: &[(Language, &[QuestionKind])] = &[
    (
        Language::Greek,
        &[
            QuestionKind::Translate {
                from: Language::English,
                to: Language::Greek,
            },
            QuestionKind::Translate {
                from: Language::Greek,
                to: Language::English,
            },
        ],
    ),
];

crate fn quiz(options: &MathemaOptions, language_str: &str) -> Fallible<()> {
    let language = Language::from_str(language_str)?;

    let suitable_questions = SUITABLE_QUESTIONS
        .iter()
        .filter_map(|(l, qks)| if *l == language { Some(*qks) } else { None })
        .next()
        .ok_or(MathemaErrorKind::DontKnowHowToQuiz { language })?;
    let repo = &mut MathemaRepository::open(options)?;
    let status = repo.load_cards()?;
    if status.warn_if_needed(options.force) {
        return Ok(());
    }

    let rng = &mut rand::thread_rng();
    selection::expired_cards(rng, repo, suitable_questions);

    let mut siv = Cursive::new();
    siv.add_layer(
        Dialog::around(TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );
    siv.run();

    Ok(())
}
