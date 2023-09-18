use crate::{get_first_arg, is_owner, owner_check, send_message, storage_try, STORAGE};
use carapax::{
    methods::SendMessage,
    types::{ChatId, Integer, ParseMode, Text, User},
    Api, ExecuteError, Ref,
};

pub(crate) async fn allowuser(
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
        "ℹ️ Usage: /allowuser [user ID]"
    );

    let Ok(uid) = arg.parse::<Integer>() else {
        send_message!(api, chat_id, "🛑 Invalid user ID provided").await?;
        return Ok(());
    };

    storage_try!(api, chat_id, allow_user, &uid, disallow_user);

    send_message!(
        api,
        chat_id,
        format!("✅ Successfully allowed _{uid}_ to use this bot"),
        ParseMode::Markdown
    )
    .await?;

    Ok(())
}
