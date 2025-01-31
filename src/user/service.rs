use super::User;

use mongodb::{
    bson, error::Error, results::InsertOneResult, Collection, Database
    // bson::DateTime,
    // options::FindOptions,
};
// use serde::{Serialize, Deserialize};
// use futures::stream::TryStreamExt;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct UserService {
    collection: Collection::<User>,
}

impl UserService {
    pub fn new(db: &Database) -> Self {
        //let user_col_name = std::env::var("MONGODB_USER_COLLECTION_NAME")
            //.expect("MONGODB_USER_COLLECTION_NAME must be set");
        let collection = db.collection::<User>("users");
        
        Self {
            collection
        }
    }
    pub async fn insert_one(&self, mut new_user: User) -> Result<InsertOneResult, Error> {
        new_user.set_id(bson::oid::ObjectId::new().to_hex());
        self.collection.insert_one(new_user).await
    }
    pub async fn find_one_by_id(&self, id: String) -> Result<Option<User>, Error> {
        self.collection.find_one(bson::doc! { "_id": id }).await
    }
    pub async fn find_one_by_user_id(&self, user_id: String) -> Result<Option<User>, Error> {
        self.collection.find_one(bson::doc! {"user_id": user_id}).await
    }
    // pub async fn find_one_by_user_name(&self, user_id)
}
