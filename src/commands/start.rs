use crate::send_message;
use carapax::{
    methods::SendMessage,
    types::{ChatId, ParseMode, User},
    Api, ExecuteError, Ref,
};

pub(crate) async fn start(api: Ref<Api>, chat_id: ChatId, user: User) -> Result<(), ExecuteError> {
    send_message!(
        api,
        chat_id.clone(),
        format!(concat!(
            "Hi, @{}!\n",
            "I'm a content downloader bot.\n",
            "Supported sites can be found [here](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md)."
        ), user.username.unwrap_or_else(|| "Unknown".to_string())),
        ParseMode::Markdown
    )
    .await?;

    Ok(())
}
