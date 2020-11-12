use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Write};
use std::str::FromStr;

use chrono::{DateTime, TimeZone, Utc};

use crate::enums::{AttributeType, Language, ProfessionType};
use crate::utils::time::{DateTimeRange, DateTimeRangeComparison};

pub mod skill;
pub mod time;

pub type CSVRecord = Vec<String>;

#[derive(Default, Debug)]
pub struct CSVFile {
    headers: Vec<String>,
    records: Vec<CSVRecord>,
}

impl CSVFile {
    fn read_header(&mut self, iter: &mut Lines<BufReader<File>>){
        let header = iter.next().unwrap().unwrap();
        let header: Vec<&str> = header.split(';').collect();
        for x in header {
            self.headers.push(x.trim().to_string());
        }
    }
    fn read_record(&mut self, raw_record: String) {
        let split_record: Vec<&str> = raw_record.split(';').collect();
        let mut record = vec![];
        for x in split_record {
            record.push(x.trim().to_string());
        }
        self.records.push(record);
    }

    fn save(path: &str, headers: Vec<String>, records: Vec<CSVRecord>) {
        let file = File::create(path).or_else(|_| File::open(path));
        file.and_then(|mut f| {
            let headers = format!("{}\n", headers.join(";"));
            f.write_all(headers.as_bytes());
            records.iter()
                .map(|record| record.join(";"))
                .map(|record| format!("{}\n", record))
                .for_each(|record| {
                    f.write_all(record.as_bytes());
                });

            Ok(())
        });
    }

