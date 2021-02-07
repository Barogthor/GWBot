use std::collections::HashMap;
use std::env;

use image::{DynamicImage, GenericImage, ImageBuffer, ImageResult};
use image::io::Reader as ImageReader;
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::http::AttachmentType;
use serenity::model::channel::{EmbedImage, Message};
use serenity::model::guild::Emoji;
use serenity::utils::MessageBuilder;
use tokio::fs::File;

use crate::get_bot_datas;
use crate::utils::{AttributeStore, ProfessionStore, SKillI18nStore};
use crate::utils::skill::SkillCodeParser;

#[command]
async fn skill(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let code_skill = args.single::<String>()?;
    let skill_record = SkillCodeParser::parse(code_skill.clone());

    let channel = msg.channel_id.to_channel(&ctx).await?.guild();
    let guild = channel.and_then(|channel| Some(channel.guild_id.0)).unwrap_or(0);
    println!("guild id: {}", guild);
    println!("env guild id: {}", env::var("HOME_GUILD").expect("missing env HOME_GUILD"));
    let datas_lock = get_bot_datas(ctx).await;
    let read_data = &datas_lock.read().await;
    let (lang, _) = read_data.guilds_config.get_guild_config(guild);
    // let i18n_messages: &I18nMessageStore = &read_data.i18n_messages.lng(lang).unwrap();
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
        .push_bold(&professions_store.from(skill_record.primary_profession).unwrap().0)
        .push(" / ")
        .push_bold(&professions_store.from(skill_record.secondary_profession).unwrap().0)
        .emoji(emoji_lookup.get(&skill_record.secondary_profession.to_string()).unwrap())
        .push("--")
        .push_mono(&code_skill)
        .push("--\n");
    let mut count = skill_record.attributes.len();
    for (attr, points) in skill_record.attributes {
        count -= 1;
        response.push(format!("{}: ", attributes_store.from(attr).unwrap().0)).push_bold(points);
        if count>0 {
            response.push(", ");
        }
    }
    response.push_line("");
    let empty_skill = skills_store.lang_and_id(lang, 0).unwrap();
    let build_image = get_embedded_build(&skill_record.skills);
    for i in 0..8 {
        let skill = skills_store.lang_and_id(lang, skill_record.skills[i]).unwrap_or_else(|| empty_skill);
        response.push(format!("Skill {}: ", i + 1)).push_bold_line(&skill.0.name);
    }


    let image_name = format!("{}.jpg", &code_skill);
    let path_build = format!("tmp/{}", &image_name);
    if let Ok(build_image) = &build_image {
        build_image.save(&path_build).unwrap();
    }

    let build_attachment = File::open(&path_build).await;
    let build_attachment = build_attachment.as_ref();
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(response);
        if build_image.is_ok() {
            let build_attachment = build_attachment.unwrap();
            let attachment = AttachmentType::File { file: build_attachment, filename: image_name.clone() };
            m.embed(|e| e.image(format!("attachment://{}", &image_name)));
            m.add_file(attachment);
        }
        m
    }).await {
        println!("Error sending message: {:?}", why);
    }

    // msg.reply(ctx, code_skill).await?;

    Ok(())
}

fn get_embedded_build(ids: &[u32; 8]) -> ImageResult<DynamicImage> {
    let mut build_image = DynamicImage::new_rgb8(64 * 8, 64);
    for (i, id) in ids.iter().enumerate() {
        let image = ImageReader::open(format!("cache/{}.jpg", id)).unwrap().decode()?;
        let mut skill_part = build_image.sub_image((i * 64) as u32, 0, 64, 64);
        skill_part.copy_from(&image, 0, 0)?;
    }

    Ok(build_image)
}