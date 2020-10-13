use chrono::Utc;
use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

use crate::constants::{DATETIME_FORMAT, SPECIAL_EVENT_PERIODS, SPECIAL_EVENTS};
use crate::enums::Language;
use crate::utils::time::{DateTimeRange, DateTimeRangeComparison};

#[command]
async fn event(ctx: &Context, msg: &Message) -> CommandResult {
    let now = Utc::now();
    let events_left: Vec<_> = SPECIAL_EVENT_PERIODS.iter()
        .filter(|it| it.1.compare(&now).ge(&DateTimeRangeComparison::Within))
        .collect();
    let running_events: Vec<_> = events_left.iter()
        .filter(|it| it.1.compare(&now).eq(&DateTimeRangeComparison::Within))
        .collect();
    let next_events: Vec<_> = events_left.iter()
        .filter(|it| it.1.compare(&now).eq(&DateTimeRangeComparison::After))
        .collect();
    let next_event = next_events.first();
    println!("{:?} {:?}", events_left, running_events);
    let localized_events = SPECIAL_EVENTS.get(&Language::French).unwrap();
    let mut response = MessageBuilder::new();
    if running_events.is_empty() {
        response.push_line("There is no event currently running");
    } else {
        for event_period in running_events {
            let event = localized_events.get_from_id(event_period.0).unwrap();
            let event_range: &DateTimeRange<_> = &event_period.1;
            response.push_bold(&event.name);
            response.push(" has begun! ");
            response.push_line(&event.note);
            response.push("Event ends in: ");
            let time_left = event_range.1.signed_duration_since(now);
            let days_left = time_left.num_days();
            response.push_bold(days_left).push(" days, ");
            let hours_left = time_left.num_hours() % 24;
            let mins_left = time_left.num_minutes() % 60;
            let secs_left = time_left.num_seconds() % 60;
            response.push_bold(format!("{:0>2}:{:0>2}:{:0>2}!", hours_left, mins_left, secs_left));
            response.push(" (").push(event_range.1.format(DATETIME_FORMAT)).push_line(")");
        }
        if let Some(event_period) = next_event {
            let event = localized_events.get_from_id(event_period.0).unwrap();
            let event_range: &DateTimeRange<_> = &event_period.1;
            println!("{:?}", now);
            response.push("The next event is ");
            response.push_bold(&event.name).push(", ");
            response.push_line(&event.note);
            response.push("Event begins in: ");
            let time_left = event_range.0.signed_duration_since(now);
            let days_left = time_left.num_days();
            response.push_bold(days_left).push(" days, ");
            let hours_left = time_left.num_hours() % 24;
            let mins_left = time_left.num_minutes() % 60;
            let secs_left = time_left.num_seconds() % 60;
            response.push_bold(format!("{:0>2}:{:0>2}:{:0>2}!", hours_left, mins_left, secs_left));
            response.push(" (").push(event_range.0.format(DATETIME_FORMAT)).push_line(")");
        }
    }
    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}