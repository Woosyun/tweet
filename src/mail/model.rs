use serde::{Serialize, Deserialize};
use leptos::prelude::*;
use crate::mail::time::IntoRelativeTime;
use bson::DateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mail {
    #[serde(rename="_id")]
    pub id: String,
    pub author_id: String,
    pub body: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: DateTime,
}

impl Mail {
    pub fn new(author_id: String, body: String, tags: Vec<String>) -> Self {
        Self {
            id: String::from(""),
            author_id,
            body,
            tags,
            created_at: DateTime::now(),
        }
    }
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
}

impl IntoRender for Mail {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
        let relative_time = self.into_relative_time();

        view! {
            <div>
                <p class="text-sm">
                    <span class="text-inherit">{self.author_id}</span>
                    <span class="ml-4 text-inherit">{relative_time}</span>
                </p>
                <p>{self.body}</p>
            </div>
        }.into_any()
    }
}

impl IntoRelativeTime for Mail {
    fn get_created_at(&self) -> DateTime {
        self.created_at
    }
}