    fn parse(path: &str) -> Result<Self, ()> {
        let mut csv = CSVFile::default();
        let res = File::open(path);
        match res {
            Ok(f) => {
                let reader = BufReader::new(f);
                let mut iter = reader.lines();
                csv.read_header(&mut iter);
                iter.for_each(|s| csv.read_record(s.unwrap()));
                Ok(csv)
            },
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub struct SkillName {
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct SkillNameStore(HashMap<u32, SkillName>);

impl SkillNameStore {
    pub fn from_csv(path: &str) -> Self {
        let csv = CSVFile::parse(path).expect(&format!("{} doesn't exist", path));
        let mut store = Self { 0: Default::default() };
        for x in csv.records {
            let id = x.get(0).map(|id| u32::from_str(&id).unwrap()).unwrap_or(0);
            let name = x.get(1).unwrap().to_string();
            let description = x.get(2).unwrap().to_string();
            store.0.insert(id, SkillName { name, description });
        }
        store
    }

    pub fn get_from_id(&self, id: u32) -> Option<&SkillName> {
        self.0.get(&id)
    }
}

#[derive(Debug)]
pub struct SkillInfo {
    pub skill_uri: String,
    pub skill_icon: String,
    pub skill_infos: HashMap<String, u32>,
    pub skill_stats: HashMap<String, String>,
}

#[derive(Debug)]
pub struct SkillInfoStore(HashMap<u32, SkillInfo>);

impl SkillInfoStore {
    pub fn from_csv(path: &str) -> Self {
        let csv = CSVFile::parse(path).expect(&format!("{} doesn't exist", path));
        let mut store = Self { 0: Default::default() };
        for x in csv.records {
            let id = x.get(0).map(|id| u32::from_str(&id).unwrap()).unwrap_or(0);
            let skill_uri = x.get(1).unwrap().to_string();
            let skill_icon = x.get(2).unwrap().to_string();
            let skill_infos_raw = x.get(3).unwrap().to_string();
            let skill_infos: HashMap<String, u32> =
                skill_infos_raw
                    .split("|")
                    .filter(|info| !info.starts_with("Special") && info.len() > 0)
                    .map(|info| {
                        let v: Vec<&str> = info.split("=").collect();
                        let id = u32::from_str(v[1]).unwrap();
                        (v[0].to_string(), id)
                    }).collect();
            let skill_stats_raw = x.get(4).unwrap().to_string();
            let skill_stats: HashMap<String, String> =
                skill_stats_raw
                    .split("|")
                    .filter(|stat| stat.len() > 0)
                    .map(|stat| {
                        let v: Vec<&str> = stat.split("=").collect();
                        (v[0].to_string(), v[1].to_string())
                    }).collect();
            store.0.insert(id, SkillInfo { skill_uri, skill_icon, skill_infos, skill_stats });
        }
        store
    }

    pub fn get_from_id(&self, id: u32) -> Option<&SkillInfo> {
        self.0.get(&id)
    }
}

#[derive(Debug)]
pub struct SKillI18nStore(HashMap<Language, SkillNameStore>, SkillInfoStore);

impl SKillI18nStore {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        m.insert(Language::French, SkillNameStore::from_csv("datas/skills_fr_FR.csv"));
        m.insert(Language::English, SkillNameStore::from_csv("datas/skills_en_US.csv"));
        let info_store = SkillInfoStore::from_csv("datas/skills.csv");
        Self(
            m,
            info_store,
        )
    }

    pub fn lang_and_id(&self, lng: Language, id: u32) -> Option<(&SkillName, Option<&SkillInfo>)> {
        self.0.get(&lng)
            .map(|store| store.get_from_id(id))
            .and_then(|skill| skill)
            .map_or(None, |skill| {
                let info = self.1.get_from_id(id);
                Some((skill, info))
            })
    }
}

#[derive(Debug)]
pub struct AttributeName(pub String);

#[derive(Debug)]
pub struct AttributeStore(HashMap<AttributeType, AttributeName>);

impl AttributeStore {
    pub fn from_csv(path: &str) -> Self {
        let csv = CSVFile::parse(path).expect(&format!("{} doesn't exist", path));
        let mut hm = HashMap::new();
        for attr in csv.records {
            let id = u32::from_str(attr.get(0).unwrap()).unwrap();
            let name = attr.get(1).unwrap().to_string();
            hm.insert(AttributeType::from(id), AttributeName(name));
        }
        Self(hm)
    }

    pub fn from(&self, attr: &AttributeType) -> Option<&AttributeName> {
        self.0.get(attr)
    }
}

#[derive(Debug)]
pub struct ProfessionName(pub String);

#[derive(Debug)]
pub struct ProfessionStore(HashMap<ProfessionType, ProfessionName>);

impl ProfessionStore {
    pub fn from_csv(path: &str) -> Self {
        let csv = CSVFile::parse(path).expect(&format!("{} doesn't exist", path));
        let mut hm = HashMap::new();
        for attr in csv.records {
            let id = u32::from_str(attr.get(0).unwrap()).unwrap();
            let name = attr.get(1).unwrap().to_string();
            hm.insert(ProfessionType::from(id), ProfessionName(name));
        }
        Self(hm)
    }

    pub fn from(&self, prof: &ProfessionType) -> Option<&ProfessionName> {
        self.0.get(prof)
    }
}


#[derive(Debug)]
pub struct ZaishenQuestData {
    pub name: String
}

#[derive(Debug)]
pub struct ZaishenQuestStore(Vec<ZaishenQuestData>);

impl ZaishenQuestStore {
    pub fn from_csv(path: &str) -> Self{
        let csv = CSVFile::parse(path).expect(&format!("{} doesn't exist", path));
        let mut store = Self { 0: vec![] };
        for x in csv.records {
            let name = x.get(1).unwrap().to_string();
            store.0.push(ZaishenQuestData{name});
        }
        store
    }

    pub fn get_from_id(&self, id: i64) -> Option<&ZaishenQuestData> {
        self.0.get(id as usize)
    }
}

#[derive(Debug)]
pub struct BonusEventData {
    pub name: String,
    pub description: String,
}
#[derive(Debug)]
pub struct BonusEventStore(Vec<BonusEventData>);

impl BonusEventStore {
    pub fn from_csv(path: &str) -> Self{
        let csv = CSVFile::parse(path).expect(&format!("{} doesn't exist", path));
        let mut store = Self { 0: vec![] };
        for x in csv.records {
            let name = x.get(1).unwrap().to_string();
            let description = x.get(2).unwrap().to_string();
            store.0.push(BonusEventData{name, description});
        }
        store
    }

    pub fn get_from_id(&self, id: i64) -> Option<&BonusEventData> {
        self.0.get(id as usize)
    }
}


#[derive(Debug)]
pub struct NicholasGiftData {
    pub item: String,
    pub location: String,
    pub region: String,
    pub campaign: String,
    pub item_url: String,
    pub location_url: String,
}

#[derive(Debug)]
pub struct NicholasGiftStore(Vec<NicholasGiftData>);

impl NicholasGiftStore {
    pub fn from_csv(path: &str) -> Self {
        let csv = CSVFile::parse(path).expect(&format!("{} doesn't exist", path));
        let mut store = Self { 0: vec![] };
        for x in csv.records {
            let item = x.get(1).unwrap().to_string();
            let location = x.get(2).unwrap().to_string();
            let region = x.get(3).unwrap().to_string();
            let campaign = x.get(4).unwrap().to_string();
            let item_url = x.get(5).unwrap().to_string();
            let location_url = x.get(6).unwrap().to_string();
            store.0.push(NicholasGiftData { item, location, region, campaign, item_url, location_url });
        }
        store
    }

    pub fn get_from_id(&self, id: i64) -> Option<&NicholasGiftData> {
        self.0.get(id as usize)
    }
}


#[derive(Debug)]
pub struct SpecialEventData {
    pub name: String,
    pub note: String,
}

#[derive(Debug)]
pub struct SpecialEventStore(Vec<SpecialEventData>);

impl SpecialEventStore {
    pub fn from_csv(path: &str) -> Self {
        let csv = CSVFile::parse(path).expect(&format!("{} doesn't exist", path));
        let mut store = Self { 0: vec![] };
        for x in csv.records {
            let name = x.get(1).unwrap().to_string();
            let note = x.get(2).unwrap().to_string();
            store.0.push(SpecialEventData { name, note });
        }
        store
    }

    pub fn get_from_id(&self, id: u32) -> Option<&SpecialEventData> {
        self.0.get(id as usize)
    }
}

#[derive(Debug)]
pub struct SpecialEventPeriod(pub u32, pub DateTimeRange<Utc>);

impl SpecialEventPeriod {
    pub fn before(&self, date: &DateTime<Utc>) -> bool {
        self.1.compare(date).eq(&DateTimeRangeComparison::Before)
    }

    pub fn within(&self, date: &DateTime<Utc>) -> bool {
        self.1.compare(date).eq(&DateTimeRangeComparison::Within)
    }

    pub fn after(&self, date: &DateTime<Utc>) -> bool {
        self.1.compare(date).eq(&DateTimeRangeComparison::After)
    }
}

pub fn get_special_events_time_range() -> Vec<SpecialEventPeriod> {
    let mut events = vec![];
    let path = "datas/special_events.csv";
    let csv = CSVFile::parse(path).expect(&format!("{} doesn't exist", path));
    for x in csv.records {
        let id = x.get(0).unwrap();
        let id = u32::from_str(id).unwrap();
        let start = x.get(1).unwrap();
        let start = Utc.datetime_from_str(start, &"%+").unwrap();
        let end = x.get(2).unwrap();
        let end = Utc.datetime_from_str(end, &"%+").unwrap();
        events.push(SpecialEventPeriod(id, DateTimeRange::new(start, end)));
    }
    events
}


#[derive(Debug)]
pub struct I18nMessageStore(HashMap<String, String>);

type Msg<'a> = &'a str;

impl I18nMessageStore {
    pub fn from_csv(path: &str) -> Self {
        let csv = CSVFile::parse(path).expect(&format!("{} doesn't exist", path));
        let mut hm = HashMap::new();
        for x in csv.records {
            let key = x.get(0).unwrap().to_string();
            let content = x.get(1).unwrap().to_string();
            hm.insert(key, content);
        }
        I18nMessageStore(hm)
    }

    pub fn time_days(&self) -> Msg {
        self.0.get("time-days").expect("'time-days' key is missing")
    }
    pub fn event_no_running(&self) -> Msg {
        self.0.get("event-no-running").expect("'event-no-running' key is missing")
    }
    pub fn event_started(&self) -> Msg {
        self.0.get("event-started").expect("'event-started' key is missing")
    }
    pub fn event_end(&self) -> Msg {
        self.0.get("event-end").expect("'event-end' key is missing")
    }
    pub fn event_next(&self) -> Msg {
        self.0.get("event-next").expect("'event-next' key is missing")
    }
    pub fn event_begin(&self) -> Msg {
        self.0.get("event-begin").expect("'event-begin' key is missing")
    }
    pub fn zaishen_quest_headline(&self) -> Msg {
        self.0.get("zaishen-quest-headline").expect("'zaishen-quest-headline' key is missing")
    }
    pub fn zaishen_quest_mission(&self) -> Msg {
        self.0.get("zaishen-quest-mission").expect("'zaishen-quest-mission' key is missing")
    }
    pub fn zaishen_quest_bounty(&self) -> Msg {
        self.0.get("zaishen-quest-bounty").expect("'zaishen-quest-bounty' key is missing")
    }
    pub fn zaishen_quest_combat(&self) -> Msg {
        self.0.get("zaishen-quest-combat").expect("'zaishen-quest-combat' key is missing")
    }
    pub fn zaishen_quest_vanquish(&self) -> Msg {
        self.0.get("zaishen-quest-vanquish").expect("'zaishen-quest-vanquish' key is missing")
    }
    pub fn zaishen_quest_reset(&self) -> Msg {
        self.0.get("zaishen-quest-reset").expect("'zaishen-quest-reset' key is missing")
    }
    pub fn zaishen_quest_tomorrow_headline(&self) -> Msg {
        self.0.get("zaishen-quest-tomorrow-headline").expect("'zaishen-quest-tomorrow-headline' key is missing")
    }
    pub fn nicholas_gift_headline(&self) -> Msg {
        self.0.get("nicholas-gift-headline").expect("'nicholas-gift-headline' key is missing")
    }
    pub fn nicholas_gift_next_headline(&self) -> Msg {
        self.0.get("nicholas-gift-next-headline").expect("'nicholas-gift-next-headline' key is missing")
    }
    pub fn nicholas_gift_collecting(&self) -> Msg {
        self.0.get("nicholas-gift-collecting").expect("'nicholas-gift-collecting' key is missing")
    }
    pub fn nicholas_gift_per(&self) -> Msg {
        self.0.get("nicholas-gift-per").expect("'nicholas-gift-per' key is missing")
    }
    pub fn nicholas_gift_in(&self) -> Msg {
        self.0.get("nicholas-gift-in").expect("'nicholas-gift-in' key is missing")
    }
    pub fn nicholas_gift_moving(&self) -> Msg {
        self.0.get("nicholas-gift-moving").expect("'nicholas-gift-moving' key is missing")
    }
    pub fn nicholas_gift_found(&self) -> Msg {
        self.0.get("nicholas-gift-found").expect("'nicholas-gift-found' key is missing")
    }
    pub fn bonus_headline(&self) -> Msg {
        self.0.get("bonus-headline").expect("'bonus-headline' key is missing")
    }
    pub fn bonus_next_headline(&self) -> Msg {
        self.0.get("bonus-next-headline").expect("'bonus-next-headline' key is missing")
    }
    pub fn bonus_pve(&self) -> Msg {
        self.0.get("bonus-pve").expect("'bonus-pve' key is missing")
    }
    pub fn bonus_pvp(&self) -> Msg {
        self.0.get("bonus-pvp").expect("'bonus-next-pvp' key is missing")
    }
    pub fn bonus_expire(&self) -> Msg {
        self.0.get("bonus-expire").expect("'bonus-next-expire' key is missing")
    }
    pub fn bonus_next_start(&self) -> Msg {
        self.0.get("bonus-next-start").expect("'bonus-next-start' key is missing")
    }
    pub fn skill_prefix(&self) -> Msg {
        self.0.get("skill-prefix").expect("'skill-prefix' key is missing")
    }
}

#[derive(Debug)]
pub struct GuildConfigData {
    language: Language,
    utc: i32,
}

impl GuildConfigData {}

type GuildRawId = u64;

#[derive(Debug)]
pub struct GuildsConfig(HashMap<GuildRawId, GuildConfigData>);

impl GuildsConfig {
    pub fn load() -> Self {
        let mut hm = HashMap::new();
        match CSVFile::parse("user-config.csv") {
            Ok(file) => {
                for x in file.records {
                    let guild = u64::from_str(&x[0]).unwrap();
                    let language = Language::from(&x[1]).unwrap();
                    let utc = i32::from_str(&x[2]).unwrap();
                    hm.insert(guild, GuildConfigData { language, utc });
                }
                Self(hm)
            }
            _ => Self(Default::default())
        }
    }

    fn save(&self) {
        let func: fn(Vec<&str>) -> Vec<String> = |vec| vec.iter().map(|s| s.to_string()).collect();
        let headers = func(vec!["guild", "language", "utc"]);
        let records: Vec<CSVRecord> = self.0.iter()
            .map(|item| {
                let guild = format!("{}", item.0);
                let utc = format!("{}", item.1.utc);
                let lang = format!("{:?}", item.1.language);
                vec![guild, lang, utc]
            }).collect();
        CSVFile::save("user-config.csv", headers, records);
    }

    pub fn set_language(&mut self, guild: GuildRawId, lng: Language) {
        self.0.get_mut(&guild)
            .and_then(|config| {
                config.language = lng;
                Some(true)
            })
            .or_else(|| {
                let config = GuildConfigData { language: lng.clone(), utc: 0 };
                self.0.insert(guild, config);
                Some(true)
            });
        self.save();
    }
    pub fn set_utc(&mut self, guild: GuildRawId, utc: i32) {
        self.0.get_mut(&guild)
            .and_then(|config| {
                config.utc = utc;
                Some(true)
            })
            .or_else(|| {
                let config = GuildConfigData { language: Language::English, utc };
                self.0.insert(guild, config);
                Some(true)
            });
        self.save();
    }
    pub fn get_guild_config(&self, guild: GuildRawId) -> (Language, i32) {
        self.0.get(&guild)
            .and_then(|config| Some((config.language, config.utc)))
            .unwrap_or((Language::English, 0))
    }
}