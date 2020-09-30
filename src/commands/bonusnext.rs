use serenity::framework::standard::macros::command;
use serenity::client::{Context};
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;
use chrono::{Utc, FixedOffset, DateTime, Timelike, Date, TimeZone, Datelike};
use serenity::utils::MessageBuilder;
use crate::constants::{
    BONUS_EVENT_PVE_SIZE_CYCLE,
    BONUS_EVENT_PVP_SIZE_CYCLE,
    BONUS_EVENT_START,
    BONUS_PVE_EVENTS,
    BONUS_PVP_EVENTS,
};
use crate::enums::Language::French;
use crate::utils::BonusEventData;
use std::ops::Add;
use std::borrow::BorrowMut;

fn get_timezone_start(date: Date<Utc>) -> DateTime<FixedOffset> {
    date.and_hms(16,0,0).with_timezone(&FixedOffset::east(2*3600))
}

fn get_bonus_cycle_start() -> Date<Utc> {
    let zq = BONUS_EVENT_START;
    Utc.ymd(zq.0, zq.1, zq.2)
}

#[command]
async fn bonusnext(ctx: &Context, msg: &Message) -> CommandResult {
    
    let now =  Utc::now().with_timezone(&FixedOffset::east(2*3600));
    let next_week =  Utc::now().add(chrono::Duration::weeks(1)).with_timezone(&FixedOffset::east(2*3600));
    let datetime_diff = |since| next_week.signed_duration_since(get_timezone_start(since)).num_weeks();
    let pve_id = datetime_diff(get_bonus_cycle_start()) % BONUS_EVENT_PVE_SIZE_CYCLE;
    let pvp_id = datetime_diff(get_bonus_cycle_start()) % BONUS_EVENT_PVP_SIZE_CYCLE;
    let days_left = 7 - now.weekday().num_days_from_monday() - 1;
    let hour_left = if now.hour() > 18 {
        24 - now.hour() + 18
    } else {
        18 - now.hour()
    } - 1;
    let min_left = 60 - now.minute() - 1;
    let sec_left = 60 - now.second() - 1;
    let current_pve: &BonusEventData = BONUS_PVE_EVENTS.get(&French).unwrap().get_from_id(pve_id).unwrap();
    let current_pvp: &BonusEventData = BONUS_PVP_EVENTS.get(&French).unwrap().get_from_id(pvp_id).unwrap();
    let mut response = MessageBuilder::new();
    response
        .push_underline_line("For next week:")
        .push("PvE bonus: ")
        .push(current_pve.name.clone()).push(" -- ")
        .push_bold_line(current_pve.description.clone())
        .push("PvP: ")
        .push(current_pvp.name.clone()).push(" -- ")
        .push_bold_line(current_pvp.description.clone())
        .push("Those Weekly bonuses will take effect in ")
        .push_bold_line(format!("{} days {:0>2}:{:0>2}:{:0>2}!", days_left, hour_left, min_left, sec_left));

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    
    Ok(())
}