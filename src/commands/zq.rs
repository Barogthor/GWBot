use serenity::framework::standard::macros::command;
use serenity::client::{Context};
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use chrono::{Utc, FixedOffset, TimeZone, Date, DateTime, Timelike};
use serenity::utils::MessageBuilder;
use crate::constants::{ZAISHEN_MISSION_START, ZAISHEN_MISSION_SIZE_CYCLE, ZAISHEN_BOUNTY_START, ZAISHEN_BOUNTY_SIZE_CYCLE, ZAISHEN_COMBAT_START, ZAISHEN_COMBAT_SIZE_CYCLE, ZAISHEN_VANQUISH_START, ZAISHEN_VANQUISH_SIZE_CYCLE};

fn get_timezone_start(start: &(i32, u32, u32)) -> DateTime<FixedOffset> {
    Utc.ymd(start.0, start.1, start.2).and_hms(16,0,0).with_timezone(&FixedOffset::east(2*3600))
}

fn get_mz_cycle_start() -> DateTime<FixedOffset> {
    let zq = ZAISHEN_MISSION_START;
    get_timezone_start(&zq)
}
fn get_bz_cycle_start() -> DateTime<FixedOffset> {
    let zq = ZAISHEN_BOUNTY_START;
    get_timezone_start(&zq)
}
fn get_cz_cycle_start() -> DateTime<FixedOffset> {
    let zq = ZAISHEN_COMBAT_START;
    get_timezone_start(&zq)
}
fn get_vz_cycle_start() -> DateTime<FixedOffset> {
    let zq = ZAISHEN_VANQUISH_START;
    get_timezone_start(&zq)
}

#[command]
async fn zq(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = msg.channel_id.to_channel(&ctx).await?.guild().unwrap();
    let guild = ctx.http.get_guild(channel.guild_id.0).await?;
    println!("{}", guild.region);
    let now =  Utc::now().with_timezone(&FixedOffset::east(2*3600));
    let datetime_diff = |since| now.signed_duration_since(since).num_days();
    let mz_id = datetime_diff(get_mz_cycle_start()) % ZAISHEN_MISSION_SIZE_CYCLE;
    let bz_id = datetime_diff(get_bz_cycle_start()) % ZAISHEN_BOUNTY_SIZE_CYCLE;
    let cz_id = datetime_diff(get_cz_cycle_start()) % ZAISHEN_COMBAT_SIZE_CYCLE;
    let vz_id = datetime_diff(get_vz_cycle_start()) % ZAISHEN_VANQUISH_SIZE_CYCLE;
    println!("{} {} {} {} ", mz_id, bz_id, cz_id, vz_id);

    let mut response = MessageBuilder::new();
    response
        .push_underline_line("Today's Zaishen Quests:")
        .push("Zaishen Mission: ")
        .push_bold_line(mz_id)
        .push("Zaishen Bounty: ")
        .push_bold_line(bz_id)
        .push("Zaishen Combat: ")
        .push_bold_line(cz_id)
        .push("Zaishen Vanquish: ")
        .push_bold_line(vz_id)
        .push("Zaishen daily quests will reset in ")
        .push_bold("");

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    
    Ok(())
}