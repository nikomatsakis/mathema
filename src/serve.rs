use std::sync::Mutex;
use crate::git::MathemaRepository;
use crate::errors::Fallible;
use crate::MathemaOptions;
use uuid::Uuid;

async fn serve_cards(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    let repo = cx.state().lock().unwrap();
    let uuids: Vec<Uuid> = repo.cards().keys().cloned().collect();
    Ok(tide::response::json(uuids))
}

async fn serve_card(cx: tide::Context<Mutex<MathemaRepository>>) -> tide::EndpointResult {
    let repo = cx.state().lock().unwrap();
    let uuid: Uuid = cx.param("uuid").unwrap();
    let card = repo.card(uuid);
    Ok(tide::response::json(card))
}

crate fn serve(options: &MathemaOptions) -> Fallible<()> {
    try {
        let mut repo = MathemaRepository::open(options)?;
        let status = repo.load_cards()?;
        if status.warn_if_needed(options.force) {
            return Ok(());
        }

        let mut app = tide::App::with_state(Mutex::new(repo));
        app.at("/cards").get(serve_cards);
        app.at("/card/:uuid").get(serve_card);
        app.serve("127.0.0.1:8000")?;
    }
}