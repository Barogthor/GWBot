use chrono::{Date, Datelike, DateTime, Timelike, TimeZone, Utc};
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

use crate::constants::{
    BONUS_EVENT_PVE_SIZE_CYCLE,
    BONUS_EVENT_PVP_SIZE_CYCLE,
    BONUS_EVENT_START,
};
use crate::enums::Language::French;
use crate::get_bot_datas;
use crate::utils::{BonusEventStore, I18nMessageStore};

fn get_timezone_start(date: Date<Utc>) -> DateTime<Utc> {
    date.and_hms(16, 0, 0)
}

fn get_bonus_cycle_start() -> Date<Utc> {
    let zq = BONUS_EVENT_START;
    Utc.ymd(zq.0, zq.1, zq.2)
}

#[command]
async fn bonus(ctx: &Context, msg: &Message) -> CommandResult {
    let datas_lock = get_bot_datas(ctx).await;
    let read_data = &datas_lock.read().await;
    let i18n_messages: &I18nMessageStore = &read_data.i18n_messages.lng(French).unwrap();
    let bonus_pve: &BonusEventStore = &read_data.bonus_pve.lng(French).unwrap();
    let bonus_pvp: &BonusEventStore = &read_data.bonus_pvp.lng(French).unwrap();
    let now = Utc::now();
    let count_weeks = |since| now.signed_duration_since(get_timezone_start(since)).num_weeks();
    let pve_id = count_weeks(get_bonus_cycle_start()) % BONUS_EVENT_PVE_SIZE_CYCLE;
    let pvp_id = count_weeks(get_bonus_cycle_start()) % BONUS_EVENT_PVP_SIZE_CYCLE;
    let days_left = 7 - now.weekday().num_days_from_monday() - 1;
    let hour_left = if now.hour() > 18 {
        24 - now.hour() + 18
    } else {
        18 - now.hour()
    } - 1;
    let min_left = 60 - now.minute() - 1;
    let sec_left = 60 - now.second() - 1;
    let current_pve = bonus_pve.get_from_id(pve_id).unwrap();
    let current_pvp = bonus_pvp.get_from_id(pvp_id).unwrap();
    let mut response = MessageBuilder::new();
    response
        .push_underline_line(i18n_messages.bonus_headline())
        .push(format!("{} {} -- ", i18n_messages.bonus_pve(), &current_pve.name))
        .push_bold_line(&current_pve.description)
        .push(format!("{} {} -- ", i18n_messages.bonus_pvp(), &current_pvp.name))
        .push_bold_line(&current_pvp.description)
        .push(i18n_messages.bonus_expire())
        .push_bold_line(format!(" {} {}, {:0>2}:{:0>2}:{:0>2}!", days_left, i18n_messages.time_days(), hour_left, min_left, sec_left));

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    
    Ok(())
}