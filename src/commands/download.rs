use crate::{allowed_check, downloader::DownloadManager, send_message, STORAGE};
use carapax::{
    methods::{EditMessageText, SendMessage, SendVideo},
    types::{ChatId, InputFile, Text, User},
    Api, ExecuteError, Ref,
};

pub(crate) async fn download(
    api: Ref<Api>,
    chat_id: ChatId,
    message: Text,
    user: User,
) -> Result<(), ExecuteError> {
    allowed_check!(api, chat_id, user);

    let video_link = message.data;
    let message = send_message!(api, chat_id.clone(), "⏳ Please wait...").await?;

    let file = match DownloadManager::download(&video_link).await {
        Ok(bytes) => bytes,
        Err(why) => {
            api.execute(EditMessageText::new(
                chat_id,
                message.id,
                format!("🛑 Failed to process request: {why}"),
            ))
            .await?;
            return Ok(());
        }
    };

    api.execute(EditMessageText::new(
        chat_id.clone(),
        message.id,
        "✅ Video downloaded successfully, sending...",
    ))
    .await?;

    let ifile = InputFile::path(file.path()).await.unwrap();
    api.execute(SendVideo::new(chat_id, ifile)).await?;

    Ok(())
}
