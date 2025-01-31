use serde::{Serialize, Deserialize};
use leptos::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mail {
    #[serde(rename="_id")]
    pub id: String,
    pub author_id: String,
    pub body: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub last_modified: String,
}

impl Mail {
    pub fn new(author_id: String, body: String, tags: Vec<String>) -> Self {
        Self {
            id: String::from(""),
            author_id,
            body,
            tags,
            last_modified: String::from(""),
        }
    }
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    pub fn set_last_modified(&mut self, cur: String) {
        self.last_modified = cur;
    }
}

impl IntoRender for Mail {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
        view! {
            <div>
                <span>{self.author_id}</span>
                <p>{self.body}</p>
                <span>{self.last_modified}</span>
            </div>
        }.into_any()
    }
}
