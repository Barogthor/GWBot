use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

use crate::get_mut_bot_datas;
use crate::enums::Language;
use crate::utils::GuildsConfig;

#[command]
async fn lang(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let language = args.single::<String>()?;
    let data_lock = get_mut_bot_datas(ctx).await;
    let mut write_data = data_lock.write().await;
    let config: &mut GuildsConfig = &mut write_data.guilds_config;
    let channel = msg.channel_id.to_channel(&ctx).await?.guild().unwrap();

    let mut response = MessageBuilder::new();
    Language::from(&language)
        .and_then(|lng| {
            config.set_language(channel.guild_id.0, lng);
            Ok(lng)
        })
        .and_then(|lng| {
            response.push(format!("GWBot will now speak {:?} on this server !", lng));
            Ok(lng)
        })
        .or_else(|err| {
            response.push(err);
            Err(())
        });
    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}