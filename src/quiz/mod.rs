use crate::prelude::*;

crate mod presentation;

const SUITABLE_QUESTIONS: &[(Language, &[QuestionKind])] = &[(
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
)];

lazy_static! {
    static ref PARENTHETICALS: Regex = Regex::new(r"\(.*\)").unwrap();
}

crate fn quiz(
    options: &MathemaOptions,
    language_str: &str,
    mode: Option<PresentationMode>,
    duration_min: i64,
) -> Fallible<()> {
    let rng = &mut rand::thread_rng();

    let mode = mode.unwrap_or_else(|| match &env::var("TERM").ok() {
        None => PresentationMode::Basic,
        Some(s) if s == "dumb" => PresentationMode::Basic,
        Some(_) => PresentationMode::Ncurses,
    });

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

    let parentheticals = Regex::new(r"\(.*\)").unwrap();

    Quiz {
        rng,
        repo,
        options,
        language,
        parentheticals,
        duration_min,
        suitable_questions,
        mode,
    }.run()
}

struct Quiz<'o> {
    rng: &'o mut ThreadRng,
    repo: &'o mut MathemaRepository,
    options: &'o MathemaOptions,
    language: Language,
    parentheticals: Regex,
    duration_min: i64,
    suitable_questions: &'o [QuestionKind],
    mode: PresentationMode,
}

impl Quiz<'o> {
    fn run(self) -> Fallible<()> {
        let original_start_time = Utc::now();
        let mut start_time = Utc::now();
        let mut max_duration = Duration::minutes(self.duration_min);

        let cards = selection::expired_cards(self.rng, self.repo, self.suitable_questions);

        let mut presentation = Presentation::with_mode(self.mode);

        let cards_len = cards.len();
        for ((uuid, question_kind), cards_remaining) in cards.into_iter().zip((1..=cards_len).rev())
        {
            let quiz_duration = Utc::now().signed_duration_since(start_time);
            if quiz_duration > max_duration {
                match presentation.quiz_expired(
                    Utc::now().signed_duration_since(original_start_time),
                    cards_remaining,
                )? {
                    None => break,
                    Some(minutes) => {
                        start_time = Utc::now();
                        max_duration = Duration::minutes(minutes);
                    }
                }
            }

            let card = self.repo.card(uuid);
            let mut expected_responses: Vec<_> = card.lines_with_kind(
                question_kind.response_line_kind(),
            ).collect();

            let prompt = Prompt {
                start_time,
                card,
                question_kind,
                num_responses: expected_responses.len(),
            };

            presentation.start_prompt(prompt)?;

            let mut counter = 1;
            let mut total_responses = expected_responses.len();
            let mut wrong_responses = vec![];
            let mut correct_responses = vec![];
            while let Some(user_response) = presentation.read_response(prompt, counter)? {
                let len_before = expected_responses.len();
                expected_responses.retain(|r| !check_user_response(r, &user_response));

                if expected_responses.len() != len_before {
                    wrong_responses.push(user_response);
                } else {
                    correct_responses.push(user_response);
                }

                if counter >= total_responses {
                    break;
                }

                counter += 1;
            }

            let result = if expected_responses.is_empty() {
                QuestionResult::Yes
            } else {
                presentation.read_result(
                    prompt,
                    &expected_responses,
                    &correct_responses,
                    &wrong_responses,
                )?
            };

            // If they said NO, then let's have them repeat until everything looks
            // right or they ask us to stop.
            match result {
                QuestionResult::Yes => {}
                QuestionResult::Almost | QuestionResult::No => {
                    let expected_responses =
                        card.lines_with_kind(question_kind.response_line_kind());
                    'repeat: for response in expected_responses {
                        'next_word: loop {
                            match presentation.repeat_back(prompt, response)? {
                                Some(user_response) => {
                                    if check_user_response(response, &user_response) {
                                        break 'next_word;
                                    }
                                }

                                None => break 'repeat,
                            }
                        }
                    }
                }
            }

            let record = self.repo.database_mut().card_record_mut(uuid);
            record.push_question_record(
                question_kind,
                QuestionRecord {
                    date: Utc::now(),
                    result: result,
                },
            );

            presentation.cleanup();
        }

        mem::drop(presentation);

        self.repo.write_database()?;

        Ok(())
    }
}

fn check_user_response(expected_response: &str, user_response: &str) -> bool {
    let user_response = user_response.trim();
    let expected_response = PARENTHETICALS.replace_all(expected_response, "");
    expected_response.trim() == user_response || {
        expected_response
            .split(",")
            .any(|r| r.trim() == user_response)
    }
}

#[test]
fn check_user_response1() {
    assert!(check_user_response("a, b (c)", "a"));
    assert!(check_user_response("a, b (c)", "b"));
    assert!(check_user_response("a, b (c)", "b "));
    assert!(check_user_response("a, b (c)", "a, b"));
    assert!(check_user_response("a, b (c)", " a, b"));
    assert!(!check_user_response("a, b (c)", "c"));
}
