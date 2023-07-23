use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use crate::get_mut_bot_datas;
use crate::utils::GuildsConfig;

#[command]
async fn utc(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let utc = args.single::<i32>()?;
    let data_lock = get_mut_bot_datas(ctx).await;
    let mut write_data = data_lock.write().await;
    let config: &mut GuildsConfig = &mut write_data.guilds_config;
    let channel = msg.channel_id.to_channel(&ctx).await?.guild().unwrap();
    config.set_utc(channel.guild_id.0, utc);
    let mut response = MessageBuilder::new();
    response.push(format!("Command on your server will now use UTC{} as reference", utc));
    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}