

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Command {
    pub arguments: Option<String>,
    pub channel: String,
    pub bot: Bot,
    pub caller: Caller,
    pub server: Server,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bot {
    pub nickname: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Caller {
    pub nickname: String,
    pub mode: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server  {
    pub hostname: String,
    pub port: u64,
    pub tls: bool,
}

#[cfg(test)]
mod tests {
}
