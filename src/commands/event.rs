use chrono::{DateTime, Utc};
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

use crate::{get_bot_datas, I18nStore};
use crate::constants::DATETIME_FORMAT;
use crate::utils::{I18nMessageStore, SpecialEventPeriod, SpecialEventStore};
use crate::utils::time::{DateTimeRange, get_time_left};

type EventTuple = (Vec<SpecialEventPeriod>, I18nStore<SpecialEventStore>);

#[command]
async fn event(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.channel_id.to_channel(&ctx).await?.guild()
        .and_then(|channel| Some(channel.guild_id.0)).unwrap_or(0);
    let datas_lock = get_bot_datas(ctx).await;
    let read_data = &datas_lock.read().await;
    let (lang, _) = read_data.guilds_config.get_guild_config(guild);
    let i18n_messages: &I18nStore<I18nMessageStore> = &read_data.i18n_messages;
    let tuple: &EventTuple = &read_data.event;
    let (periods, event_store) = tuple;
    let now = Utc::now();
    let events_left: Vec<_> = periods.iter()
        .filter(|it| it.within(&now) || it.after(&now))
        .collect();
    let running_events: Vec<_> = events_left.iter()
        .filter(|it| it.within(&now))
        .collect();
    let next_events: Vec<_> = events_left.iter()
        .filter(|it| it.after(&now))
        .collect();
    let next_event = next_events.first();
    println!("{:?} {:?}", events_left, running_events);
    let localized_events = event_store.lng(lang).unwrap();
    let localized_messages = i18n_messages.lng(lang).unwrap();
    let mut response = MessageBuilder::new();
    if running_events.is_empty() {
        response.push_line(localized_messages.event_no_running());
    } else {
        running_events.iter().for_each(|evt| print_running_event(&mut response, *evt, localized_events, localized_messages, &now));

        if let Some(event_period) = next_event {
            print_next_event(&mut response, event_period, localized_events, localized_messages, &now);
        }
    }
    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

fn print_running_event(response: &mut MessageBuilder, event_period: &SpecialEventPeriod, localized_events: &SpecialEventStore, localized_messages: &I18nMessageStore, now: &DateTime<Utc>) {
    let event = localized_events.get_from_id(event_period.0).unwrap();
    let event_range: &DateTimeRange<_> = &event_period.1;
    response.push_bold(&event.name);
    response.push_line(format!(" {} {}", localized_messages.event_started(), &event.note));
    response.push(localized_messages.event_end());
    let (days_left, hours_left, mins_left, secs_left) = get_time_left(event_range.1, *now);
    response.push_bold(format!(" {} {}, {:0>2}:{:0>2}:{:0>2}!", days_left, localized_messages.time_days(), hours_left, mins_left, secs_left));
    response.push_line(format!(" ({})\n", event_range.1.format(DATETIME_FORMAT)));
}

fn print_next_event(response: &mut MessageBuilder, event_period: &SpecialEventPeriod, localized_events: &SpecialEventStore, localized_messages: &I18nMessageStore, now: &DateTime<Utc>) {
    let event = localized_events.get_from_id(event_period.0).unwrap();
    let event_range: &DateTimeRange<_> = &event_period.1;
    response.push(localized_messages.event_next());
    response.push_bold(format!(" {}, ", &event.name));
    response.push_line(&event.note);
    response.push(localized_messages.event_begin());
    let (days_left, hours_left, mins_left, secs_left) = get_time_left(event_range.0, *now);
    response.push_bold(format!(" {} {}, {:0>2}:{:0>2}:{:0>2}!", days_left, localized_messages.time_days(), hours_left, mins_left, secs_left));
    response.push_line(format!(" ({})\n", event_range.0.format(DATETIME_FORMAT)));
}