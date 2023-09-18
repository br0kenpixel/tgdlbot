use crate::{get_first_arg, is_owner, owner_check, send_message, storage_try, STORAGE};
use carapax::{
    methods::SendMessage,
    types::{ChatId, Integer, ParseMode, Text, User},
    Api, ExecuteError, Ref,
};

pub(crate) async fn allowchat(
    api: Ref<Api>,
    chat_id: ChatId,
    message: Text,
    user: User,
) -> Result<(), ExecuteError> {
    owner_check!(user, api, chat_id);

    get_first_arg!(
        arg,
        message,
        api,
        chat_id.clone(),
        "ℹ️ Usage: /allowchat [chat ID]"
    );

    let Ok(cid) = arg.parse::<Integer>() else {
        send_message!(api, chat_id, "🛑 Invalid chat ID provided").await?;
        return Ok(());
    };

    storage_try!(api, chat_id, allow_chat, &cid, disallow_chat);

    send_message!(
        api,
        chat_id,
        format!("✅ Successfully allowed group _{cid}_ to use this bot"),
        ParseMode::Markdown
    )
    .await?;

    Ok(())
}
