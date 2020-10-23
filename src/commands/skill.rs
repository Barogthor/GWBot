use std::collections::HashMap;
use std::env;

use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::channel::Message;
use serenity::model::guild::Emoji;
use serenity::utils::MessageBuilder;

use crate::get_bot_datas;
use crate::utils::{AttributeStore, I18nMessageStore, ProfessionStore, SKillI18nStore};
use crate::utils::skill::SkillCodeParser;

#[command]
async fn skill(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let code_skill = args.single::<String>()?;
    let skill_record = SkillCodeParser::parse(code_skill.clone());

    let channel = msg.channel_id.to_channel(&ctx).await?.guild();
    let guild = channel.and_then(|channel| Some(channel.guild_id.0)).unwrap_or(0);
    let datas_lock = get_bot_datas(ctx).await;
    let read_data = &datas_lock.read().await;
    let (lang, _) = read_data.guilds_config.get_guild_config(guild);
    let i18n_messages: &I18nMessageStore = &read_data.i18n_messages.lng(lang).unwrap();
    let skills_store: &SKillI18nStore = &read_data.skills;
    let attributes_store: &AttributeStore = &read_data.attributes.lng(lang).unwrap();
    let professions_store: &ProfessionStore = &read_data.professions.lng(lang).unwrap();

    let emojis = ctx.http.get_guild(guild).await?.emojis;
    let emoji_lookup = emojis.iter()
        .map(|(_id, emoji)| (emoji.name.clone(), emoji.clone()))
        .collect::<HashMap<String, Emoji>>();

    let mut response = MessageBuilder::new();
    response
        .emoji(emoji_lookup.get(&skill_record.primary_profession.to_string()).unwrap())
        .push_bold(&professions_store.from(&skill_record.primary_profession).unwrap().0)
        .push(" / ")
        .push_bold(&professions_store.from(&skill_record.secondary_profession).unwrap().0)
        .emoji(emoji_lookup.get(&skill_record.secondary_profession.to_string()).unwrap())
        .push("--")
        .push_mono(code_skill)
        .push("--\n");
    let mut count = skill_record.attributes.len();
    for (attr, points) in skill_record.attributes {
        count -= 1;
        response.push(format!("{}: ", attributes_store.from(&attr).unwrap().0)).push_bold(points);
        if count>0 {
            response.push(", ");
        }
    }
    response.push_line("");
    let empty_skill = skills_store.lang_and_id(lang, 0).unwrap();
    for i in 0..8 {
        let skill = skills_store.lang_and_id(lang, skill_record.skills[i]).unwrap_or_else(|| empty_skill.clone());
        response.push(format!("Skill {}: ", i + 1)).push_bold_line(&skill.0.name);
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        println!("Error sending message: {:?}", why);
    }
    // msg.reply(ctx, code_skill).await?;

    Ok(())
}