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
typeMap['spell'] = 0
typeMap['signet'] = 1
typeMap['hex spell'] = 2
typeMap['ward spell'] = 3
typeMap['bow attack'] = 4
typeMap['enchantment spell'] = 5
typeMap['shout'] = 6
typeMap['stance'] = 7
typeMap['hammer attack'] = 8
typeMap['binding ritual'] = 9
typeMap['lead attack'] = 10
typeMap['trap'] = 11
typeMap['preparation'] = 12
typeMap['spear attack'] = 13
typeMap['nature ritual'] = 14
typeMap['touch skill'] = 15
typeMap['item spell'] = 16
typeMap['dual attack'] = 17
typeMap['weapon spell'] = 18
typeMap['axe attack'] = 19
typeMap['off-hand attack'] = 20
typeMap['flash enchantment spell'] = 21
typeMap['melee attack'] = 22
typeMap['echo'] = 23
typeMap['scythe attack'] = 24
typeMap['chant'] = 25
typeMap['sword attack'] = 26
typeMap['well spell'] = 27
typeMap['skill'] = 28
typeMap['pet attack'] = 29
typeMap['form'] = 30
typeMap['glyph'] = 31
typeMap['ranged attack'] = 32
typeMap['ebon vanguard ritual'] = 33

let countTypes = 0;
const serializeStats = stat => `${stat.type}=${stat.value}`;
const serializeSkillInfo = skillInfo => {
    if (skillInfo.type === "Profession")
        return `${skillInfo.type}=${professionMap[skillInfo.value]}`
    if (skillInfo.type === "Attribute")
        return `${skillInfo.type}=${attributeMap[skillInfo.value]}`
    if (skillInfo.type === "Type") {
        let value = skillInfo.value.trim().replace(/\s+/g, " ").toLowerCase();
        // value = value.substr(0, 1).toUpperCase() + value.substr(1)
        if (typeMap[value] === undefined) {
            typeMap[value] = countTypes;
            countTypes++;
        }
        return `${skillInfo.type}=${typeMap[value]}`
    }
    return `${skillInfo.type}=${skillInfo.value}`
}

const serializeArray = array => {
    return array.map(record => record.join(";")).join("\n")
}

async function saveCsv(fr_skills, en_skills, skills) {
    const raw_fr = "skillId;skillName;skillDescription\n" + serializeArray(fr_skills)
    const raw_en = "skillId;skillName;skillDescription\n" + serializeArray(en_skills)
    const raw_skill = "skillId;skillUri;skillIcon;skillInfos;skillStats;onlyPve;elite\n" + serializeArray(skills)
    // console.log(raw_skill);
    await fs.writeFile(__dirname + '/skills_fr_FR_bis.csv', raw_fr, {encoding: 'utf8'})
    await fs.writeFile(__dirname + '/skills_en_US_bis.csv', raw_en, {encoding: 'utf8'})
    await fs.writeFile(__dirname + '/skills_bis.csv', raw_skill, {encoding: 'utf8'})
}


const prepareSave = ({fr_skills, en_skills, skills, new_datas}) => {

    new_datas.forEach((record, i) => {
        const fr_skill = fr_skills.find(r => r[0] === record.id)
        const en_skill = en_skills.find(r => r[0] === record.id)
        const skill = skills.find(r => r[0] === record.id)
        skill[2] = record.imgUrl
        // console.log(skill, fr_skill, record)
        fr_skill[2] = (record.description)
        en_skill[2] = (record.description)
        const skillInfos = record.skillInfos.map(serializeSkillInfo).join("|")
        const stats = record.stats.map(serializeStats).join("|")
        skill[3] = skillInfos
        skill[4] = stats
        skill.push(record.pve)
        skill.push(record.elite)
        // console.log(skill, record, record.skillInfos.map(serializeSkillInfo).join("|"));
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
    const imgUrl = record[2];
    const stats = [];
    let pve = false;
    let elite = false;
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
    const typeRec = skillInfos.find(skillInfo => skillInfo.type === "Type")
    if (typeRec) {
        const isPve = typeRec.value.includes("(PvE-only)")
        const isElite = typeRec.value.includes("Elite")
        if (isPve) {
            typeRec.value = typeRec.value.replace(/\(PvE-only\)/g, "")
            // skillInfos.push({type: "onlypve", value:"true"})
            pve = true;
        }
        if (isElite) {
            typeRec.value = typeRec.value.replace(/Elite/g, "")
            // skillInfos.push({type: "elite", value:"true"})
            elite = true;
        }
    }
    let description = root.querySelectorAll("#mw-content-text .mw-parser-output .noexcerpt")[0].text.replace(/\n/g, "");
    return {id: record[0], stats, skillInfos, description, imgUrl, pve, elite}
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
                // if (context.counter === context.skills.length-3) {
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


