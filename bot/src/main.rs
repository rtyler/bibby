use irc::client::prelude::*;
use log::*;
use async_std::stream::StreamExt;

use model::*;

#[async_std::main]
async fn main() -> Result<(), irc::error::Error> {
    pretty_env_logger::init();
    let config = Config::load("bot.toml").unwrap();
    // configuration is loaded from config.toml into a Config
    let mut client = Client::new("bot.toml").await?;
    // identify comes from ClientExt
    client.identify()?;

    let mut stream = client.stream()?;
    let regex = regex::Regex::new(r"^!(?P<action>\w+)[\s+]?(?P<arguments>.*)?").unwrap();

    while let Some(message) = stream.next().await.transpose()? {
        if let irc::client::prelude::Command::PRIVMSG(channel, text) = &message.command {
            debug!("Received: {:?}", message);
            if let Some(captured) = regex.captures(text) {
                if let Some(action) = captured.name("action") {

                    let arguments = match captured.name("arguments") {
                        Some(arguments) => {
                            let arguments = arguments.as_str();
                            if arguments.is_empty() {
                                None
                            } else {
                                Some(arguments.to_string())
                            }
                        },

                        None => None,
                    };

                    let data = model::Command {
                        arguments,
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
                    let mut res = surf::post(format!("http://localhost:8041/api/v0/{}", action.as_str()))
                        .body(surf::Body::from_json(&data).unwrap()).await.unwrap();
                    assert_eq!(res.status(), 200);
                    let m = res.body_string().await.unwrap();
                    client.send_privmsg(&channel, m).unwrap();
                }
            }
        }
    }
    Ok(())
}
