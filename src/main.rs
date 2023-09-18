use appstorage::AppStorage;
use carapax::{
    longpoll::LongPoll, types::User, Api, App, Chain, CommandExt, Context, ErrorExt, HandlerError,
};
use std::{error::Error, sync::OnceLock};

pub mod appstorage;
mod commands;
pub mod downloader;
mod macros;

pub static STORAGE: AppStorage = AppStorage::new();
pub static STORAGE_PATH: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot = Api::new(env!("BOT_TOKEN"))?;
    let mut context = Context::default();
    STORAGE.init().await;
    STORAGE.load_from_disk().await?;

    if let Err(why) = env!("OWNER_ID").parse::<i64>() {
        return Err(format!("Invalid `OWNER_ID` - cannot be parsed: {why}").into());
    }

    let handler = Chain::all()
        .add(commands::help::help.command("/help"))
        .add(commands::start::start.command("/start"))
        .add(commands::stop::stop.command("/stop"))
        .add(commands::allowuser::allowuser.command("/allowuser"))
        .add(commands::deluser::deluser.command("/deluser"))
        .add(commands::allowchat::allowchat.command("/allowchat"))
        .add(commands::delchat::delchat.command("/delchat"))
        .add(commands::download::download)
        .on_error(error_handler);
    context.insert(bot.clone());

    LongPoll::new(bot, App::new(context, handler)).run().await;
    Ok(())
}

async fn error_handler(err: HandlerError) -> HandlerError {
    eprintln!("Got an error in custom error handler: {err}");
    err
}

pub fn is_owner(user: &User) -> bool {
    user.id == unsafe { env!("OWNER_ID").parse::<i64>().unwrap_unchecked() }
}
