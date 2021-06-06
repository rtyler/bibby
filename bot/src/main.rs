use irc::client::prelude::*;
use log::*;
use async_std::stream::StreamExt;

use model::*;

#[async_std::main]
async fn main() -> Result<(), irc::error::Error> {
    pretty_env_logger::init();
    // configuration is loaded from config.toml into a Config
    let mut client = Client::new("bot.toml").await?;
    // identify comes from ClientExt
    client.identify()?;

    let mut stream = client.stream()?;

    while let Some(message) = stream.next().await.transpose()? {
        if let irc::client::prelude::Command::PRIVMSG(channel, message) = message.command {
            debug!("Received: {:?}", message);
            if message.contains(&*client.current_nickname()) {
                // send_privmsg comes from ClientExt
                client.send_privmsg(&channel, "beep boop").unwrap();
                let data = model::Command {
                    arguments: None,
                    channel: channel.clone(),
                    bot: Bot {
                        nickname: client.current_nickname().to_string(),
                    },
                    caller: Caller {
                        nickname: "blah".to_string(),
                        mode: "".to_string(),
                    },
                    server: Server {
                        hostname: "".to_string(),
                        port: 0,
                        tls: true,
                    },
                };

                let mut res = surf::post("http://localhost:8041/api/v0/time").body(surf::Body::from_json(&data).unwrap()).await.unwrap();
                assert_eq!(res.status(), 200);
                let m = res.body_string().await.unwrap();
                client.send_privmsg(&channel, m).unwrap();
            }
        }
    }
    Ok(())
}
