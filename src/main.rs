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

        if msg.author.bot {
            return;
        }

        // this is likely incredibly inefficient but i can't think of a better way to do this
        let mut files = Vec::new();

        let reactions_list = get_image_list();
        for reaction in &reactions_list {
            if msg.content.to_lowercase().contains(reaction.trigger) {
                files.push(reaction.path);
            }
        }

        if files.len() > 0 {
            if let Err(why) = msg.channel_id.send_files(&ctx.http, files, |m| {
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

struct Reaction<'a> {
    trigger: &'a str,
    path: &'a str
}

fn get_image_list() -> Vec<Reaction<'static>> {
    // ok time to figure this out 
    let reactions = vec![
        Reaction {trigger: "bruh",   path: "bruh.png"}, 
        Reaction {trigger: "poop",   path: "spoon.jpg"},
        Reaction {trigger: "oman",   path: "banana.jpg"},
        Reaction {trigger: "ayup",   path: "ayup.jpg"},
        Reaction {trigger: "what‘s", path: "up.png"},
        Reaction {trigger: "what's", path: "up.png"}];

    reactions
}

fn main() {

    let image_list = get_image_list();

    for reactions in &image_list {
        if !Path::new(reactions.path).exists() {
            panic!("Image {} does not exist in images/", reactions.path);
        }
    }

    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Set DISCORD_TOKEN environment variable (.env: DISCORD_TOKEN=aaaa)");

    let mut client = Client::new(&token, Handler).expect("Error on creating client");
    if let Err(why) = client.start_autosharded() {
        println!("Client error: {:?}", why);
    }
}