use std::str::Chars;

use crate::constants::STANDARD_DECODE;
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
        if c == '1' {
            num |= 1 << (len - bit_pos);
        }
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
        let attribute_points = string_from_n_chars(binary, 4);
        let attribute_points = unflip_binary(attribute_points);
        attributes.push((AttributeType::from(attribute_id), attribute_points));
    }
    attributes
}

fn read_skills(binary: &mut Chars) -> [u32; 8] {
    let skill_chunk_size = string_from_n_chars(binary, 4);
    let skill_chunk_size = unflip_binary(skill_chunk_size) + 8;
    let mut skills = [0u32; 8];
    for skill in skills.iter_mut() {
        let skill_id = string_from_n_chars(binary, skill_chunk_size as usize);
        let skill_id = unflip_binary(skill_id);
        *skill = skill_id;
    }
    skills
}

#[derive(Debug, PartialEq)]
pub struct SkillCodeRecord {
    pub primary_profession: ProfessionType,
    pub secondary_profession: ProfessionType,
    pub attributes: Vec<(AttributeType, u32)>,
    pub skills: [u32; 8],
}

fn decode(code: String) -> String {
    code
        .bytes()
        .map(|x| STANDARD_DECODE[x as usize])
        .map(|x| format!("{:0>6b}", x))
        .map(flip_binary_pad)
        .collect::<Vec<String>>().join("")
}

pub struct SkillCodeParser;

impl SkillCodeParser {
    pub fn parse(skill_code: String) -> SkillCodeRecord {
        let binary_code = decode(skill_code);
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

// https://wiki.guildwars.com/wiki/Widget:Build_template_decoder
#[cfg(test)]
mod test {
    use std::fs;
    use std::fs::File;
    use std::io::Read;

    use image::{DynamicImage, GenericImage};
    use image::io::Reader as ImageReader;

    use crate::BotData;
    use crate::enums::{AttributeType, Language, ProfessionType};
    use crate::enums::AttributeType::*;
    use crate::enums::ProfessionType::*;
    use crate::utils::skill::{SkillCodeParser, SkillCodeRecord};
    use crate::utils::SKillI18nStore;

    #[derive(Debug, PartialEq)]
    pub struct SkillCodeRecordTranslated {
        pub primary_profession: ProfessionType,
        pub secondary_profession: ProfessionType,
        pub attributes: Vec<(AttributeType, u32)>,
        pub skills: Vec<String>,
    }

    fn record_to_translated(record: SkillCodeRecord, skills_store: &SKillI18nStore) -> SkillCodeRecordTranslated {
        let mut skills_name = vec![];
        record.skills.iter().for_each(|id| {
            let skill = skills_store.lang_and_id(Language::English, *id).unwrap();
            skills_name.push(skill.0.name.clone());
        });
        SkillCodeRecordTranslated {
            primary_profession: record.primary_profession,
            secondary_profession: record.secondary_profession,
            attributes: record.attributes,
            skills: skills_name,
        }
    }


    #[test]
    pub fn full_skill_set() {
        let datas = BotData::init();
        let code_skill = "OgdCoMzjyAYg7OiDDeBuQAA".to_string();
        let skills = vec!["Glyph of Sacrifice".to_string(), "Meteor Shower".to_string(), "Death's Charge".to_string(), "Star Burst".to_string(), "Lava Font".to_string(), "Flame Burst".to_string(), "Fire Attunement".to_string(), "Resurrection Signet".to_string()];
        let expected = SkillCodeRecordTranslated { primary_profession: Elementalist, secondary_profession: Assassin, attributes: vec![(FireMagic, 12), (EnergyStorage, 12)], skills };
        let actual = SkillCodeParser::parse(code_skill);
        let actual = record_to_translated(actual, &datas.skills);
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn skill_holed_set() {
        let datas = BotData::init();
        let code_skill = "OgdR8ZaCC3xmkUMCCAAAIVE".to_string();
        let skills = vec!["Dash".to_string(), "Death's Charge".to_string(), "\"You Move Like a Dwarf!\"".to_string(), "Light of Deldrimor".to_string(), "Unseen Fury".to_string(), "No Skill".to_string(), "No Skill".to_string(), "\"By Ural's Hammer!\"".to_string()];
        let expected = SkillCodeRecordTranslated {
            primary_profession: Elementalist,
            secondary_profession: Assassin,
            attributes: vec![(ShadowArts, 12)],
            skills,
        };
        let actual = SkillCodeParser::parse(code_skill);
        let actual = record_to_translated(actual, &datas.skills);
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn skill_seed_mindbender_lightdeldrimor() {
        let datas = BotData::init();
        let code_skill = "OwYT4yXCZCgYtcZIHMlAAgUMeAA".to_string();
        let skills = vec!["Blessed Aura".to_string(), "Mindbender".to_string(), "Glyph of Renewal".to_string(), "Seed of Life".to_string(), "Blessed Signet".to_string(), "No Skill".to_string(), "Light of Deldrimor".to_string(), "Life Bond".to_string()];
        let expected = SkillCodeRecordTranslated {
            primary_profession: Monk,
            secondary_profession: Elementalist,
            attributes: vec![(SmitingPrayers, 9), (ProtectionPrayers, 9), (DivineFavor, 12)],
            skills,
        };
        let actual = SkillCodeParser::parse(code_skill);
        let actual = record_to_translated(actual, &datas.skills);
        assert_eq!(expected, actual);
    }
    //rajouter test skill pve lulu/kuku et co et inconnu

    #[test]
    pub fn test_image_build() {
        let ids = [1043, 952, 2358, 2212, 1041, 0, 0, 2217];
        let mut build_image = DynamicImage::new_rgb8(64 * 8, 64);
        for (i, id) in ids.iter().enumerate() {
            let image = ImageReader::open(format!("cache/{}.jpg", id)).unwrap().decode().unwrap();
            let mut skill_part = build_image.sub_image((i * 64) as u32, 0, 64, 64);
            skill_part.copy_from(&image, 0, 0).ok();
        }
        build_image.save("test.jpg").ok();
    }

    #[test]
    pub fn test_dataset_skill() {
        let dataset_paths = fs::read_dir("./dataset_skill_test").expect("Dataset test directory missing or unvalid path.");
        for path in dataset_paths {
            let mut file_skill = File::open(path.unwrap().path()).unwrap();
            let mut skill = String::new();
            file_skill.read_to_string(&mut skill).expect("");
            println!("Skill code : {:?}", skill);
            SkillCodeParser::parse(skill);
        }
    }
}
