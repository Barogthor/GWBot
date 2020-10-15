use chrono::{Date, Datelike, DateTime, FixedOffset, Timelike, TimeZone, Utc};
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

use crate::constants::{EMOTE_MAP, EMOTE_POINT_RIGHT, NICHOLAS_TRAVELER, NICHOLAS_TRAVELER_SIZE_CYCLE, NICHOLAS_TRAVELER_START};
use crate::enums::Language::French;
use crate::utils::NicholasGiftData;

fn get_timezone_start(date: Date<Utc>) -> DateTime<FixedOffset> {
    date.and_hms(16, 0, 0).with_timezone(&FixedOffset::east(2 * 3600))
}

fn get_cycle_start() -> Date<Utc> {
    let zq = NICHOLAS_TRAVELER_START;
    Utc.ymd(zq.0, zq.1, zq.2)
}

#[command]
async fn nick(ctx: &Context, msg: &Message) -> CommandResult {
    let now = Utc::now().with_timezone(&FixedOffset::east(2 * 3600));
    let datetime_diff = |since| now.signed_duration_since(get_timezone_start(since)).num_weeks();
    let gift_id = datetime_diff(get_cycle_start()) % NICHOLAS_TRAVELER_SIZE_CYCLE;
    let days_left = 7 - now.weekday().num_days_from_monday() - 1;
    let hour_left = if now.hour() > 18 {
        24 - now.hour() + 18
    } else {
        18 - now.hour()
    } - 1;
    let min_left = 60 - now.minute() - 1;
    let sec_left = 60 - now.second() - 1;

    let gift: &NicholasGiftData = NICHOLAS_TRAVELER.get(&French).unwrap().get_from_id(gift_id).unwrap();
    let mut response = MessageBuilder::new();
    response
        .push_underline_line("This week:")
        .push("Nicholas the traveler is collecting ")
        .push_bold(&gift.item)
        .push(" per present at ")
        .push_bold_line(&gift.location)
        .push("in ")
        .push_bold(&gift.region)
        .push(format!(" ({}).", gift.campaign))
        .push("Moving off in ")
        .push_bold_line(format!("{} days and {:0>2}:{:0>2}:{:0>2}!", days_left, hour_left, min_left, sec_left))
        .push(EMOTE_MAP).push(" ").push(EMOTE_POINT_RIGHT).push(" ")
        .push_spoiler_line(&gift.location_url);

    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(response);
        m
    }).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}