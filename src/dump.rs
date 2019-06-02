use crate::prelude::*;

crate fn dump(options: &MathemaOptions, filter: &Option<String>, expired: bool) -> Fallible<()> {
    let repo = &mut MathemaRepository::open(options)?;
    let status = repo.load_cards()?;
    if status.warn_if_needed(options.force) {
        return Ok(());
    }

    let mut question_kinds: Vec<QuestionKind> = SUITABLE_QUESTIONS
        .iter()
        .flat_map(|(_lang, kinds)| kinds.iter().cloned())
        .collect();
    question_kinds.sort();
    question_kinds.dedup();

    let stdout = &mut std::io::stdout();

    let db = &repo.database();

    if !expired {
        for uuid in repo.card_uuids() {
            dump_card(stdout, repo, db, uuid, &question_kinds, filter)?;
        }
    } else {
        let rng = &mut rand::thread_rng();
        let cards = selection::expired_cards(rng, repo, &question_kinds);
        for (uuid, question_kind) in cards {
            dump_card(stdout, repo, db, uuid, &[question_kind], filter)?;
        }
    }

    Ok(())
}

fn dump_card(
    stdout: &mut impl Write,
    repo: &MathemaRepository,
    db: &Database,
    uuid: Uuid,
    question_kinds: &[QuestionKind],
    filter: &Option<String>,
) -> Fallible<()> {
    let card = repo.card(uuid);

    if let Some(filter) = filter {
        if !card.lines.iter().any(|line| line.text.contains(filter)) {
            return Ok(());
        }
    }

    cards::write_cards_to(stdout, std::slice::from_ref(&card))?;
    for &question_kind in question_kinds {
        let ever_asked: Option<()> = try {
            let record = db.card_record(uuid)?;
            let last_question = record.questions(question_kind).last()?;

            let mut next: Option<&QuestionRecord> = None;
            for question in record.questions(question_kind).iter().rev() {
                let interval: Option<_> = try {
                    format!(
                        " (interval {})",
                        next?.date.signed_duration_since(question.date)
                    )
                };

                println!(
                    "* Got {:?} on {}{}",
                    question.result,
                    question.date,
                    interval.unwrap_or_default(),
                );

                next = Some(question);

                if question.result != last_question.result {
                    break;
                }
            }

            match selection::expiration_duration(question_kind, record) {
                Some(duration) => {
                    let expiration_date = last_question.date + duration;
                    println!(
                        "* {}: expires on {} (duration {})",
                        question_kind.prompt_text(),
                        expiration_date,
                        duration,
                    );
                }

                None => {
                    println!(
                        "* {}: Not enough data to figure out when to ask next.\
                         \n  Last asked on {}.",
                        question_kind.prompt_text(),
                        last_question.date,
                    );
                }
            }
        };

        if let None = ever_asked {
            println!(
                "* {}: No record of ever asking this",
                question_kind.prompt_text()
            );
        }
    }
    println!("");

    Ok(())
}
