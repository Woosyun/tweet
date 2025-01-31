use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use mongodb::Database;
use serde::Deserialize;
use sha2::{Sha256, Digest};
use crate::user::{
    User, 
    UserService,
};


#[derive(Debug, Clone)]
pub struct SessionUser {
    pub user_name: String,
    pub user_id: String,
    pub password_hash: Vec<u8>,
}
impl SessionUser {
    pub fn hash_pw(pw: String) -> Vec<u8> {
        let mut hasher = Sha256::new();       // Create a SHA256 hasher instance
        hasher.update(pw.as_bytes());    // Feed the password bytes to the hasher
        let result = hasher.finalize();        // Get the hash result
        result.to_vec()   
    }
    pub fn from_user(user: User) -> Self {
        Self {
            user_name: user.user_name,
            user_id: user.user_id,
            password_hash: Self::hash_pw(user.password),
        }
    }
}

impl AuthUser for SessionUser {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.user_id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password_hash
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    user_id: String,
    password: String,
}

impl Credentials {
    pub fn new(id: String, pw: String) -> Result<Self, String> {
        Ok(Self {
            user_id: id,
            password: pw,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Backend {
    users: UserService,
}

impl Backend {
    pub async fn new(db: &Database) -> Self {
        Self {
            users: UserService::new(db),
        }
    }

    //TODO: implement register, find_one
    pub async fn register(&self, new_user: User) -> Result<(), String> {
        let user = self.users.find_one_by_user_id(new_user.user_id.clone()).await
            .map_err(|e| e.to_string())?;

        if !user.is_none() {
            return Err("you cannot use that user id!!".to_string());
        }

        self.users.insert_one(new_user).await
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = SessionUser;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        Credentials { user_id, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<SessionUser> = match self.users.find_one_by_user_id(user_id).await {
            Ok(Some(user)) => if user.password == password {
                Some(Self::User::from_user(user))
            } else {
                None
            },
            _ => None,
        };

        Ok(user)
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = match self.users.find_one_by_user_id(user_id.clone()).await {
            Ok(Some(user)) => Some(Self::User::from_user(user)),
            _ => None
        };

        Ok(user)
    }
}
