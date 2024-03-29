use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use crate::enums::Language;
use crate::get_mut_bot_datas;
use crate::utils::GuildsConfig;

#[command]
async fn lang(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let language = args.single::<String>()?;
    let data_lock = get_mut_bot_datas(ctx).await;
    let mut write_data = data_lock.write().await;
    let config: &mut GuildsConfig = &mut write_data.guilds_config;
    let channel = msg.channel_id.to_channel(&ctx).await?.guild();

    if let Some(channel) = channel {
        let mut response = MessageBuilder::new();
        Language::from(&language)
            .and_then(|lng| {
                config.set_language(channel.guild_id.0, lng);
                Ok(lng)
            })
            .and_then(|lng| {
                response.push(format!("From then on, I shall speak {:?} on this server !", lng));
                Ok(lng)
            })
            .or_else(|err| {
                response.push(err);
                Err(())
            });
        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            println!("Error sending message: {:?}", why);
        }
    } else {
        msg.channel_id.say(&ctx.http, "Can't configure outside a server").await.ok();
    }
    Ok(())
}