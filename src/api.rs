use std::sync::Arc;

use serenity::{
    cache::CacheRwLock,
    model::{
        id::{ChannelId, GuildId, UserId},
        voice::VoiceState, 
    },
    prelude::{Mutex, TypeMapKey},
};

use reqwest;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSettings {
    pub yt_url: String,
    pub special_channel_id: ChannelId,
    pub volume: f32,
}

pub struct Api {
    api_client: reqwest::blocking::Client,
    api_host: String,
}

impl Api {
    pub fn new(host: &str) -> Self {
        Api {
            api_host: host.to_string(),
            api_client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get_guild_settings(&self, guild_id: GuildId) -> Option<ApiSettings> {
        let url = [
            self.api_host.as_str(),
            "/guilds/",
            guild_id.to_string().as_str(),
            "/static",
        ].concat();

        self.get_json_safe(&url)
    }

    fn get_json_safe<T: DeserializeOwned>(&self, url: &str) -> Option<T> {
        if let Ok(response) = self.api_client.get(url).send() {
            if response.status().is_success() {
                if let Ok(data) = response.json::<T>() {
                    return Some(data);
                }
            }
        }

        None
    }

    pub fn check_is_someone_in_special_channel(special_channel_id: ChannelId, cache: &CacheRwLock) -> bool {
        if let Some(channel) = cache.read().guild_channel(special_channel_id) {
            if let Ok(members) = channel.read().members(&cache) {
                return members.iter().any(|m| !m.user.read().bot);
            }
        }

        false
    }

    pub fn is_channel_changed(old: Option<VoiceState>, new: VoiceState) -> bool {
        let zero_id = ChannelId(0);
        let new_channel_id = new.channel_id.unwrap_or(zero_id);

        if !old.is_some() && new_channel_id != zero_id {
            return true;
        }

        let old_channel_id = old.unwrap().channel_id.unwrap_or(zero_id);

        if old_channel_id != zero_id && new_channel_id == zero_id {
            return true
        }

        return old_channel_id == new_channel_id;
    }

    pub fn is_bot(user_id: &UserId, cache: &CacheRwLock) -> bool {
        if let Some(user) = cache.read().user(user_id) {
            let user_data = user.read();

            return user_data.bot;
        }

        false
    }
}

impl TypeMapKey for Api {
    type Value = Arc<Mutex<Api>>;
}
