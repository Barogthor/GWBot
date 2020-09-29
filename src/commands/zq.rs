use serenity::framework::standard::macros::command;
use serenity::client::{Context};
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use chrono::{Utc, FixedOffset, DateTime, Timelike, Date, TimeZone, Datelike};
use serenity::utils::MessageBuilder;
use crate::constants::{
    ZAISHEN_MISSION_START,
    ZAISHEN_MISSION_SIZE_CYCLE,
    ZAISHEN_BOUNTY_START,
    ZAISHEN_BOUNTY_SIZE_CYCLE,
    ZAISHEN_COMBAT_START,
    ZAISHEN_COMBAT_SIZE_CYCLE,
    ZAISHEN_VANQUISH_START,
    ZAISHEN_VANQUISH_SIZE_CYCLE,
    ZAISHEN_MISSION_QUESTS,
    ZAISHEN_BOUNTY_QUESTS,
    ZAISHEN_COMBAT_QUESTS,
    ZAISHEN_VANQUISH_QUESTS,
};
use crate::enums::Language::French;
fn get_timezone_start(date: Date<Utc>) -> DateTime<FixedOffset> {
    date.and_hms(16,0,0).with_timezone(&FixedOffset::east(2*3600))
}

fn get_mz_cycle_start() -> Date<Utc> {
    let zq = ZAISHEN_MISSION_START;
    Utc.ymd(zq.0, zq.1, zq.2)
}
fn get_bz_cycle_start() -> Date<Utc> {
    let zq = ZAISHEN_BOUNTY_START;
    Utc.ymd(zq.0, zq.1, zq.2)
}
fn get_cz_cycle_start() -> Date<Utc> {
    let zq = ZAISHEN_COMBAT_START;
    Utc.ymd(zq.0, zq.1, zq.2)
}
fn get_vz_cycle_start() -> Date<Utc> {
    let zq = ZAISHEN_VANQUISH_START;
    Utc.ymd(zq.0, zq.1, zq.2)
}

#[command]
async fn zq(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = msg.channel_id.to_channel(&ctx).await?.guild().unwrap();
    let guild = ctx.http.get_guild(channel.guild_id.0).await?;
    println!("{}", guild.region);
    let now =  Utc::now().with_timezone(&FixedOffset::east(2*3600));
    let datetime_diff = |since| now.signed_duration_since(get_timezone_start(since)).num_days();
    let mz_id = datetime_diff(get_mz_cycle_start()) % ZAISHEN_MISSION_SIZE_CYCLE;
    let bz_id = datetime_diff(get_bz_cycle_start()) % ZAISHEN_BOUNTY_SIZE_CYCLE;
    let cz_id = datetime_diff(get_cz_cycle_start()) % ZAISHEN_COMBAT_SIZE_CYCLE;
    let vz_id = datetime_diff(get_vz_cycle_start()) % ZAISHEN_VANQUISH_SIZE_CYCLE;
    let hour_left = if now.hour() > 18 {
        24 - now.hour() + 18
    } else {
        18 - now.hour()
    } - 1;
    let min_left = 60 - now.minute() - 1;
    let sec_left = 60 - now.second() - 1;

    let mut response = MessageBuilder::new();
    response
        .push_underline_line("Today's Zaishen Quests:")
        .push("Zaishen Mission: ")
        .push_bold_line(ZAISHEN_MISSION_QUESTS.get(&French).unwrap().get_from_id(mz_id).unwrap().name.clone())
        .push("Zaishen Bounty: ")
        .push_bold_line(ZAISHEN_BOUNTY_QUESTS.get(&French).unwrap().get_from_id(bz_id).unwrap().name.clone())
        .push("Zaishen Combat: ")
        .push_bold_line(ZAISHEN_COMBAT_QUESTS.get(&French).unwrap().get_from_id(cz_id).unwrap().name.clone())
        .push("Zaishen Vanquish: ")
        .push_bold_line(ZAISHEN_VANQUISH_QUESTS.get(&French).unwrap().get_from_id(vz_id).unwrap().name.clone())
        .push("Zaishen daily quests will reset in ")
        .push_bold_line(format!("{:0>2}:{:0>2}:{:0>2}", hour_left, min_left, sec_left));

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    
    Ok(())
}