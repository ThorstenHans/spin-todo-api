use crate::domain;

use super::JsonResponse;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{
    conversions::IntoBody, IntoResponse, Params, Request, Response, ResponseBuilder,
};
use uuid::Uuid;

pub(crate) fn get_all(_req: Request, _p: Params) -> Result<impl IntoResponse> {
    let todos = domain::ToDo::get_all()?;
    let models = ToDoListModel::from(
        todos
            .into_iter()
            .filter(|i| !i.is_deleted)
            .map(ToDoModel::from)
            .collect::<Vec<_>>(),
    );

    JsonResponse::from(models)
}

pub(crate) fn get_by_id(_req: Request, p: Params) -> Result<impl IntoResponse> {
    let id = p.get("id").expect("router guarantees id is set");
    let Ok(id) = Uuid::parse_str(id) else {
        return Ok(Response::new(400, "Bad Request"));
    };
    match domain::ToDo::get_by_id(id)? {
        Some(todo) => JsonResponse::from(ToDoModel::from(todo)),
        None => Ok(Response::new(404, "Not Found")),
    }
}

pub(crate) fn delete_by_id(_req: Request, p: Params) -> Result<impl IntoResponse> {
    let id = p.get("id").expect("router guarantees id is set");
    let Ok(id) = Uuid::parse_str(id) else {
        return Ok(Response::new(400, "Bad Request"));
    };
    let Some(mut found) = domain::ToDo::get_by_id(id)? else {
        return Ok(Response::new(404, "Not Found"));
    };
    found.is_deleted = true;
    found.save()?;
    Ok(Response::new(204, ()))
}

pub(crate) fn toggle_by_id(_req: Request, p: Params) -> Result<impl IntoResponse> {
    let id = p.get("id").expect("router guarantees id is set");
    let Ok(id) = Uuid::parse_str(id) else {
        return Ok(Response::new(400, "Bad Request"));
    };
    let Some(mut found) = domain::ToDo::get_by_id(id)? else {
        return Ok(Response::new(404, "Not Found"));
    };
    found.is_completed = !found.is_completed;
    found.save()?;
    Ok(Response::new(204, ()))
}

pub(crate) fn create_todo(req: Request, _p: Params) -> Result<impl IntoResponse> {
    let Ok(model) = serde_json::from_slice::<CreateToDoModel>(req.body()) else {
        return Ok(Response::new(400, "Bad Request"));
    };
    let new_todo = domain::ToDo::new(model.contents);
    new_todo.save()?;
    Ok(ResponseBuilder::new(201)
        .header("location", format!("/api/todos/{}", new_todo.id))
        .header("content-type", "application/json")
        .body(ToDoModel::from(new_todo))
        .build())
}

#[derive(Deserialize)]
struct CreateToDoModel {
    pub contents: String,
}

struct ToDoListModel {
    items: Vec<ToDoModel>,
}

impl From<Vec<ToDoModel>> for ToDoListModel {
    fn from(value: Vec<ToDoModel>) -> Self {
        Self { items: value }
    }
}

impl IntoBody for ToDoListModel {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self.items).expect("Error while serializing ToDoListModel")
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ToDoModel {
    pub id: Uuid,
    pub contents: String,
    pub is_completed: bool,
}

impl IntoBody for ToDoModel {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self).expect("Error while serializing ToDoModel")
    }
}

impl From<domain::ToDo> for ToDoModel {
    fn from(value: domain::ToDo) -> Self {
        Self {
            id: value.id,
            contents: value.contents.clone(),
            is_completed: value.is_completed,
        }
    }
}
