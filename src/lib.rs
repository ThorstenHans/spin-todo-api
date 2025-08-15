use spin_sdk::http::{IntoResponse, Request, Router};
use spin_sdk::http_component;

mod domain;
mod handlers;

#[http_component]
fn handle_spin_todo_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/api/todos", handlers::todo::get_all);
    router.get("/api/todos/:id", handlers::todo::get_by_id);
    router.post("/api/todos", handlers::todo::create_todo);
    router.post("/api/todos/:id/toggle", handlers::todo::toggle_by_id);
    router.delete("/api/todos/:id", handlers::todo::delete_by_id);
    Ok(router.handle(req))
}
