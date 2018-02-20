use crate::prelude::*;

crate mod presentation;

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

crate fn quiz(options: &MathemaOptions, language_str: &str, duration_min: i64) -> Fallible<()> {
    let rng = &mut rand::thread_rng();

    let language = Language::from_str(language_str)?;

    let suitable_questions = SUITABLE_QUESTIONS
        .iter()
        .filter_map(|(l, qks)| if *l == language { Some(*qks) } else { None })
        .next()
        .ok_or(MathemaErrorKind::DontKnowHowToQuiz {
            language: language.full_name(),
        })?;
    let repo = &mut MathemaRepository::open(options)?;
    let status = repo.load_cards()?;
    if status.warn_if_needed(options.force) {
        return Ok(());
    }

    let original_start_time = Utc::now();
    let mut start_time = Utc::now();
    let mut max_duration = Duration::minutes(duration_min);

    let cards = selection::expired_cards(rng, repo, suitable_questions);

    let mut presentation = presentation::basic();

    let parentheticals = Regex::new(r"([^)]*)|\[[^]\]").unwrap();

    for (uuid, question_kind) in cards {
        let quiz_duration = Utc::now().signed_duration_since(start_time);
        if quiz_duration > max_duration {
            match presentation.quiz_expired(Utc::now().signed_duration_since(original_start_time))? {
                None => break,
                Some(minutes) => {
                    start_time = Utc::now();
                    max_duration = Duration::minutes(minutes);
                }
            }
        }

        let card = repo.card(uuid);
        let mut expected_responses: Vec<_> = card.lines_with_kind(question_kind.response_line_kind())
            .collect();

        let prompt = Prompt {
            start_time,
            card,
            question_kind,
            num_responses: expected_responses.len(),
        };

        presentation.start_prompt(prompt)?;

        let mut counter = 1;
        while let Some(user_response) = presentation.read_response(prompt, counter)? {
            counter += 1;

            expected_responses.retain(|r| {
                let r = parentheticals.replace_all(r, "");
                r != user_response
            });

            if expected_responses.is_empty() {
                break;
            }
        }

        let result = if expected_responses.is_empty() {
            QuestionResult::Yes
        } else {
            presentation.read_result(prompt, &expected_responses)?
        };
        let record = repo.database_mut().card_record_mut(uuid);
        record.push_question_record(question_kind, QuestionRecord {
            date: Utc::now(),
            result: result,
        });

        presentation.cleanup();
    }

    mem::drop(presentation);

    repo.write_database()?;

    Ok(())
}
