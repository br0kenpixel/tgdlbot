use crate::{get_first_arg, is_owner, owner_check, send_message, storage_try, STORAGE};
use carapax::{
    methods::SendMessage,
    types::{ChatId, Integer, ParseMode, Text, User},
    Api, ExecuteError, Ref,
};

pub(crate) async fn deluser(
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
        "ℹ️ Usage: /deluser [user ID]"
    );

    let Ok(uid) = arg.parse::<Integer>() else {
        send_message!(api, chat_id, "🛑 Invalid user ID provided").await?;
        return Ok(());
    };

    storage_try!(api, chat_id, disallow_user, &uid, allow_user);

    send_message!(
        api,
        chat_id,
        format!("✅ Successfully revoked access for _{uid}_"),
        ParseMode::Markdown
    )
    .await?;

    Ok(())
}
