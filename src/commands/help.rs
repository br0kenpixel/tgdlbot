use crate::{is_owner, send_message};
use carapax::{
    methods::SendMessage,
    types::{ChatId, ParseMode, User},
    Api, ExecuteError, Ref,
};

pub(crate) async fn help(api: Ref<Api>, chat_id: ChatId, user: User) -> Result<(), ExecuteError> {
    send_message!(
        api,
        chat_id.clone(),
        format!("Hi, @{}!", user.username.as_ref().unwrap())
    )
    .await?;

    if is_owner(&user) {
        send_message!(api, chat_id.clone(), "🚧 Administrator mode").await?;
        send_message!(
            api,
            chat_id,
            concat!(
                "These commands are supported:\n\n",
                "👤 /allowuser — _Allow a user to use this bot._\n",
                "📝 /allowchat — _Allow a chat to use this bot._\n",
                "👤 /deluser — _Disallow a user to use this bot._\n",
                "📝 /delchat — _Disallow a chat to use this bot._\n",
                "🔖 /manage — _Manage allowed chats & users._\n",
                "🛑 /stop — _Stop the bot._\n",
            ),
            ParseMode::Markdown
        )
        .await?;

        return Ok(());
    }

    send_message!(api, chat_id, "📎 Send a video link to download").await?;

    Ok(())
}
