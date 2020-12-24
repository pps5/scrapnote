use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ItemType {
    File,
    Command,
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
}

#[derive(Deserialize, Serialize)]
pub struct GetFilesResponse {
    pub files: Vec<Item>,
}

#[derive(Deserialize, Serialize)]
pub struct GetFileContentResponse {
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct SaveFileContentRequest {
    pub content: String,
}
