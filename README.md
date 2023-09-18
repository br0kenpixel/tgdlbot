# Telegram Video Downloader Bot
A Telegram bot for downloading videos from various different sites like YouTube, Reddit and [others](https://github.com/yt-dlp/yt-dlp/blob/master/supportedsites.md). It uses [`yt-dlp`](https://github.com/yt-dlp/yt-dlp) as a backend.

## Note
```
⚠️ This is beta software. Some features might not work properly.
```

# Setup
1. Create a Telegram bot using [@BotFather](https://t.me/BotFather).
2. Rename [`.cargo/config.toml.example`](.cargo/config.toml.example) to `config.toml`.
3. In this file, you need to fill in your bot token and the ID of *your* Telegram account, so the bot can allow you to use certain admin-only commands and bypass some restrictions.
    Example:
    ```toml
    [env]
    BOT_TOKEN = "1234567890:ABCDEFGHIJKLMNOPQRSTUVXYZ"
    OWNER_ID = "0123456789"
    STORAGE_FILE = "~/tgdlstorage"
    ```
3. _(Optional)_ Change the location of the storage file (`STORAGE_FILE`) if needed.
4. Build and run the bot.

# Usage
Just send a link to the bot and it will begin downloading.
Commands can only be used by the owner of the bot.