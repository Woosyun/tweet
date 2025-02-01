use crate::mail::Mail;
use mongodb::{
    error::Error, 
    Collection, 
    results::InsertOneResult,
    bson,
    options::FindOptions,
    Database,
};
use futures::TryStreamExt;

#[derive(Debug, Clone)]
pub struct MailService {
    collection: Collection::<Mail>,
}

impl MailService {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection::<Mail>("mails"),
        }
    }

    pub async fn insert_one(&self, mut mail: Mail) -> Result<InsertOneResult, Error> {
        mail.set_id(bson::oid::ObjectId::new().to_hex());
        mail.set_last_modified(bson::DateTime::now().to_string());
        self.collection.insert_one(mail).await
    }

    pub async fn find_one_by_id(&self, id: String) -> Result<Option<Mail>, Error> {
        self.collection.find_one(bson::doc!{"_id": id}).await
    }

    pub async fn find_by_tags(&self, tags: Vec<String>) -> Result<Vec<Mail>, Error> {
        
        let options = FindOptions::builder()
            .limit(30)
            .sort(bson::doc! { "last_modified": -1 })
            .build();

        let query = if tags.is_empty() {
            bson::doc! {} 
        } else {
            bson::doc! {"tags": { "$all": tags }}
        };

        self.collection.find(query)
            .with_options(options)
            .await
            .map_err(|e| dbg!(e))?
            .try_collect()
            .await
            .map_err(|e| dbg!(e))
    }
}
