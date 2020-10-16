use chrono::{Date, Datelike, DateTime, Timelike, TimeZone, Utc};
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

use crate::constants::{EMOTE_MAP, EMOTE_POINT_RIGHT, NICHOLAS_TRAVELER_SIZE_CYCLE, NICHOLAS_TRAVELER_START};
use crate::enums::Language::French;
use crate::get_bot_datas;
use crate::utils::{I18nMessageStore, NicholasGiftStore};

fn get_timezone_start(date: Date<Utc>) -> DateTime<Utc> {
    date.and_hms(16, 0, 0)
}

fn get_cycle_start() -> Date<Utc> {
    let zq = NICHOLAS_TRAVELER_START;
    Utc.ymd(zq.0, zq.1, zq.2)
}

#[command]
async fn nick(ctx: &Context, msg: &Message) -> CommandResult {
    let datas_lock = get_bot_datas(ctx).await;
    let read_data = &datas_lock.read().await;
    let i18n_messages: &I18nMessageStore = &read_data.i18n_messages.lng(French).unwrap();
    let nicholas_gift: &NicholasGiftStore = &read_data.nicholas_traveler.lng(French).unwrap();
    let now = Utc::now();
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

    let gift = nicholas_gift.get_from_id(gift_id).unwrap();
    let mut response = MessageBuilder::new();
    response
        .push_underline_line(i18n_messages.nicholas_gift_headline())
        .push(format!("{} ", i18n_messages.nicholas_gift_collecting()))
        .push_bold(&gift.item)
        .push(format!(" {} ", i18n_messages.nicholas_gift_per()))
        .push_bold_line(&gift.location)
        .push(format!("{} ", i18n_messages.nicholas_gift_in()))
        .push_bold(&gift.region)
        .push_line(format!(" ({}).", &gift.campaign))
        .push(format!("{} ", i18n_messages.nicholas_gift_moving()))
        .push_bold_line(format!("{} {}, {:0>2}:{:0>2}:{:0>2}!", days_left, i18n_messages.time_days(), hour_left, min_left, sec_left))
        .push(format!("{} {} ", EMOTE_MAP, EMOTE_POINT_RIGHT))
        .push_spoiler_line(&gift.location_url);

    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(response);
        m
    }).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}