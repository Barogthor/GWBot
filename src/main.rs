extern crate dotenv;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::ops::Sub;

use chrono::Duration;
use chrono::prelude::*;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::group,
};
use serenity::framework::StandardFramework;
use serenity::model::prelude::Ready;

use commands::{
    bonus::*,
    bonusnext::*,
    event::*,
    menu::*,
    nick::*,
    nicknext::*,
    ping::*,
    skill::*,
    zq::*,
    zqnext::*,
};

use crate::utils::get_special_events_time_range;
use crate::utils::time::DateTimeRangeComparison;

pub mod constants;
pub mod enums;
mod commands;
pub mod utils;

#[group]
#[commands(ping, skill, menu, zq, zqnext, bonus, bonusnext, nick, nicknext, event)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{}#{} is connected!", ready.user.name, ready.user.discriminator);
    }
}

#[tokio::main]
async fn main() {
    let augury_rock = Utc.ymd(2011, 3, 3);
    let drake_kabob = Utc.ymd(2020, 9, 7);
    let augury_rock_2020 = Utc.ymd(2020, 3, 27);
    let raisu_2020 = Utc.ymd(2020, 3, 29);
    let frost_gate = Utc.ymd(2011, 5, 10);
    let diff = frost_gate.signed_duration_since(augury_rock);
    let diff2 = augury_rock_2020.signed_duration_since(augury_rock);
    let diff3 = raisu_2020.signed_duration_since(augury_rock);
    let diff4 = drake_kabob.sub(Duration::weeks(137));
    println!("{}", diff.num_days());
    println!("{}", diff2.num_days()%69);
    println!("{}", diff3.num_days()%69);
    println!("{}", diff4);
    // exit(0);
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-")) // set the bot's commands prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    // println!("{}", token);
    let mut client = Client::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    {
        // let mut data = client.data.write();
        // data.insert::<MessageEventCounter>(HashMap::default());
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}


// fn main() {
//     let code = "OQBDApwTOhwcgM4mmBaCeAUA".to_string();
//     let skill_record = SkillCodeParser::parse(code);
//     println!("{:?}", skill_record);
//     // Ok(())
// }
// https://wiki.guildwars.com/wiki/Skill_template_format#See_also
// https://wiki.guildwars.com/wiki/Talk:Skill_template_format
// https://wiki.guildwars.com/wiki/Template:Cycle
// https://wiki.guildwars.com/wiki/Zaishen_Mission/cycles
// https://wiki.guildwars.com/wiki/Zaishen_Bounty/cycles
// https://wiki.guildwars.com/wiki/Zaishen_Combat/cycles
// https://wiki.guildwars.com/wiki/Zaishen_Vanquish/cycles
// https://wiki.guildwars.com/wiki/Weekly_bonuses
// https://wiki.guildwars.com/wiki/Special_event
// https://kamadan-chat.com/Search.php?search={}
// 1.5-1.6mo
// During Wayfarer's Reverie, the following weekly bonuses are also active (source):
// Elonian Support Bonus
// Extra Luck Bonus
// Faction Support Bonus
// Northern Support Bonus