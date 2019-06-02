extern crate serenity;

use std::path::Path;
use std::env;
use dotenv;
use serenity::{
    model::{user::OnlineStatus, channel::Message, gateway::Ready, gateway::Activity},
    prelude::*
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("poop") {
            if let Err(why) = msg.channel_id.send_files(&ctx.http, vec!["./spoon.jpg"], |m| {
                m.content("")
            }) {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} connected!", ready.user.tag());
        ctx.set_presence(Some(Activity::playing("help")), OnlineStatus::Idle);
    }
}

fn main() {

    let image = Path::new("./spoon.jpg");

    if !image.exists() {
        panic!("Please put an image at spoon.jpg");
    }

    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Set DISCORD_TOKEN environment variable (.env: DISCORD_TOKEN=aaaa)");

    let mut client = Client::new(&token, Handler).expect("Error on creating client");
    if let Err(why) = client.start_autosharded() {
        println!("Client error: {:?}", why);
    }

}