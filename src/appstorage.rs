use crate::is_owner;
use carapax::types::{Integer, User};
use std::{io::ErrorKind, ops::Deref, path::PathBuf};
use tokio::{
    fs, io,
    sync::{Mutex as AsyncMutex, MutexGuard},
};

type UserIdType = Integer;
type ChatIdType = Integer;
type ChatIdList = Vec<ChatIdType>;
type UserIdList = Vec<UserIdType>;

#[derive(Debug)]
pub struct AppStorage {
    allowed_chats: AsyncMutex<ChatIdList>,
    allowed_users: AsyncMutex<UserIdList>,
    storage_location: AsyncMutex<String>,
}

impl AppStorage {
    pub const fn new() -> Self {
        Self {
            allowed_chats: AsyncMutex::const_new(ChatIdList::new()),
            allowed_users: AsyncMutex::const_new(UserIdList::new()),
            storage_location: AsyncMutex::const_new(String::new()),
        }
    }

    pub async fn init(&self) {
        let path = shellexpand::tilde(env!("STORAGE_FILE")).to_string();
        let mut sl_ref = self.storage_location.lock().await;

        *sl_ref = path;
    }

    pub async fn allow_chat(&self, chat_id: &ChatIdType) {
        let mut chats = self.allowed_chats_mut().await;

        if chats.contains(chat_id) {
            return;
        }

        chats.push(*chat_id);
    }

    pub async fn disallow_chat(&self, chat_id: &ChatIdType) {
        let mut chats = self.allowed_chats_mut().await;

        chats.retain(|cid| cid != chat_id)
    }

    pub async fn allow_user(&self, user_id: &UserIdType) {
        let mut users = self.allowed_users_mut().await;

        if users.contains(user_id) {
            return;
        }

        users.push(*user_id);
    }

    pub async fn disallow_user(&self, user_id: &UserIdType) {
        let mut users = self.allowed_users_mut().await;

        users.retain(|uid| uid != user_id)
    }

    pub async fn is_chat_allowed(&self, chat_id: &ChatIdType) -> bool {
        self.allowed_chats_mut().await.contains(chat_id)
    }

    pub async fn is_user_allowed(&self, user_id: &UserIdType) -> bool {
        self.allowed_users_mut().await.contains(user_id)
    }

    pub async fn accept_request(&self, from_user: &User, from_chat: Option<&ChatIdType>) -> bool {
        if is_owner(from_user) {
            return true;
        }

        match from_chat {
            Some(value) if self.is_chat_allowed(value).await => true,
            _ => self.is_user_allowed(&from_user.id).await,
        }
    }

    pub async fn load_from_disk(&self) -> Result<(), io::Error> {
        let storage_location = self.storage_location().await;

        if !storage_location.exists() {
            eprintln!("Initializing default storage");
            return self.save_to_disk().await;
        }

        let mut chats = self.allowed_chats_mut().await;
        let mut users = self.allowed_users_mut().await;
        let storage_raw = fs::read_to_string(storage_location).await?;

        let (saved_chats, saved_users): (ChatIdList, UserIdList) =
            serde_json::from_str(&storage_raw).map_err(|e| {
                io::Error::new(
                    ErrorKind::Other,
                    format!("Failed to parse storage file: {e}"),
                )
            })?;

        *chats = saved_chats;
        *users = saved_users;

        Ok(())
    }

    pub async fn save_to_disk(&self) -> Result<(), io::Error> {
        let chats = self.allowed_chats_mut().await;
        let users = self.allowed_users_mut().await;
        let content = (chats.deref(), users.deref());
        let raw = serde_json::to_string_pretty(&content).map_err(|e| {
            io::Error::new(
                ErrorKind::Other,
                format!("Failed to serialize storage data: {e}"),
            )
        })?;

        fs::write(self.storage_location().await, raw.as_bytes()).await
    }

    async fn storage_location(&self) -> PathBuf {
        PathBuf::from(self.storage_location.lock().await.deref())
    }

    async fn allowed_chats_mut(&self) -> MutexGuard<'_, ChatIdList> {
        self.allowed_chats.lock().await
    }

    async fn allowed_users_mut(&self) -> MutexGuard<'_, UserIdList> {
        self.allowed_users.lock().await
    }
}
