use tide::Request;
use tide::prelude::*;

use model::*;

#[async_std::main]
async fn main() -> tide::Result<()> {
    pretty_env_logger::init();
    let mut app = tide::new();
    app.at("/api/v0/time").post(cmd_time);
    app.listen("127.0.0.1:8041").await?;
    Ok(())
}

async fn cmd_time(mut req: Request<()>) -> tide::Result {
    Ok("hello worl".into())
}
