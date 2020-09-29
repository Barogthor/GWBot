use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::{
    builder::CreateMessage,
    model::prelude::{
        Message,
    },
    prelude::Context,

};
use serenity_utils::{
    menu::{Menu, MenuOptions},
};
use serenity_utils::menu::Control;
use std::sync::Arc;
use serenity::model::prelude::Reaction;
use serenity::model::channel::ReactionType::Unicode;
use crate::constants::{REACTION_ONE, REACTION_TWO};

#[command]
async fn menu(ctx: &Context, msg: &Message) -> CommandResult {

    let channel = msg.channel_id.to_channel(&ctx).await?.guild().unwrap();
    let emojis = ctx.http.get_guild(channel.guild_id.0).await?.emojis;
    println!("{:?}", emojis);
    let mut page_one = CreateMessage::default();
    page_one
        .content("Page number one!")
        .embed(|e| {
            e.description("The first page!");

            e
        });

    let mut page_two = CreateMessage::default();
    page_two
        .content("Page number two!")
        .embed(|e| {
            e.description("The second page!");

            e
        });

    let pages = [page_one, page_two];
    let controls = vec![
        Control::new(
            Unicode(REACTION_ONE.to_string()),
            Arc::new(|m, r| Box::pin(first_page(m, r))),
        ),
        Control::new(
            Unicode(REACTION_TWO.to_string()),
            Arc::new(|m, r| Box::pin(second_page(m, r))),
        )
    ];
    let options = MenuOptions {
        controls,
        ..Default::default()
    };
    // Creates a new menu.
    let menu = Menu::new(ctx, msg, &pages, options);

    // Runs the menu and returns optional `Message` used to display the menu.
    let _opt_message = menu.run().await?;

    Ok(())
}

async fn first_page(menu: &mut Menu<'_>, reaction: Reaction) {
    // Remove the reaction used to change the menu.
    let _ = &reaction.delete(&menu.ctx.http).await;

    // Set page number to `0`.
    menu.options.page = 0;
}

// A custom function to be used as a control function for the menu.
async fn second_page(menu: &mut Menu<'_>, reaction: Reaction) {
    // Remove the reaction used to change the menu.
    let _ = &reaction.delete(&menu.ctx.http).await;
    println!("{:?}", reaction.emoji);

    // Set page number to total - 1.
    menu.options.page = 1;
}