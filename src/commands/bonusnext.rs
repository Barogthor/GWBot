use chrono::{Date, TimeZone, Utc};
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
use crate::get_bot_datas;
use crate::utils::{BonusEventStore, I18nMessageStore};
use crate::utils::time::{get_next_week, get_time_left, get_weekly_start};

fn get_bonus_cycle_start() -> Date<Utc> {
    let zq = BONUS_EVENT_START;
    Utc.ymd(zq.0, zq.1, zq.2)
}

#[command]
async fn bonusnext(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.channel_id.to_channel(&ctx).await?.guild()
        .and_then(|channel| Some(channel.guild_id.0)).unwrap_or(0);
    let datas_lock = get_bot_datas(ctx).await;
    let read_data = &datas_lock.read().await;
    let (lang, _) = read_data.guilds_config.get_guild_config(guild);
    let i18n_messages: &I18nMessageStore = &read_data.i18n_messages.lng(lang).unwrap();
    let bonus_pve: &BonusEventStore = &read_data.bonus_pve.lng(lang).unwrap();
    let bonus_pvp: &BonusEventStore = &read_data.bonus_pvp.lng(lang).unwrap();
    let now = Utc::now();
    let next_week = get_weekly_start(get_next_week(now));
    let count_weeks = |since| next_week.signed_duration_since(get_weekly_start(since)).num_weeks();
    let pve_id = count_weeks(get_bonus_cycle_start()) % BONUS_EVENT_PVE_SIZE_CYCLE;
    let pvp_id = count_weeks(get_bonus_cycle_start()) % BONUS_EVENT_PVP_SIZE_CYCLE;
    let (days_left, hours_left, mins_left, secs_left) = get_time_left(next_week, now);
    let current_pve = bonus_pve.get_from_id(pve_id).unwrap();
    let current_pvp = bonus_pvp.get_from_id(pvp_id).unwrap();
    let mut response = MessageBuilder::new();
    response
        .push_underline_line(i18n_messages.bonus_next_headline())
        .push(format!("{} {} -- ", i18n_messages.bonus_pve(), &current_pve.name))
        .push_bold_line(&current_pve.description)
        .push(format!("{} {} -- ", i18n_messages.bonus_pvp(), &current_pvp.name))
        .push_bold_line(&current_pvp.description)
        .push(i18n_messages.bonus_next_start())
        .push_bold_line(format!(" {} {}, {:0>2}:{:0>2}:{:0>2}!", days_left, i18n_messages.time_days(), hours_left, mins_left, secs_left));

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    
    Ok(())
}