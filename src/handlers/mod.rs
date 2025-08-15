use anyhow::Result;
use spin_sdk::http::{conversions::IntoBody, Response, ResponseBuilder};
mod docs;
pub(crate) mod todo;

pub(super) struct JsonResponse {}

impl JsonResponse {
    pub(super) fn from(payload: impl IntoBody) -> Result<Response> {
        Ok(ResponseBuilder::new(200)
            .header("content-type", "application/json")
            .body(payload)
            .build())
    }
}
