#[macro_use]
extern crate lazy_static;

extern crate dotenv;
pub mod constants;
pub mod enums;
use std::env;
use dotenv::dotenv;
mod commands;
pub mod utils;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    StandardFramework,
    macros::{
        group
    }
};
use commands::{
    ping::*,
    skill::*,
    menu::*,
    zq::*
};
use serenity::model::prelude::Ready;
use crate::constants::{
    STANDARD_DECODE,
    SKILLS_EN
};
use std::str::{Chars};
use crate::enums::{ProfessionType, AttributeType};
use std::process::exit;
use chrono::Date;


#[group]
#[commands(ping, skill, menu, zq)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{}#{} is connected!", ready.user.name, ready.user.discriminator);
    }
}

use chrono::prelude::*;
use chrono::offset::LocalResult;
#[tokio::main]
async fn main() {
    let augury_rock = Utc.ymd(2011, 3, 3);
    let drake_kabob = Utc.ymd(2020, 9, 7);
    let next_drake_kabob = Utc.ymd(2023, 4, 24);
    let augury_rock_2020 = Utc.ymd(2020, 3, 27);
    let raisu_2020 = Utc.ymd(2020, 3, 29);
    let frost_gate = Utc.ymd(2011, 5, 10);
    let diff = frost_gate.signed_duration_since(augury_rock);
    let diff2 = augury_rock_2020.signed_duration_since(augury_rock);
    let diff3 = raisu_2020.signed_duration_since(augury_rock);
    let diff4 = next_drake_kabob.signed_duration_since(drake_kabob);
    println!("{}", diff.num_days());
    println!("{}", diff2.num_days()%69);
    println!("{}", diff3.num_days()%69);
    println!("{}", diff4.num_weeks());
    // exit(0);
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-")) // set the bot's commands prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    // println!("{}", token);
    let mut client = Client::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn flip_binary_pad(binary: String) -> String{
    let mut bit_pos = 5;
    let mut num = 0u8;
    for c in binary.chars() {
        if c == '1' {
            num |= 1 << (5 - bit_pos);
        }
        bit_pos-=1;
    }

    format!("{:0>6b}", num)
}

fn unflip_binary(binary: String) -> u32{
    let len = (binary.len()) as u32;
    let mut bit_pos = len;
    let mut num = 0u32;
    for c in binary.chars() {
        // print!("{} {} || ", bit_pos, num);
        if c == '1' {
            num |= 1 << (len - bit_pos);
        }
        // println!("{} {}", bit_pos, num);
        bit_pos-=1;
    }
    num
}

fn string_from_n_chars(iter: &mut Chars, n: usize) -> String{
    let mut buffer = String::default();
    for _i in 0..n{
        buffer.push(iter.next().unwrap());
    }
    buffer
}

fn read_template_header(binary: &mut Chars){
    let _template_type = string_from_n_chars(binary, 4);
    // let template_type = unflip_binary(template_type);
    let _version = string_from_n_chars(binary, 4);
    // let version = unflip_binary(version);
    // println!("template type: {}, version: {}", template_type, version);
}

fn read_professions(binary: &mut Chars) -> (ProfessionType, ProfessionType) {
    let profession_chunk_size = string_from_n_chars(binary, 2);
    let profession_chunk_size = unflip_binary(profession_chunk_size) * 2 + 4;
    let first_profession = string_from_n_chars(binary, profession_chunk_size as usize);
    let second_profession = string_from_n_chars(binary, profession_chunk_size as usize);
    let first_profession = unflip_binary(first_profession);
    let second_profession = unflip_binary(second_profession);
    println!("first prof: {}, second prof: {}", first_profession, second_profession);
    (ProfessionType::from(first_profession), ProfessionType::from(second_profession))
}

fn read_attributes(binary: &mut Chars) -> Vec<(AttributeType, u32)> {
    let count_attributes = string_from_n_chars(binary, 4);
    let count_attributes = unflip_binary(count_attributes);
    let attribute_chunk_size = string_from_n_chars(binary, 4);
    let attribute_chunk_size = unflip_binary(attribute_chunk_size)+4;
    let mut attributes = vec![];
    for _i in 0..count_attributes{
        let attribute_id = string_from_n_chars(binary, attribute_chunk_size as usize);
        let attribute_id = unflip_binary(attribute_id);
        let attribute_points = string_from_n_chars(binary, attribute_chunk_size as usize);
        let attribute_points = unflip_binary(attribute_points);
        println!("attribute: {} - {} pts", attribute_id, attribute_points);
        attributes.push((AttributeType::from(attribute_id), attribute_points));
    }
    attributes
}

fn read_skills<'a>(binary: &mut Chars) -> [(u32, &'a str); 8] {
    let skill_chunk_size = string_from_n_chars(binary, 4);
    let skill_chunk_size = unflip_binary(skill_chunk_size) + 8;
    let mut skills = [(0u32,"");8];
    for i in 0..8 {
        let skill_id = string_from_n_chars(binary, skill_chunk_size as usize);
        let skill_id = unflip_binary(skill_id);
        let skill_name = SKILLS_EN.get(&skill_id).unwrap();
        skills[i] = (skill_id, skill_name);
    }
    skills
}

#[derive(Debug)]
pub struct SkillCodeRecord<'a> {
    pub primary_profession: ProfessionType,
    pub secondary_profession: ProfessionType,
    pub attributes: Vec<(AttributeType, u32)>,
    pub skills: [(u32, &'a str); 8],
}
pub struct SkillCodeParser;
impl SkillCodeParser {
    pub fn parse<'a>(skill_code: String) -> SkillCodeRecord<'a> {
        let binary_code = skill_code
            .bytes()
            .map(|x| STANDARD_DECODE[x as usize])
            .map(|x| format!("{:0>6b}", x))
            .map(flip_binary_pad)
            .collect::<Vec<String>>();
        let binary_code: String = binary_code.join("");
        let mut binary_code = binary_code.chars();
        read_template_header(&mut binary_code);
        let (primary, secondary) = read_professions(&mut binary_code);
        let attributes = read_attributes(&mut binary_code);
        let skills = read_skills(&mut binary_code);
        SkillCodeRecord{
            primary_profession: primary,
            secondary_profession: secondary,
            attributes,
            skills,
        }
    }
}

// fn main() {
//     let code = "OQBDApwTOhwcgM4mmBaCeAUA".to_string();
//     let skill_record = SkillCodeParser::parse(code);
//     println!("{:?}", skill_record);
//     // Ok(())
// }
// https://wiki.guildwars.com/wiki/Skill_template_format#See_also
// https://wiki.guildwars.com/wiki/Talk:Skill_template_format
// https://wiki.guildwars.com/wiki/Template:Cycle
// https://wiki.guildwars.com/wiki/Zaishen_Mission/cycles
// https://wiki.guildwars.com/wiki/Zaishen_Bounty/cycles
// https://wiki.guildwars.com/wiki/Zaishen_Combat/cycles
// https://wiki.guildwars.com/wiki/Zaishen_Vanquish/cycles