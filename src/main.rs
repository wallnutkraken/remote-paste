use serde_derive::{Deserialize, Serialize};
use std::{error::Error, io::{self, Write}};

use telegram_bot::Api;

use crate::{bot::Bot, paster::Paster};

mod bot;
mod paster;

const CFG_NAME: &str = "paster-config.toml";

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PasterConfig {
    username: Option<String>,
    token: Option<String>,
}

impl ::std::default::Default for PasterConfig {
    fn default() -> Self {
        Self {
            username: None,
            token: None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cfg: PasterConfig = confy::load_path(CFG_NAME)?;
    if cfg.username == None || cfg.token == None {
        cfg = create_config()?;
        confy::store_path(CFG_NAME, cfg.clone())?;
    }

    let username = if let Some(name) = cfg.username {
        name
    } else {
        String::new()
    };

    println!("Starting remote-paste with username to listen to: {}", username);
    let client = Api::new(if let Some(token) = cfg.token {
        token
    } else {
        String::new()
    });

    let output_sink = Paster::new();

    let mut bot = Bot::new(client, username);
    bot.listen_and_paste(output_sink).await?;
    Ok(())
}

fn create_config() -> Result<PasterConfig, Box<dyn Error>> {
    let username = get_config_from_user("Enter username to listen to:")?;
    let token = get_config_from_user("Enter telegram bot token:")?;
    Ok(PasterConfig {
        username: Some(username),
        token: Some(token),
    })
}

fn get_config_from_user(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{} ", prompt);
	io::stdout().flush()?;
    let mut value = String::new();
    io::stdin().read_line(&mut value)?;
    Ok(value.trim().to_string())
}
