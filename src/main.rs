use actix_web::{error, http, server, App, Json, Path, Result, State};

use failure::Fail;
use wx::domain::Event;
use wx::store::Client;

struct AppState {
    store_client: Client,
}

#[derive(Fail, Debug)]
#[fail(display = "api error")]
struct ApiError {
    msg: &'static str,
}

impl error::ResponseError for ApiError {}

fn main() {
    server::new(|| {
        App::with_state(AppState {
            store_client: Client::default(),
        })
        .resource("/events/{ts}", |r| {
            r.method(http::Method::GET).with(events_handler)
        })
        .resource("/all", |r| {
            r.method(http::Method::GET).with(all_events_handler)
        })
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
}

fn events_handler(state: State<AppState>, info: Path<u64>) -> Result<Json<Vec<Event>>, ApiError> {
    // let start = wx::util::get_system_micros();
    let events_result = state.store_client.get_events(info.into_inner());
    // let end = wx::util::get_system_micros();
    println!("event_lookup: {}", "test");

    match events_result {
        Ok(events) => Ok(Json(events)),
        Err(_) => Err(ApiError {
            msg: "Unable to retrieve events",
        }),
    }
}

fn all_events_handler(state: State<AppState>) -> Result<Json<Vec<Event>>, ApiError> {
    let events_result = state.store_client.get_all_events();

    match events_result {
        Ok(events) => Ok(Json(events)),
        Err(_) => Err(ApiError {
            msg: "Unable to retrieve events",
        }),
    }
}
