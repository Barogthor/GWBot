const html_parser = require('node-html-parser');
const fs = require('fs').promises;
const parse = require('csv-parse');
const parse_sync = require('csv-parse/lib/sync');
const http = require('https');

const professionMap = {}
professionMap["None"] = 0
professionMap["Warrior"] = 1
professionMap["Ranger"] = 2
professionMap["Monk"] = 3
professionMap["Necromancer"] = 4
professionMap["Mesmer"] = 5
professionMap["Elementalist"] = 6
professionMap["Assassin"] = 7
professionMap["Ritualist"] = 8
professionMap["Paragon"] = 9
professionMap["Dervish"] = 10
const attributeMap = {}
attributeMap["Fast Casting"] = 0
attributeMap["Illusion Magic"] = 1
attributeMap["Domination Magic"] = 2
attributeMap["Inspiration Magic"] = 3
attributeMap["Blood Magic"] = 4
attributeMap["Death Magic"] = 5
attributeMap["Soul Reaping"] = 6
attributeMap["Curses"] = 7
attributeMap["Air Magic"] = 8
attributeMap["Earth Magic"] = 9
attributeMap["Fire Magic"] = 10
attributeMap["Water Magic"] = 11
attributeMap["Energy Storage"] = 12
attributeMap["Healing Prayers"] = 13
attributeMap["Smiting Prayers"] = 14
attributeMap["Protection Prayers"] = 15
attributeMap["Divine Favor"] = 16
attributeMap["Strength"] = 17
attributeMap["Axe Mastery"] = 18
attributeMap["Hammer Mastery"] = 19
attributeMap["Swordsmanship"] = 20
attributeMap["Tactics"] = 21
attributeMap["Beast Mastery"] = 22
attributeMap["Expertise"] = 23
attributeMap["Wilderness Survival"] = 24
attributeMap["Marksmanship"] = 25
attributeMap["Dagger Mastery"] = 29
attributeMap["Deadly Arts"] = 30
attributeMap["Shadow Arts"] = 31
attributeMap["Communing"] = 32
attributeMap["Restoration Magic"] = 33
attributeMap["Channeling Magic"] = 34
attributeMap["Critical Strikes"] = 35
attributeMap["Spawning Power"] = 36
attributeMap["Spear Mastery"] = 37
attributeMap["Command"] = 38
attributeMap["Motivation"] = 39
attributeMap["Leadership"] = 40
attributeMap["Scythe Mastery"] = 41
attributeMap["Wind Prayers"] = 42
attributeMap["Earth Prayers"] = 43
attributeMap["Mysticism"] = 44

