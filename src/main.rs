mod api;
mod handler;
mod voice_manager;

use std::{env, sync::Arc};

use serenity::{
    prelude::Mutex,
    client::Client,
    framework::StandardFramework,
};

use crate::{
    api::Api,
    voice_manager::VoiceManager,
    handler::Handler,
};


fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let api_host = env::var("API_HOST")
        .expect("Expected an api host in the environment");

    let api = Api::new(&api_host);

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
    {
        let mut data = client.data.write();
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
        data.insert::<Api>(Arc::new(Mutex::new(api)));
    }
 
    client.with_framework(StandardFramework::new());

    let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
}
