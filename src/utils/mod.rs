use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;

use chrono::{DateTime, TimeZone, Utc};

use crate::utils::time::{DateTimeRange, DateTimeRangeComparison};

pub mod skill;
pub mod time;

pub trait FileParser {
    fn parse(path: &str) -> Self;
}

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
    fn read_record(&mut self, raw_record: String){
        let split_record: Vec<&str> = raw_record.split(';').collect();
        let mut record = vec![];
        for x in split_record {
            record.push(x.trim().to_string());
        }
        self.records.push(record);
    }
}

impl FileParser for CSVFile {
    fn parse(path: &str) -> Self {
        let mut csv = CSVFile::default();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut iter = reader.lines();
        csv.read_header(&mut iter);
        iter.for_each(|s| csv.read_record(s.unwrap()));
        csv
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
        let csv = CSVFile::parse(path);
        let mut store = Self{ 0: vec![] };
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
        let csv = CSVFile::parse(path);
        let mut store = Self{ 0: vec![] };
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
        let csv = CSVFile::parse(path);
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
        let csv = CSVFile::parse(path);
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
    let csv = CSVFile::parse("datas/special_events.csv");
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

pub struct I18nMessageStore(HashMap<String, String>);

type Msg<'a> = &'a str;

impl I18nMessageStore {
    pub fn from_csv(path: &str) -> Self {
        let csv = CSVFile::parse(path);
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
    pub fn bonus_next_pve(&self) -> Msg {
        self.0.get("bonus-next-pve").expect("'bonus-next-pve' key is missing")
    }
    pub fn bonus_next_pvp(&self) -> Msg {
        self.0.get("bonus-next-pvp").expect("'bonus-next-pvp' key is missing")
    }
    pub fn bonus_next_expire(&self) -> Msg {
        self.0.get("bonus-next-expire").expect("'bonus-next-expire' key is missing")
    }
    pub fn bonus_next_start(&self) -> Msg {
        self.0.get("bonus-next-start").expect("'bonus-next-start' key is missing")
    }
    pub fn skill_prefix(&self) -> Msg {
        self.0.get("skill-prefix").expect("'skill-prefix' key is missing")
    }
}