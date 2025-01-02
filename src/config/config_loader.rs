use anyhow::{Ok, Result};

use super::{config_model::{AdventuresSecret, Database, DotEnvyConfig, GuildCommandersSecret, Server}, stage::Stage};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT").expect("SERVER_PORT is not invalid").parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT").expect("SERVER_BODY_LIMIT is not invalid").parse()?,
        timeout: std::env::var("SERVER_TIMEOUT").expect("SERVER_TIMEOUT is not invalid").parse()?,
    };

    let database: Database =Database { url: std::env::var("DATABASE_URL").expect("DATABASE_URL is not invalid"), };


    Ok(DotEnvyConfig {
        server,
        database,
    })

}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()

}

pub fn get_adventurers_secret_env() -> Result<AdventuresSecret> {
    dotenvy::dotenv().ok();

    Ok(AdventuresSecret {
        secret: std::env::var("JWT_ADVENTURER_SECRET").expect("JWT_ADVENTURER_SECRET is not invalid"),
        refresh_secret: std::env::var("JWT_ADVENTURE_REFRESH_SECRET").expect("JWT_ADVENTURE_REFRESH_SECRET is not invalid"),
    })
}

pub fn get_guild_commanders_secret_env() -> Result<GuildCommandersSecret> {
    dotenvy::dotenv().ok();

    Ok(GuildCommandersSecret {
        secret: std::env::var("JWT_GUILD_COMMANDER_SECRET").expect("JWT_GUILD_COMMANDER_SECRET is not invalid"),
        refresh_secret: std::env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET").expect("JWT_GUILD_COMMANDER_REFRESH_SECRET is not invalid"),
    })
}