const typeMap = {}
typeMap["Signet"] = 0
typeMap["Stance"] = 1
typeMap["Hex spell"] = 2
typeMap["Snow fighting skill"] = 3
typeMap["Shout"] = 4
typeMap["Skill"] = 5
typeMap["Dual attack"] = 6
typeMap["Duplicate skill"] = 7
typeMap["Spell"] = 8
typeMap["Off-hand attack"] = 9
typeMap["Lead attack"] = 10
typeMap["Enchantment spell"] = 11
typeMap["Elite enchantment spell"] = 12
typeMap["Elite hex spell"] = 13
typeMap["Elite stance"] = 14
typeMap["Elite touch skill"] = 15
typeMap["Elite spell"] = 16
typeMap["Touch skill"] = 17
typeMap["Ward spell"] = 18
typeMap["Glyph"] = 19
typeMap["Resurrection skill"] = 20
typeMap["Melee attack"] = 21
typeMap["Sword attack"] = 22
typeMap["Axe attack"] = 23
typeMap["Hammer attack"] = 24
typeMap["Celestial skill"] = 25
typeMap["Skill (PvE-only)"] = 26
typeMap["Elite skill"] = 27
typeMap["Bow attack"] = 28
typeMap["Elite bow attack"] = 29
typeMap["Elite preparation"] = 30
typeMap["Elite signet"] = 31
typeMap["Pet attack"] = 32
typeMap["Elite pet attack"] = 33
typeMap["Trap"] = 34
typeMap["Elite nature ritual"] = 35
typeMap["Nature ritual"] = 36
typeMap["Item spell"] = 37
typeMap["Elite item spell"] = 38
typeMap["Binding ritual"] = 39
typeMap["Elite binding ritual"] = 40
typeMap["Elite weapon spell"] = 41
typeMap["Weapon spell"] = 42
typeMap["Well spell"] = 43
typeMap["Elite shout"] = 44
typeMap["Elite sword attack"] = 45
typeMap["Preparation"] = 46
typeMap["Scythe attack"] = 47
typeMap["Flash enchantment spell"] = 48
typeMap["Elite flash enchantment spell"] = 49
typeMap["Elite form"] = 50
typeMap["Elite scythe attack"] = 51
typeMap["Spear attack"] = 52
typeMap["Elite spear attack"] = 53
typeMap["Elite chant"] = 54
typeMap["Chant"] = 55
typeMap["Echo"] = 56
typeMap["Elite dual attack"] = 57
typeMap["Elite off-hand attack"] = 58
typeMap["Elite melee attack"] = 59
typeMap["Elite axe attack"] = 60
typeMap["Elite hammer attack"] = 61
typeMap["Elite trap"] = 62
typeMap["Elite echo"] = 63
typeMap["Signet (PvE-only)"] = 64
typeMap["Elite glyph"] = 65
typeMap["Enchantment spell (PvE-only)"] = 66
typeMap["Spell (PvE-only)"] = 67
typeMap["Melee attack (PvE-only)"] = 68
typeMap["Binding ritual (PvE-only)"] = 69
typeMap["Shout (PvE-only)"] = 70
typeMap["Touch skill (PvE-only)"] = 71
typeMap["Stance (PvE-only)"] = 72
typeMap["Weapon spell (PvE-only)"] = 73
typeMap["Trap (PvE-only)"] = 74
typeMap["Ranged attack (PvE-only)"] = 75
typeMap["Ward spell (PvE-only)"] = 76
typeMap["Hex spell (PvE-only)"] = 77
typeMap["Elite form (PvE-only)"] = 78
typeMap["Elite ward spell"] = 79
typeMap["Ebon vanguard ritual (PvE-only)"] = 80
typeMap["Bonus Mission Pack skill"] = 81
typeMap["Elite ward spell (PvE-only)"] = 82
typeMap["Elite enchantment spell (PvE-only)"] = 83
typeMap["Elite melee attack (PvE-only)"] = 84
typeMap["Elite stance (PvE-only)"] = 85
typeMap["Elite shout (PvE-only)"] = 86
typeMap["Elite skill (PvE-only)"] = 87
typeMap["Elite weapon spell (PvE-only)"] = 88
typeMap["Elite echo (PvE-only)"] = 89
typeMap["Elite well spell"] = 90


let countTypes = 91;
const serializeStats = stat => `${stat.type}=${stat.value}`;
const serializeSkillInfo = skillInfo => {
    if (skillInfo.type === "Profession")
        return `${skillInfo.type}=${professionMap[skillInfo.value]}`
    if (skillInfo.type === "Attribute")
        return `${skillInfo.type}=${attributeMap[skillInfo.value]}`
    let value = skillInfo.value.trim().replace(/\s+/g, " ")
    value = value.substr(0, 1).toUpperCase() + value.substr(1)
    if (typeMap[value] === undefined) {
        typeMap[value] = countTypes;
        countTypes++;
    }
    return `${skillInfo.type}=${typeMap[value]}`
}

const serializeArray = array => {
    return array.map(record => record.join(";")).join("\n")
}

async function saveCsv(fr_skills, en_skills, skills) {
    const raw_fr = serializeArray(fr_skills)
    const raw_en = serializeArray(en_skills)
    const raw_skill = serializeArray(skills)
    // console.log(raw_skill);
    await fs.writeFile(__dirname + '/skills_fr_FR_bis.csv', raw_fr, {encoding: 'utf8'})
    await fs.writeFile(__dirname + '/skills_en_US_bis.csv', raw_en, {encoding: 'utf8'})
    await fs.writeFile(__dirname + '/skills_bis.csv', raw_skill, {encoding: 'utf8'})
}


