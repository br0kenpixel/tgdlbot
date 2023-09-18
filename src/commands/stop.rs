use crate::{is_owner, send_message};
use carapax::{
    methods::SendMessage,
    types::{ChatId, User},
    Api, ExecuteError, Ref,
};
use std::process::exit;

pub(crate) async fn stop(api: Ref<Api>, chat_id: ChatId, user: User) -> Result<(), ExecuteError> {
    if !is_owner(&user) {
        send_message!(
            api,
            chat_id.clone(),
            "⛔️ You are not authorized to use this command!"
        )
        .await?;

        return Ok(());
    }

    send_message!(api, chat_id, "Bye!").await?;
    exit(0);
}
