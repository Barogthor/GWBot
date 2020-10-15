use chrono::{DateTime, Utc};
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

use crate::{get_bot_datas, I18nStore};
use crate::constants::DATETIME_FORMAT;
use crate::enums::Language;
use crate::utils::{SpecialEventPeriod, SpecialEventStore};
use crate::utils::time::DateTimeRange;

type EventTuple = (Vec<SpecialEventPeriod>, I18nStore<SpecialEventStore>);

#[command]
async fn event(ctx: &Context, msg: &Message) -> CommandResult {
    let datas_lock = get_bot_datas(ctx).await;
    let tuple: &EventTuple = &datas_lock.read().await.event;
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
    let localized_events = event_store.lng(Language::French).unwrap();
    let mut response = MessageBuilder::new();
    if running_events.is_empty() {
        response.push_line("There is no event currently running");
    } else {
        running_events.iter().for_each(|evt| print_running_event(&mut response, evt, localized_events, &now));

        if let Some(event_period) = next_event {
            print_next_event(&mut response, event_period, localized_events, &now);
        }
    }
    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

fn print_running_event(response: &mut MessageBuilder, event_period: &&&SpecialEventPeriod, localized_events: &SpecialEventStore, now: &DateTime<Utc>) {
    let event = localized_events.get_from_id(event_period.0).unwrap();
    let event_range: &DateTimeRange<_> = &event_period.1;
    response.push_bold(&event.name);
    response.push(" has begun! ");
    response.push_line(&event.note);
    response.push("Event ends in: ");
    let time_left = event_range.1.signed_duration_since(*now);
    let days_left = time_left.num_days();
    response.push_bold(days_left).push(" days, ");
    let hours_left = time_left.num_hours() % 24;
    let mins_left = time_left.num_minutes() % 60;
    let secs_left = time_left.num_seconds() % 60;
    response.push_bold(format!("{:0>2}:{:0>2}:{:0>2}!", hours_left, mins_left, secs_left));
    response.push(format!(" ({})\n", event_range.1.format(DATETIME_FORMAT)));
}

fn print_next_event(response: &mut MessageBuilder, event_period: &&&SpecialEventPeriod, localized_events: &SpecialEventStore, now: &DateTime<Utc>) {
    let event = localized_events.get_from_id(event_period.0).unwrap();
    let event_range: &DateTimeRange<_> = &event_period.1;
    response.push("The next event is ");
    response.push_bold(&event.name).push(", ");
    response.push_line(&event.note);
    response.push("Event begins in: ");
    let time_left = event_range.0.signed_duration_since(*now);
    let days_left = time_left.num_days();
    response.push_bold(days_left).push(" days, ");
    let hours_left = time_left.num_hours() % 24;
    let mins_left = time_left.num_minutes() % 60;
    let secs_left = time_left.num_seconds() % 60;
    response.push_bold(format!("{:0>2}:{:0>2}:{:0>2}!", hours_left, mins_left, secs_left));
    response.push(format!(" ({})\n", event_range.0.format(DATETIME_FORMAT)));
}