const prepareSave = ({fr_skills, en_skills, skills, new_datas}) => {
    const typeMap = {}
    new_datas.forEach((record, i) => {
        const fr_skill = fr_skills.find(r => r[0] === record.id)
        const en_skill = en_skills.find(r => r[0] === record.id)
        const skill = skills.find(r => r[0] === record.id)
        skill[2] = record.imgUrl
        // console.log(skill, fr_skill, record)
        fr_skill.push(record.description)
        en_skill.push(record.description)
        const skillInfos = record.skillInfos.map(serializeSkillInfo).join("|")
        const stats = record.stats.map(serializeStats).join("|")
        // skill.push(skillInfos)
        // skill.push(stats)
        // console.log(skill);
        // console.log(record, stats, skillInfos);
        // console.log(fr_skills[i], en_skills[i]);
    })
    saveCsv(fr_skills, en_skills, skills)
    // console.log(typeMap);

}


const searchSkillInfo = (record, html) => {
    const root = html_parser.parse(html).querySelector("#bodyContent");
    const img = root.querySelector('.skill-image img');
    const skill_stats = root.querySelectorAll(".skill-stats li");
    const imgUrl = "https://wiki.guildwars.com" + img.attributes.src;
    const stats = [];
    skill_stats.forEach(stat => {
        try {
            const statValue = stat.childNodes[0].rawText
            const statType = stat.querySelector("a").attributes.title
            stats.push({value: statValue.trim(), type: statType})
        } catch (e) {
            console.log("Stat read went wrong for ", record[1])
        }
    })

    let rawSkillInfo = root.querySelectorAll(".infobox dl")[0].childNodes.filter(child => child.nodeType === 1).map(child => child.childNodes.map(subChild => subChild.text).join(""));
    let skillInfos = [];
    for (var i = 0; i < rawSkillInfo.length - 2; i += 2) {
        const type = rawSkillInfo[i].replace(/ /g, "");
        const value = rawSkillInfo[i + 1];
        skillInfos.push({type, value})
    }
    let description = root.querySelectorAll("#mw-content-text .mw-parser-output .noexcerpt")[0].text.replace(/\n/g, "");
    return {id: record[0], stats, skillInfos, description, imgUrl}
}

const compare = (a, b) => {
    if (a.id < b.id)
        return -1
    else if (a.id === b.id)
        return 0
    else
        return 1
}
let n = 0;
const fetchSkill = context => record => {
    // if( n++ > 3) return;
    http.get(record[1], (res) => {
        res.setEncoding('utf8');
        let rawData = '';
        res.on('data', (chunk) => {
            rawData += chunk;
        });
        res.on('end', () => {
            try {
                const new_skill_data = searchSkillInfo(record, rawData)
                context.new_datas.push(new_skill_data);
                context.counter--
                if (context.counter === 0) {
                    // context.new_datas.sort(compare)
                    prepareSave(context)
                }

            } catch (e) {
                console.error(e.message);
            }
        })
    }).on('error', (e) => {
        console.error(`Got error: ${e.message}`);
    });
}

// var i18nParser = input => parse_sync(input,{delimiter: ';', from_line: 2})
//
// var parser = parse({delimiter: ';', from_line: 2}, function(err, data){
//     data.forEach(fetchSkill)
// })

const {fr_skills, en_skills, skills} = (async function () {
    const fr_skills_buffer = await fs.readFile(__dirname + '/skills_fr_FR.csv')
    const en_skills_buffer = await fs.readFile(__dirname + '/skills_en_US.csv')
    const skills_buffer = await fs.readFile(__dirname + '/skills.csv')
    const fr_skills = parse_sync(fr_skills_buffer, {delimiter: ';', from_line: 2})
    const en_skills = parse_sync(en_skills_buffer, {delimiter: ';', from_line: 2})
    const skills = parse_sync(skills_buffer, {delimiter: ';', from_line: 2})
    const context = {fr_skills, en_skills, skills, counter: skills.length, new_datas: []}
    const fetchSkillContext = fetchSkill(context)
    skills.forEach(fetchSkillContext)
})()


