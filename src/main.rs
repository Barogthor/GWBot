extern crate dotenv;

use std::collections::HashMap;
use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::group,
};
use serenity::framework::StandardFramework;
use serenity::model::prelude::Ready;
use serenity::prelude::TypeMapKey;

use commands::{
    bonus::*,
    bonusnext::*,
    event::*,
    lang::*,
    menu::*,
    nick::*,
    nicknext::*,
    ping::*,
    skill::*,
    // utc::*,
    zq::*,
    zqnext::*,
};

use crate::enums::Language;
use crate::utils::{AttributeStore, BonusEventStore, get_special_events_time_range, GuildsConfig, I18nMessageStore, NicholasGiftStore, ProfessionStore, SKillI18nStore, SpecialEventPeriod, SpecialEventStore, ZaishenQuestStore};

pub mod constants;
pub mod enums;
mod commands;
pub mod utils;

#[group]
#[commands(ping, skill, menu, zq, zqnext, bonus, bonusnext, nick, nicknext, event, lang)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{}#{} is connected!", ready.user.name, ready.user.discriminator);
    }
}

#[tokio::main]
async fn main() {
    // let now = Utc::now();
    // let tt = get_special_events_time_range();
    // println!("{:?} \n", tt);
    // let f = tt.iter()
    //     .filter(|it| it.1.compare(&now).ge(&DateTimeRangeComparison::Within));
    // let fv: Vec<_> = f.collect();
    // println!("{:?}", fv);
    // let augury_rock = Utc.ymd(2011, 3, 3);
    // let drake_kabob = Utc.ymd(2020, 9, 7);
    // let augury_rock_2020 = Utc.ymd(2020, 3, 27);
    // let raisu_2020 = Utc.ymd(2020, 3, 29);
    // let frost_gate = Utc.ymd(2011, 5, 10);
    //
    // let diff = frost_gate.signed_duration_since(augury_rock);
    // let diff2 = augury_rock_2020.signed_duration_since(augury_rock);
    // let diff3 = raisu_2020.signed_duration_since(augury_rock);
    // let diff4 = drake_kabob.sub(Duration::weeks(137));
    // println!("{}", diff.num_days());
    // println!("{}", diff2.num_days() % 69);
    // println!("{}", diff3.num_days() % 69);
    // println!("{}", diff4);
    // let test_date = Utc.ymd(2020, 2, 29);
    // println!("{}", test_date);
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
    {
        // https://docs.rs/serenity/0.8.7/serenity/client/struct.Client.html#structfield.data
        let mut data = client.data.write().await;
        let bot_datas = Arc::new(tokio::sync::RwLock::new(BotData::init()));
        data.insert::<BotData>(bot_datas);
    }


    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

pub type I18nMap<T> = HashMap<Language, T>;

#[derive(Debug)]
pub struct I18nStore<T>(I18nMap<T>);

impl<T> I18nStore<T> {
    pub fn lng(&self, lng: Language) -> Option<&T> {
        self.0.get(&lng)
    }
}

#[derive(Debug)]
pub struct BotData {
    pub zaishen_vanquish: I18nStore<ZaishenQuestStore>,
    pub zaishen_bounty: I18nStore<ZaishenQuestStore>,
    pub zaishen_mission: I18nStore<ZaishenQuestStore>,
    pub zaishen_combat: I18nStore<ZaishenQuestStore>,
    pub nicholas_traveler: I18nStore<NicholasGiftStore>,
    pub bonus_pve: I18nStore<BonusEventStore>,
    pub bonus_pvp: I18nStore<BonusEventStore>,
    pub i18n_messages: I18nStore<I18nMessageStore>,
    pub event: (Vec<SpecialEventPeriod>, I18nStore<SpecialEventStore>),
    pub guilds_config: GuildsConfig,
    pub skills: SKillI18nStore,
    pub attributes: I18nStore<AttributeStore>,
    pub professions: I18nStore<ProfessionStore>,
}

impl BotData {
    pub fn init() -> Self {
        let special_events = {
            let mut m = HashMap::new();
            m.insert(Language::English, SpecialEventStore::from_csv("datas/special_events_en_US.csv"));
            m.insert(Language::French, SpecialEventStore::from_csv("datas/special_events_fr_FR.csv"));
            m
        };

        let special_event_periods: Vec<SpecialEventPeriod> = {
            get_special_events_time_range()
        };
        let nicholas_traveler = {
            let mut m = HashMap::new();
            m.insert(Language::English, NicholasGiftStore::from_csv("datas/nicolas_traveler_en_US.csv"));
            m.insert(Language::French, NicholasGiftStore::from_csv("datas/nicolas_traveler_fr_FR.csv"));
            m
        };
        let bonus_pve_events = {
            let mut m = HashMap::new();
            m.insert(Language::English, BonusEventStore::from_csv("datas/bonus_pve_en_US.csv"));
            m.insert(Language::French, BonusEventStore::from_csv("datas/bonus_pve_fr_FR.csv"));
            m
        };
        let bonus_pvp_events = {
            let mut m = HashMap::new();
            m.insert(Language::English, BonusEventStore::from_csv("datas/bonus_pvp_en_US.csv"));
            m.insert(Language::French, BonusEventStore::from_csv("datas/bonus_pvp_fr_FR.csv"));
            m
        };
        let zaishen_combat_quests = {
            let mut m = HashMap::new();
            m.insert(Language::English, ZaishenQuestStore::from_csv("datas/cz_en_US.csv"));
            m.insert(Language::French, ZaishenQuestStore::from_csv("datas/cz_fr_FR.csv"));
            m
        };
        let zaishen_bounty_quests = {
            let mut m = HashMap::new();
            m.insert(Language::English, ZaishenQuestStore::from_csv("datas/bz_en_US.csv"));
            m.insert(Language::French, ZaishenQuestStore::from_csv("datas/bz_fr_FR.csv"));
            m
        };
        let zaishen_mission_quests = {
            let mut m = HashMap::new();
            m.insert(Language::English, ZaishenQuestStore::from_csv("datas/mz_en_US.csv"));
            m.insert(Language::French, ZaishenQuestStore::from_csv("datas/mz_fr_FR.csv"));
            m
        };
        let zaishen_vanquish_quests = {
            let mut m = HashMap::new();
            m.insert(Language::English, ZaishenQuestStore::from_csv("datas/vz_en_US.csv"));
            m.insert(Language::French, ZaishenQuestStore::from_csv("datas/vz_fr_FR.csv"));
            m
        };

        let attributes = {
            let mut m = HashMap::new();
            m.insert(Language::English, AttributeStore::from_csv("datas/attributes_en_US.csv"));
            m.insert(Language::French, AttributeStore::from_csv("datas/attributes_fr_FR.csv"));
            m
        };

        let professions = {
            let mut m = HashMap::new();
            m.insert(Language::English, ProfessionStore::from_csv("datas/professions_en_US.csv"));
            m.insert(Language::French, ProfessionStore::from_csv("datas/professions_fr_FR.csv"));
            m
        };

        let i18n_messages = {
            let mut m = HashMap::new();
            m.insert(Language::English, I18nMessageStore::from_csv("datas/message_en_US.csv"));
            m.insert(Language::French, I18nMessageStore::from_csv("datas/message_fr_FR.csv"));
            m
        };

        Self {
            zaishen_vanquish: I18nStore(zaishen_vanquish_quests),
            zaishen_bounty: I18nStore(zaishen_bounty_quests),
            zaishen_mission: I18nStore(zaishen_mission_quests),
            zaishen_combat: I18nStore(zaishen_combat_quests),
            nicholas_traveler: I18nStore(nicholas_traveler),
            bonus_pve: I18nStore(bonus_pve_events),
            bonus_pvp: I18nStore(bonus_pvp_events),
            i18n_messages: I18nStore(i18n_messages),
            event: (special_event_periods, I18nStore(special_events)),
            guilds_config: GuildsConfig::load(),
            skills: SKillI18nStore::new(),
            attributes: I18nStore(attributes),
            professions: I18nStore(professions),
        }
        // Arc::new(tokio::sync::RwLock::new(datas))
    }
}

impl TypeMapKey for BotData {
    type Value = Arc<tokio::sync::RwLock<Self>>;
}

#[inline]
pub async fn get_bot_datas(ctx: &Context) -> Arc<tokio::sync::RwLock<BotData>> {
    let data = ctx.data.read().await;
    data.get::<BotData>().expect("Excepted Bot data in the shared map").clone()
}

#[inline]
pub async fn get_mut_bot_datas(ctx: &Context) -> Arc<tokio::sync::RwLock<BotData>> {
    let mut data = ctx.data.write().await;
    data.get_mut::<BotData>().expect("Excepted Bot data in the shared map").clone()
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
// https://wiki.guildwars.com/wiki/Weekly_bonuses
// https://wiki.guildwars.com/wiki/Special_event
// https://kamadan-chat.com/Search.php?search={}
// 1.5-1.6mo
// During Wayfarer's Reverie, the following weekly bonuses are also active (source):
// Elonian Support Bonus
// Extra Luck Bonus
// Faction Support Bonus
// Northern Support Bonus