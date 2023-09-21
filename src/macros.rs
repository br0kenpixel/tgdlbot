#[macro_export]
macro_rules! send_message {
    ($api: ident, $chat_id: expr, $text: expr) => {
        $api.execute(SendMessage::new($chat_id, $text))
    };

    ($api: ident, $chat_id: expr, $text: expr, $parse_mode: expr) => {
        $api.execute(SendMessage::new($chat_id, $text).parse_mode($parse_mode))
    };
}

#[macro_export]
macro_rules! owner_check {
    ($user: ident, $api: ident, $chat_id: expr) => {
        if !is_owner(&$user) {
            send_message!(
                $api,
                $chat_id.clone(),
                "⛔️ You are not authorized to use this command!"
            )
            .await?;

            return Ok(());
        }
    };
}

#[macro_export]
macro_rules! allowed_check {
    ($api: ident, $chat_id: ident, $user: ident) => {
        let chat_id_n = match &$chat_id {
            ChatId::Id(n) => Some(n),
            ChatId::Username(..) => None,
        };

        if !STORAGE.accept_request(&$user, chat_id_n).await {
            send_message!(
                $api,
                $chat_id.clone(),
                format!(
                    concat!(
                        "❌ You are not allowed to use this bot\n",
                        "► Please ask the administrator to allow you account - ",
                        "@{}"
                    ),
                    $user.id
                ),
                ParseMode::Markdown
            )
            .await?;

            if chat_id_n.is_some() {
                send_message!(
                    $api,
                    $chat_id.clone(),
                    "⚠️ Unable to verify access using chat ID"
                )
                .await?;
            }

            return Ok(());
        }
    };
}

#[macro_export]
macro_rules! get_first_arg {
    ($arg_out: ident, $message: ident, $api: ident, $chat_id: expr, $usage_text: literal) => {
        let $arg_out = match $message.data.split_once(' ').map(|v| v.1) {
            Some(arg) => arg,
            None => {
                send_message!($api, $chat_id, $usage_text, ParseMode::Markdown).await?;
                return Ok(());
            }
        };
    };
}

#[macro_export]
macro_rules! storage_try {
    ($api: ident, $chat_id: expr, $func: ident, $arg: expr, $undo_func: ident) => {
        STORAGE.$func($arg).await;
        if let Err(why) = STORAGE.save_to_disk().await {
            eprintln!("Failed to save storage to disk: {why}");
            STORAGE.$undo_func($arg).await;

            send_message!(
                $api,
                $chat_id,
                format!("⚠️ Could not save storage to disk: {why}")
            )
            .await?;

            return Ok(());
        }
    };
}
