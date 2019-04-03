use actix_web::{server, App, HttpRequest, Json, Result};
use wx::domain::Event;
use wx::store::Client;

struct AppState {
    store_client: Client,
}

// todo how many contexts do i have? should be 1 shared
fn main() {
    server::new(|| {
        // App::new().resource("/events", |r| r.f(events))
        App::with_state(AppState { store_client: Client::default() })
            .resource("/events", |r| r.f(events))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}

// todo take optional offset and distance
fn events(req: &HttpRequest<AppState>) -> Result<Json<Vec<Event>>> {
    let events = req.state().store_client.get_all_events().unwrap();
    Ok(Json(events))
}
