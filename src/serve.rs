use crate::prelude::*;
use http::status::StatusCode;
use http_service::Body;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(rust_embed::RustEmbed)]
#[folder = "mathema/build"]
struct Asset;

async fn serve_cards(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    let repo = cx.app_data().lock().unwrap();
    eprintln!("serve_cards");
    let uuids: Vec<Uuid> = repo.cards().keys().cloned().collect();
    Ok(tide::response::json(uuids))
}

async fn serve_card(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    let repo = cx.app_data().lock().unwrap();
    let uuid: Uuid = cx.param("uuid").map_err(|_| StatusCode::BAD_REQUEST)?;
    eprintln!("serve_card uuid={}", uuid);
    let card = repo.card(uuid);
    Ok(tide::response::json(card))
}

async fn quiz_cards(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    let repo = cx.app_data().lock().unwrap();
    let language: Language = cx.param("lang").map_err(|_| StatusCode::BAD_REQUEST)?;
    eprintln!("quiz_cards language={:?}", language);

    let suitable_questions = SUITABLE_QUESTIONS
        .iter()
        .filter_map(|(l, qks)| if *l == language { Some(*qks) } else { None })
        .next()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let rng = &mut rand::thread_rng();
    let cards = selection::expired_cards(rng, &repo, &suitable_questions);
    eprintln!("cards={:?}", cards.len());
    Ok(tide::response::json(cards))
}

async fn transliterate(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    eprintln!("transliterate");
    let language: Language = cx.param("lang").map_err(|_| StatusCode::BAD_REQUEST)?;
    eprintln!("transliterate={:?}", language);

    // FIXME(tide) -- this all looks like tide bugs to me
    let text: String = cx.param("text*").map_err(|_| StatusCode::BAD_REQUEST)?;
    let text: String = percent_encoding::percent_decode(text.as_bytes()).decode_utf8().map_err(|_| StatusCode::BAD_REQUEST)?.into_owned();

    eprintln!("transliterate={:?}", text);
    let out_text = language.transliterate(&text);
    eprintln!("transliterate={:?}", out_text);
    Ok(tide::response::json(out_text))
}

async fn check_answer(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    eprintln!("check_answer");
    let expected: String = cx.param("expected").map_err(|_| StatusCode::BAD_REQUEST)?;
    let expected: String = percent_encoding::percent_decode(expected.as_bytes()).decode_utf8().map_err(|_| StatusCode::BAD_REQUEST)?.into_owned(); // TIDE bug?
    let user: String = cx.param("user").map_err(|_| StatusCode::BAD_REQUEST)?;
    let user: String = percent_encoding::percent_decode(user.as_bytes()).decode_utf8().map_err(|_| StatusCode::BAD_REQUEST)?.into_owned(); // TIDE bug?
    let result = quiz::check_user_response(&expected, &user);
    eprintln!("expected={:?} user={:?} result={:?}", expected, user, result);
    Ok(tide::response::json(result))
}

async fn mark_answer(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    eprintln!("mark_answer");
    let uuid: Uuid = cx.param("uuid").map_err(|_| StatusCode::BAD_REQUEST)?;
    let from: Language = cx.param("from").map_err(|_| StatusCode::BAD_REQUEST)?;
    let to: Language = cx.param("to").map_err(|_| StatusCode::BAD_REQUEST)?;
    let response: QuestionResult = cx.param("response").map_err(|_| StatusCode::BAD_REQUEST)?;

    let question_kind = QuestionKind::Translate { from, to };

    let mut repo = cx.app_data().lock().unwrap();
    let record = repo.database_mut().card_record_mut(uuid);
    record.push_question_record(
        question_kind,
        QuestionRecord {
            date: Utc::now(),
            result: response,
        },
    );

    Ok(tide::response::json("ok"))
}

async fn write_db(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    eprintln!("write_db");
    let mut repo = cx.app_data().lock().unwrap();
    repo.write_database().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(tide::response::json("ok"))
}

async fn serve_asset(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    eprintln!("serve_asset(cx.uri().path() = {:?})", cx.uri().path());
    serve_path(cx.uri().path()).await
}

async fn serve_index(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    eprintln!("serve_index(cx.uri().path() = {:?})", cx.uri().path());
    serve_path("index.html").await
}

async fn serve_path(path: &str) -> tide::EndpointResult {
    let data = Asset::get(&path[1..]).ok_or(StatusCode::NOT_FOUND)?;

    let content_type: Option<&str> = try {
        match Path::new(path).extension()?.to_str()? {
            "json" => "application/json",
            "html" => "text/html;charset=UTF-8",
            "js" => "text/javascript;charset=UTF-8",
            "ico" => "image/vnd.microsoft.icon",
            "css" => "text/css;charset=UTF-8",
            _ => None?,
        }
    };

    Ok(
        http::Response::builder()
            .status(http::status::StatusCode::OK)
            .header("Content-Type", content_type.unwrap_or("application/octet-stream"))
            .body(Body::from(&data[..]))
            .unwrap()
    )
}

crate fn serve(options: &MathemaOptions) -> Fallible<()> {
    try {
        let mut repo = MathemaRepository::open(options)?;
        let status = repo.load_cards()?;
        if status.warn_if_needed(options.force) {
            return Ok(());
        }

        let mut app = tide::App::new(Mutex::new(repo));
        app.at("/api/cards").get(serve_cards);
        app.at("/api/card/:uuid").get(serve_card);
        app.at("/api/quiz_cards/:lang").get(quiz_cards);
        app.at("/api/transliterate/:lang/:text*").get(transliterate);
        app.at("/api/check_answer/:expected/:user").get(check_answer);
        app.at("/api/mark_answer/:uuid/translate/:from/:to/:response").post(mark_answer);
        app.at("/api/write_db").post(write_db);

        // Register the static assets. I don't think that tide supports a fallback,
        // which is annoying.
        app.at("/").get(serve_index);
        for file in Asset::iter() {
            assert!(!file.contains(":"));
            app.at(&file).get(serve_asset);
            eprintln!("serving assert {:?}", file);
        }

        app.serve("127.0.0.1:8000")?;
    }
}
