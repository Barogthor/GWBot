use chrono::{Date, TimeZone, Utc};
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

use crate::constants::{EMOTE_MAP, EMOTE_POINT_RIGHT, NICHOLAS_TRAVELER_SIZE_CYCLE, NICHOLAS_TRAVELER_START};
use crate::get_bot_datas;
use crate::utils::{I18nMessageStore, NicholasGiftData, NicholasGiftStore};
use crate::utils::time::{get_next_week, get_time_left, get_weekly_start};

fn get_cycle_start() -> Date<Utc> {
    let zq = NICHOLAS_TRAVELER_START;
    Utc.ymd(zq.0, zq.1, zq.2)
}

#[command]
async fn nick(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.channel_id.to_channel(&ctx).await?.guild()
        .and_then(|channel| Some(channel.guild_id.0)).unwrap_or(0);
    let datas_lock = get_bot_datas(ctx).await;
    let read_data = &datas_lock.read().await;
    let (lang, _) = read_data.guilds_config.get_guild_config(guild);
    let i18n_messages: &I18nMessageStore = &read_data.i18n_messages.lng(lang).unwrap();
    let nicholas_gift: &NicholasGiftStore = &read_data.nicholas_traveler.lng(lang).unwrap();
    let now = Utc::now();
    let next_week = get_weekly_start(get_next_week(now));
    let count_weeks = |since| now.signed_duration_since(get_weekly_start(since)).num_weeks();
    let gift_id = count_weeks(get_cycle_start()) % NICHOLAS_TRAVELER_SIZE_CYCLE;
    let (days_left, hours_left, mins_left, secs_left) = get_time_left(next_week, now);

    let gift = nicholas_gift.get_from_id(gift_id).unwrap();
    let response = build_response(i18n_messages, days_left, hours_left, mins_left, secs_left, gift);

    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(response);
        m
    }).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

fn build_response(i18n_messages: &I18nMessageStore, days_left: i64, hours_left: i64, mins_left: i64, secs_left: i64, gift: &NicholasGiftData) -> MessageBuilder {
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
        .push_bold_line(format!("{} {}, {:0>2}:{:0>2}:{:0>2}!", days_left, i18n_messages.time_days(), hours_left, mins_left, secs_left))
        .push(format!("{} {} ", EMOTE_MAP, EMOTE_POINT_RIGHT))
        .push_spoiler_line(&gift.location_url);
    response
}
