use serenity::client::Context;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;

#[command]
async fn utc(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}