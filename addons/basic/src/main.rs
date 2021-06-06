use log::*;
use model::*;
use tide::prelude::*;
use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    pretty_env_logger::init();
    let mut app = tide::new();
    app.at("/api/v0/time").post(cmd_time);
    app.at("/api/v0/cookie").post(cmd_cookie);
    if let Some(fd) = std::env::var("LISTEN_FD")
        .ok()
        .and_then(|fd| fd.parse().ok())
    {
        /*
         * Allow the use of catflag for local development
         *
         * <https://github.com/passcod/catflap>
         */
        use std::net::TcpListener;
        use std::os::unix::io::FromRawFd;
        app.listen(unsafe { TcpListener::from_raw_fd(fd) }).await?;
    } else {
        app.listen("127.0.0.1:8041").await?;
    }
    Ok(())
}

async fn cmd_cookie(mut req: Request<()>) -> tide::Result {
    let command: Command = req.body_json().await?;
    Ok(format!("*nom* *nom* thanks {}", command.caller.nickname).into())
}

async fn cmd_time(mut req: Request<()>) -> tide::Result {
    use chrono::prelude::*;

    let command: Command = req.body_json().await?;
    debug!("Received command: {:?}", command);

    if let Some(arguments) = &command.arguments {
        if arguments == "UTC" {
            Ok(Utc::now().to_rfc2822().into())
        } else {
            Ok(format!("I don't know how to handle: {}", arguments).into())
        }
    } else {
        Ok(Local::now().to_rfc2822().into())
    }
}
