use crate::{api::Api, voice_manager::VoiceManager};

use serenity::{
    client::{Context, EventHandler},
    model::{
        gateway::Ready,
        id::GuildId,
        voice::VoiceState,
    },
    voice,
};

pub struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn voice_state_update(&self, ctx: Context, guild_id: Option<GuildId>, old: Option<VoiceState>, new: VoiceState) {
        if !guild_id.is_some() || Api::is_bot(&new.user_id, &ctx.cache) || !Api::is_channel_changed(old, new) {
            return
        }

        let gid = guild_id.unwrap();

        let api_lock = ctx.data.read().get::<Api>().cloned().expect("Expected Api in ShareMap.");
        let api = api_lock.lock();

        let settings = match api.get_guild_settings(gid) {
            Some(settings) => settings,
            None => {
                println!("cannot find settings for guild {:?}", gid);

                return;
            }
        };

        let someone_in_special_channel = Api::check_is_someone_in_special_channel(settings.special_channel_id, &ctx.cache);

        let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
        let mut manager = manager_lock.lock();

        if let Some(handler) = manager.join(gid, settings.special_channel_id) {
            println!("connected to channel");

            if !someone_in_special_channel {
                handler.stop();
                println!("stop playing");

                return;
            }

            if let Ok(source) = voice::ytdl(settings.yt_url.as_str()) {
                let safe_audio = handler.play_only(source);

                {
                    let audio_lock = safe_audio.clone();
                    let mut audio = audio_lock.lock();

                    audio.volume(settings.volume);
                }

                println!("playing");
            } else {
                println!("Err starting source");
            }
        } else {
            println!("failed to connect to channel");
        }
    }
}
