use serenity::framework::standard::macros::command;
use serenity::client::{Context};
use serenity::model::channel::Message;
use serenity::framework::standard::CommandResult;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}