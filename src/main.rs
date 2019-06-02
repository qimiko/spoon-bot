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
        // bot check so it doesn't trigger itself or anything
        if msg.author.bot {
            return;
        }

        // this is likely incredibly inefficient but i can't think of a better way to do this
        let mut files = Vec::new();
        let mut content = String::new();

        let reactions_list = get_image_list();
        for reaction in &reactions_list {
            if msg.content.to_lowercase().contains(reaction.trigger) {
                match reaction.response {
                    ReactionType::Image => files.push(reaction.path),
                    ReactionType::Text => content.push_str(reaction.path),
                };
            }
        }

        if files.len() > 0 {
            if let Err(why) = msg.channel_id.send_files(&ctx.http, files, |m| {
                m.content(&content)
            }) {
                println!("Error sending message: {:?}", why);
            }
        } else if content.len() > 0 {
            if let Err(why) = msg.channel_id.say(&ctx.http, content) {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    
    fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} connected!", ready.user.tag());
        ctx.set_presence(Some(Activity::playing("Tria by ZikNeko")), OnlineStatus::Idle);
    }
}

#[derive(PartialEq)]
enum ReactionType {
    Image,
    Text,
}

struct Reaction<'a> {
    trigger: &'a str,
    path: &'a str,
    response: &'a ReactionType
}

fn get_image_list() -> Vec<Reaction<'static>> {
    // ok time to figure this out 
    let reactions = vec![
        Reaction {trigger: "bruh",    path: "bruh.png",   response: &ReactionType::Image}, 
        Reaction {trigger: "poop",    path: "spoon.jpg",  response: &ReactionType::Image},
        Reaction {trigger: "oman",    path: "banana.jpg", response: &ReactionType::Image},
        Reaction {trigger: "ayup",    path: "ayup.jpg",   response: &ReactionType::Image},
        Reaction {trigger: "what‘s",  path: "up.png",     response: &ReactionType::Image},
        Reaction {trigger: "what's",  path: "up.png",     response: &ReactionType::Image},
        Reaction {trigger: "pp niga", path: "pp niga",    response: &ReactionType::Text}];

    reactions
}

fn main() {
    let image_list = get_image_list();

    for reaction in &image_list {
        if !Path::new(reaction.path).exists() && reaction.response != &ReactionType::Text {
            panic!("Image {} does not exist in images/", reaction.path);
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