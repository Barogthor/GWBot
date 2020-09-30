use std::str::Chars;

use crate::constants::{
    SKILLS_EN,
    STANDARD_DECODE,
};
use crate::enums::{AttributeType, ProfessionType};

fn flip_binary_pad(binary: String) -> String {
    let mut bit_pos = 5;
    let mut num = 0u8;
    for c in binary.chars() {
        if c == '1' {
            num |= 1 << (5 - bit_pos);
        }
        bit_pos -= 1;
    }

    format!("{:0>6b}", num)
}

fn unflip_binary(binary: String) -> u32 {
    let len = (binary.len()) as u32;
    let mut bit_pos = len;
    let mut num = 0u32;
    for c in binary.chars() {
        // print!("{} {} || ", bit_pos, num);
        if c == '1' {
            num |= 1 << (len - bit_pos);
        }
        // println!("{} {}", bit_pos, num);
        bit_pos -= 1;
    }
    num
}

fn string_from_n_chars(iter: &mut Chars, n: usize) -> String {
    let mut buffer = String::default();
    for _i in 0..n {
        buffer.push(iter.next().unwrap());
    }
    buffer
}

fn read_template_header(binary: &mut Chars) {
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
    let attribute_chunk_size = unflip_binary(attribute_chunk_size) + 4;
    let mut attributes = vec![];
    for _i in 0..count_attributes {
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
    let mut skills = [(0u32, ""); 8];
    for i in 0..8 {
        let skill_id = string_from_n_chars(binary, skill_chunk_size as usize);
        let skill_id = unflip_binary(skill_id);
        let skill_name = SKILLS_EN.get(&skill_id).unwrap_or(&"None");
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
        SkillCodeRecord {
            primary_profession: primary,
            secondary_profession: secondary,
            attributes,
            skills,
        }
    }
}

