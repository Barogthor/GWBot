use chrono::{Date, TimeZone, Utc};
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

use crate::constants::{
    ZAISHEN_BOUNTY_SIZE_CYCLE,
    ZAISHEN_BOUNTY_START,
    ZAISHEN_COMBAT_SIZE_CYCLE,
    ZAISHEN_COMBAT_START,
    ZAISHEN_MISSION_SIZE_CYCLE,
    ZAISHEN_MISSION_START,
    ZAISHEN_VANQUISH_SIZE_CYCLE,
    ZAISHEN_VANQUISH_START,
};
use crate::get_bot_datas;
use crate::utils::{I18nMessageStore, ZaishenQuestStore};
use crate::utils::time::{get_next_day, get_time_left, get_utc_start};

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
async fn zqnext(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.channel_id.to_channel(&ctx).await?.guild()
        .and_then(|channel| Some(channel.guild_id.0)).unwrap_or(0);
    let datas_lock = get_bot_datas(ctx).await;
    let read_data = &datas_lock.read().await;
    let (lang, _) = read_data.guilds_config.get_guild_config(guild);
    let i18n_messages: &I18nMessageStore = &read_data.i18n_messages.lng(lang).unwrap();
    let zq_mission: &ZaishenQuestStore = &read_data.zaishen_mission.lng(lang).unwrap();
    let zq_bounty: &ZaishenQuestStore = &read_data.zaishen_bounty.lng(lang).unwrap();
    let zq_combat: &ZaishenQuestStore = &read_data.zaishen_combat.lng(lang).unwrap();
    let zq_vanquish: &ZaishenQuestStore = &read_data.zaishen_vanquish.lng(lang).unwrap();
    let now = Utc::now();
    let tomorrow = get_utc_start(get_next_day(now, 16));
    let count_days = |since| tomorrow.signed_duration_since(get_utc_start(since)).num_days();
    let mz_id = count_days(get_mz_cycle_start()) % ZAISHEN_MISSION_SIZE_CYCLE;
    let bz_id = count_days(get_bz_cycle_start()) % ZAISHEN_BOUNTY_SIZE_CYCLE;
    let cz_id = count_days(get_cz_cycle_start()) % ZAISHEN_COMBAT_SIZE_CYCLE;
    let vz_id = count_days(get_vz_cycle_start()) % ZAISHEN_VANQUISH_SIZE_CYCLE;
    let (_, hours_left, mins_left, secs_left) = get_time_left(tomorrow, now);

    let mission = zq_mission.get_from_id(mz_id).unwrap();
    let bounty = zq_bounty.get_from_id(bz_id).unwrap();
    let combat = zq_combat.get_from_id(cz_id).unwrap();
    let vanquish = zq_vanquish.get_from_id(vz_id).unwrap();
    let mut response = MessageBuilder::new();
    response
        .push_underline_line(i18n_messages.zaishen_quest_tomorrow_headline())
        .push(format!("{} ", i18n_messages.zaishen_quest_mission()))
        .push_bold_line(&mission.name)
        .push(format!("{} ", i18n_messages.zaishen_quest_bounty()))
        .push_bold_line(&bounty.name)
        .push(format!("{} ", i18n_messages.zaishen_quest_combat()))
        .push_bold_line(&combat.name)
        .push(format!("{} ", i18n_messages.zaishen_quest_vanquish()))
        .push_bold_line(&vanquish.name)
        .push(i18n_messages.zaishen_quest_reset())
        .push_bold_line(format!(" {:0>2}:{:0>2}:{:0>2}!", hours_left, mins_left, secs_left));

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    
    Ok(())
}