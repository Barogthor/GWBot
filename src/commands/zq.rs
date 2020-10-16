use chrono::{Date, DateTime, Timelike, TimeZone, Utc};
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
use crate::enums::Language::French;
use crate::get_bot_datas;
use crate::utils::{I18nMessageStore, ZaishenQuestStore};

fn get_timezone_start(date: Date<Utc>) -> DateTime<Utc> {
    date.and_hms(16, 0, 0)
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
    let datas_lock = get_bot_datas(ctx).await;
    let read_data = &datas_lock.read().await;
    let i18n_messages: &I18nMessageStore = &read_data.i18n_messages.lng(French).unwrap();
    let zq_mission: &ZaishenQuestStore = &read_data.zaishen_mission.lng(French).unwrap();
    let zq_bounty: &ZaishenQuestStore = &read_data.zaishen_bounty.lng(French).unwrap();
    let zq_combat: &ZaishenQuestStore = &read_data.zaishen_combat.lng(French).unwrap();
    let zq_vanquish: &ZaishenQuestStore = &read_data.zaishen_vanquish.lng(French).unwrap();
    let now = Utc::now();
    let count_days = |since| now.signed_duration_since(get_timezone_start(since)).num_days();
    let mz_id = count_days(get_mz_cycle_start()) % ZAISHEN_MISSION_SIZE_CYCLE;
    let bz_id = count_days(get_bz_cycle_start()) % ZAISHEN_BOUNTY_SIZE_CYCLE;
    let cz_id = count_days(get_cz_cycle_start()) % ZAISHEN_COMBAT_SIZE_CYCLE;
    let vz_id = count_days(get_vz_cycle_start()) % ZAISHEN_VANQUISH_SIZE_CYCLE;
    let hour_left = if now.hour() > 18 {
        24 - now.hour() + 18
    } else {
        18 - now.hour()
    } - 1;
    let min_left = 60 - now.minute() - 1;
    let sec_left = 60 - now.second() - 1;
    let mission = zq_mission.get_from_id(mz_id).unwrap();
    let bounty = zq_bounty.get_from_id(bz_id).unwrap();
    let combat = zq_combat.get_from_id(cz_id).unwrap();
    let vanquish = zq_vanquish.get_from_id(vz_id).unwrap();
    let mut response = MessageBuilder::new();
    response
        .push_underline_line(i18n_messages.zaishen_quest_headline())
        .push(format!("{} ", i18n_messages.zaishen_quest_mission()))
        .push_bold_line(&mission.name)
        .push(format!("{} ", i18n_messages.zaishen_quest_bounty()))
        .push_bold_line(&bounty.name)
        .push(format!("{} ", i18n_messages.zaishen_quest_combat()))
        .push_bold_line(&combat.name)
        .push(format!("{} ", i18n_messages.zaishen_quest_vanquish()))
        .push_bold_line(&vanquish.name)
        .push(i18n_messages.zaishen_quest_reset())
        .push_bold_line(format!(" {:0>2}:{:0>2}:{:0>2}!", hour_left, min_left, sec_left));

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    
    Ok(())
}