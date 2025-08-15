use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::key_value::Store;
use uuid::Uuid;

const KEY_PREFIX: &str = "todo-";

#[derive(Serialize, Deserialize)]
pub struct ToDo {
    pub id: Uuid,
    pub contents: String,
    pub is_completed: bool,
    pub is_deleted: bool,
}
impl ToDo {
    pub fn new(contents: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            contents,
            is_completed: false,
            is_deleted: false,
        }
    }

    fn build_key(id: Uuid) -> String {
        format!("{}{}", KEY_PREFIX, id)
    }

    fn get_key(&self) -> String {
        Self::build_key(self.id)
    }

    pub fn save(&self) -> Result<()> {
        let store = Store::open_default()?;
        store.set_json(self.get_key(), &self)
    }

    pub fn get_by_id(id: Uuid) -> Result<Option<Self>> {
        let store = Store::open_default()?;
        let key = Self::build_key(id);
        store.get_json::<Self>(key)
    }

    pub fn get_all() -> Result<Vec<Self>> {
        let store = Store::open_default()?;

        Ok(store
            .get_keys()?
            .iter()
            .filter(|key| key.starts_with(KEY_PREFIX))
            .filter_map(|key| store.get_json::<Self>(key).ok())
            .map(|td| td.unwrap())
            .collect())
    }
}
