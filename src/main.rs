use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use hypertext::html_elements;
use hypertext::{rsx, rsx_move, Attribute, GlobalAttributes, Renderable};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[allow(unused, non_upper_case_globals)]
trait HtmxAttributes {
    // core
    const hx_get: Attribute = Attribute;
    const hx_post: Attribute = Attribute;
    // WARN: not supported by the hypertext crate
    const hx_on: Attribute = Attribute;
    const hx_push_url: Attribute = Attribute;
    const hx_select: Attribute = Attribute;
    const hx_select_oob: Attribute = Attribute;
    const hx_swap: Attribute = Attribute;
    const hx_swap_oob: Attribute = Attribute;
    const hx_target: Attribute = Attribute;
    const hx_trigger: Attribute = Attribute;
    const hx_vals: Attribute = Attribute;
    // additional
    const hx_boost: Attribute = Attribute;
    const hx_confirm: Attribute = Attribute;
    const hx_delete: Attribute = Attribute;
    const hx_disable: Attribute = Attribute;
    const hx_disable_elt: Attribute = Attribute;
    const hx_disinherit: Attribute = Attribute;
    const hx_encoding: Attribute = Attribute;
    const hx_ext: Attribute = Attribute;
    const hx_headers: Attribute = Attribute;
    const hx_history: Attribute = Attribute;
    const hx_history_elt: Attribute = Attribute;
    const hx_include: Attribute = Attribute;
    const hx_indicator: Attribute = Attribute;
    const hx_params: Attribute = Attribute;
    const hx_patch: Attribute = Attribute;
    const hx_preserve: Attribute = Attribute;
    const hx_prompt: Attribute = Attribute;
    const hx_put: Attribute = Attribute;
    const hx_replace_url: Attribute = Attribute;
    const hx_request: Attribute = Attribute;
    const hx_sync: Attribute = Attribute;
    const hx_validate: Attribute = Attribute;
}

impl<T> HtmxAttributes for T where T: GlobalAttributes {}

#[derive(Clone)]
struct AppState {
    counter: Arc<AtomicUsize>,
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index))
        .route("/clicked", post(clicked))
        .with_state(AppState {
            counter: Arc::new(AtomicUsize::new(0)),
        })
        .nest_service("/static", ServeDir::new("static"));

    let listener = TcpListener::bind("::0:8000").await.unwrap();
    let _ = axum::serve(listener, router).await;
}

async fn index(State(s): State<AppState>) -> impl IntoResponse {
    let click_count = s.counter.load(Ordering::SeqCst);
    rsx! {
        <!DOCTYPE html>
        <html>
            <head>
                <script src="https://unpkg.com/htmx.org@2.0.4"></script>
                <link rel="stylesheet" href="/static/stylesheet.css">
            </head>
            <body>
                {counter(click_count)}
                <button hx-post="/clicked" hx-target="#counter" class="border-2">Click Me!</button>
            </body>
        </html>
    }
    .render()
}

async fn clicked(State(state): State<AppState>) -> impl IntoResponse {
    let _ = state.counter.fetch_add(1, Ordering::SeqCst);
    let click_count = state.counter.load(Ordering::SeqCst);

    counter(click_count).render()
}

fn counter(count: usize) -> impl Renderable {
    rsx_move! {
        <div class="text-red-500" id="counter">
            {format!("Clicked: {count}")}
        </div>
    }
}
