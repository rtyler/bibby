use irc::client::prelude::*;
use async_std::stream::StreamExt;

#[async_std::main]
async fn main() -> Result<(), irc::error::Error> {
    pretty_env_logger::init();
    // configuration is loaded from config.toml into a Config
    let mut client = Client::new("bot.toml").await?;
    // identify comes from ClientExt
    client.identify()?;

    let mut stream = client.stream()?;

    while let Some(message) = stream.next().await.transpose()? {
        if let Command::PRIVMSG(channel, message) = message.command {
            if message.contains(&*client.current_nickname()) {
                // send_privmsg comes from ClientExt
                client.send_privmsg(&channel, "beep boop").unwrap();
            }
        }
    }
    Ok(())
}
