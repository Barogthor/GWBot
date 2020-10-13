use std::collections::HashMap;

use crate::enums::Language;
use crate::utils::BonusEventStore;
use crate::utils::get_special_events_time_range;
use crate::utils::NicholasGiftStore;
use crate::utils::SpecialEventPeriod;
use crate::utils::SpecialEventStore;
use crate::utils::ZaishenQuestStore;

const INVALID_VALUE: u8 = 255;
pub const DATETIME_FORMAT: &str = "%F %X %Z";

pub const ZAISHEN_MISSION_START: (i32, u32, u32) = (2011, 3, 3);
pub const ZAISHEN_MISSION_SIZE_CYCLE: i64 = 69;
pub const ZAISHEN_BOUNTY_START: (i32, u32, u32) = (2009, 6, 11);
pub const ZAISHEN_BOUNTY_SIZE_CYCLE: i64 = 66;
pub const ZAISHEN_COMBAT_START: (i32, u32, u32) = (2009, 10, 22);
pub const ZAISHEN_COMBAT_SIZE_CYCLE: i64 = 28;
pub const ZAISHEN_VANQUISH_START: (i32, u32, u32) = (2017, 2, 15);
pub const ZAISHEN_VANQUISH_SIZE_CYCLE: i64 = 136;
pub const BONUS_EVENT_START: (i32, u32, u32) = (2020, 8, 10);
pub const BONUS_EVENT_PVE_SIZE_CYCLE: i64 = 9;
pub const BONUS_EVENT_PVP_SIZE_CYCLE: i64 = 6;
pub const NICHOLAS_TRAVELER_START: (i32, u32, u32) = (2018, 1, 22);
pub const NICHOLAS_TRAVELER_SIZE_CYCLE: i64 = 137;

pub const REACTION_ONE: &str = "1\u{fe0f}\u{20e3}";
pub const REACTION_TWO: &str = "2\u{fe0f}\u{20e3}";
pub const REACTION_THREE: &str = "3\u{fe0f}\u{20e3}";
pub const REACTION_FOUR: &str = "4\u{fe0f}\u{20e3}";
pub const REACTION_FIVE: &str = "5\u{fe0f}\u{20e3}";
pub const REACTION_SIX: &str = "6\u{fe0f}\u{20e3}";
pub const REACTION_SEVEN: &str = "7\u{fe0f}\u{20e3}";
pub const REACTION_EIGHT: &str = "8\u{fe0f}\u{20e3}";
pub const EMOTE_MAP: &str = "🗺️";
pub const EMOTE_ARROW_RIGHT: &str = "➡️";
pub const EMOTE_POINT_RIGHT: &str = "👉";
pub const EMOTE_GLASS_RIGHT: &str = "🔎";
pub const EMOTE_GLASS_LEFT: &str = "🔍";

pub const SECOND_PER_MINUTE: u64 = 60;
pub const SECOND_PER_HOUR: u64 = 60 * SECOND_PER_MINUTE;
pub const SECOND_PER_DAY: u64 = 24 * SECOND_PER_HOUR;

pub const STANDARD_DECODE: &[u8; 256] = {
    &[
        INVALID_VALUE, // input 0 (0x0)
        INVALID_VALUE, // input 1 (0x1)
        INVALID_VALUE, // input 2 (0x2)
        INVALID_VALUE, // input 3 (0x3)
        INVALID_VALUE, // input 4 (0x4)
        INVALID_VALUE, // input 5 (0x5)
        INVALID_VALUE, // input 6 (0x6)
        INVALID_VALUE, // input 7 (0x7)
        INVALID_VALUE, // input 8 (0x8)
        INVALID_VALUE, // input 9 (0x9)
        INVALID_VALUE, // input 10 (0xA)
        INVALID_VALUE, // input 11 (0xB)
        INVALID_VALUE, // input 12 (0xC)
        INVALID_VALUE, // input 13 (0xD)
        INVALID_VALUE, // input 14 (0xE)
        INVALID_VALUE, // input 15 (0xF)
        INVALID_VALUE, // input 16 (0x10)
        INVALID_VALUE, // input 17 (0x11)
        INVALID_VALUE, // input 18 (0x12)
        INVALID_VALUE, // input 19 (0x13)
        INVALID_VALUE, // input 20 (0x14)
        INVALID_VALUE, // input 21 (0x15)
        INVALID_VALUE, // input 22 (0x16)
        INVALID_VALUE, // input 23 (0x17)
        INVALID_VALUE, // input 24 (0x18)
        INVALID_VALUE, // input 25 (0x19)
        INVALID_VALUE, // input 26 (0x1A)
        INVALID_VALUE, // input 27 (0x1B)
        INVALID_VALUE, // input 28 (0x1C)
        INVALID_VALUE, // input 29 (0x1D)
        INVALID_VALUE, // input 30 (0x1E)
        INVALID_VALUE, // input 31 (0x1F)
        INVALID_VALUE, // input 32 (0x20)
        INVALID_VALUE, // input 33 (0x21)
        INVALID_VALUE, // input 34 (0x22)
        INVALID_VALUE, // input 35 (0x23)
        INVALID_VALUE, // input 36 (0x24)
        INVALID_VALUE, // input 37 (0x25)
        INVALID_VALUE, // input 38 (0x26)
        INVALID_VALUE, // input 39 (0x27)
        INVALID_VALUE, // input 40 (0x28)
        INVALID_VALUE, // input 41 (0x29)
        INVALID_VALUE, // input 42 (0x2A)
        62, // input 43 (0x2B char '+') => 62 (0x3E)
        INVALID_VALUE, // input 44 (0x2C)
        INVALID_VALUE, // input 45 (0x2D)
        INVALID_VALUE, // input 46 (0x2E)
        63, // input 47 (0x2F char '/') => 63 (0x3F)
        52, // input 48 (0x30 char '0') => 52 (0x34)
        53, // input 49 (0x31 char '1') => 53 (0x35)
        54, // input 50 (0x32 char '2') => 54 (0x36)
        55, // input 51 (0x33 char '3') => 55 (0x37)
        56, // input 52 (0x34 char '4') => 56 (0x38)
        57, // input 53 (0x35 char '5') => 57 (0x39)
        58, // input 54 (0x36 char '6') => 58 (0x3A)
        59, // input 55 (0x37 char '7') => 59 (0x3B)
        60, // input 56 (0x38 char '8') => 60 (0x3C)
        61, // input 57 (0x39 char '9') => 61 (0x3D)
        INVALID_VALUE, // input 58 (0x3A)
        INVALID_VALUE, // input 59 (0x3B)
        INVALID_VALUE, // input 60 (0x3C)
        INVALID_VALUE, // input 61 (0x3D)
        INVALID_VALUE, // input 62 (0x3E)
        INVALID_VALUE, // input 63 (0x3F)
        INVALID_VALUE, // input 64 (0x40)
        0, // input 65 (0x41 char 'A') => 0 (0x0)
        1, // input 66 (0x42 char 'B') => 1 (0x1)
        2, // input 67 (0x43 char 'C') => 2 (0x2)
        3, // input 68 (0x44 char 'D') => 3 (0x3)
        4, // input 69 (0x45 char 'E') => 4 (0x4)
        5, // input 70 (0x46 char 'F') => 5 (0x5)
        6, // input 71 (0x47 char 'G') => 6 (0x6)
        7, // input 72 (0x48 char 'H') => 7 (0x7)
        8, // input 73 (0x49 char 'I') => 8 (0x8)
        9, // input 74 (0x4A char 'J') => 9 (0x9)
        10, // input 75 (0x4B char 'K') => 10 (0xA)
        11, // input 76 (0x4C char 'L') => 11 (0xB)
        12, // input 77 (0x4D char 'M') => 12 (0xC)
        13, // input 78 (0x4E char 'N') => 13 (0xD)
        14, // input 79 (0x4F char 'O') => 14 (0xE)
        15, // input 80 (0x50 char 'P') => 15 (0xF)
        16, // input 81 (0x51 char 'Q') => 16 (0x10)
        17, // input 82 (0x52 char 'R') => 17 (0x11)
        18, // input 83 (0x53 char 'S') => 18 (0x12)
        19, // input 84 (0x54 char 'T') => 19 (0x13)
        20, // input 85 (0x55 char 'U') => 20 (0x14)
        21, // input 86 (0x56 char 'V') => 21 (0x15)
        22, // input 87 (0x57 char 'W') => 22 (0x16)
        23, // input 88 (0x58 char 'X') => 23 (0x17)
        24, // input 89 (0x59 char 'Y') => 24 (0x18)
        25, // input 90 (0x5A char 'Z') => 25 (0x19)
        INVALID_VALUE, // input 91 (0x5B)
        INVALID_VALUE, // input 92 (0x5C)
        INVALID_VALUE, // input 93 (0x5D)
        INVALID_VALUE, // input 94 (0x5E)
        INVALID_VALUE, // input 95 (0x5F)
        INVALID_VALUE, // input 96 (0x60)
        26, // input 97 (0x61 char 'a') => 26 (0x1A)
        27, // input 98 (0x62 char 'b') => 27 (0x1B)
        28, // input 99 (0x63 char 'c') => 28 (0x1C)
        29, // input 100 (0x64 char 'd') => 29 (0x1D)
        30, // input 101 (0x65 char 'e') => 30 (0x1E)
        31, // input 102 (0x66 char 'f') => 31 (0x1F)
        32, // input 103 (0x67 char 'g') => 32 (0x20)
        33, // input 104 (0x68 char 'h') => 33 (0x21)
        34, // input 105 (0x69 char 'i') => 34 (0x22)
        35, // input 106 (0x6A char 'j') => 35 (0x23)
        36, // input 107 (0x6B char 'k') => 36 (0x24)
        37, // input 108 (0x6C char 'l') => 37 (0x25)
        38, // input 109 (0x6D char 'm') => 38 (0x26)
        39, // input 110 (0x6E char 'n') => 39 (0x27)
        40, // input 111 (0x6F char 'o') => 40 (0x28)
        41, // input 112 (0x70 char 'p') => 41 (0x29)
        42, // input 113 (0x71 char 'q') => 42 (0x2A)
        43, // input 114 (0x72 char 'r') => 43 (0x2B)
        44, // input 115 (0x73 char 's') => 44 (0x2C)
        45, // input 116 (0x74 char 't') => 45 (0x2D)
        46, // input 117 (0x75 char 'u') => 46 (0x2E)
        47, // input 118 (0x76 char 'v') => 47 (0x2F)
        48, // input 119 (0x77 char 'w') => 48 (0x30)
        49, // input 120 (0x78 char 'x') => 49 (0x31)
        50, // input 121 (0x79 char 'y') => 50 (0x32)
        51, // input 122 (0x7A char 'z') => 51 (0x33)
        INVALID_VALUE, // input 123 (0x7B)
        INVALID_VALUE, // input 124 (0x7C)
        INVALID_VALUE, // input 125 (0x7D)
        INVALID_VALUE, // input 126 (0x7E)
        INVALID_VALUE, // input 127 (0x7F)
        INVALID_VALUE, // input 128 (0x80)
        INVALID_VALUE, // input 129 (0x81)
        INVALID_VALUE, // input 130 (0x82)
        INVALID_VALUE, // input 131 (0x83)
        INVALID_VALUE, // input 132 (0x84)
        INVALID_VALUE, // input 133 (0x85)
        INVALID_VALUE, // input 134 (0x86)
        INVALID_VALUE, // input 135 (0x87)
        INVALID_VALUE, // input 136 (0x88)
        INVALID_VALUE, // input 137 (0x89)
        INVALID_VALUE, // input 138 (0x8A)
        INVALID_VALUE, // input 139 (0x8B)
        INVALID_VALUE, // input 140 (0x8C)
        INVALID_VALUE, // input 141 (0x8D)
        INVALID_VALUE, // input 142 (0x8E)
        INVALID_VALUE, // input 143 (0x8F)
        INVALID_VALUE, // input 144 (0x90)
        INVALID_VALUE, // input 145 (0x91)
        INVALID_VALUE, // input 146 (0x92)
        INVALID_VALUE, // input 147 (0x93)
        INVALID_VALUE, // input 148 (0x94)
        INVALID_VALUE, // input 149 (0x95)
        INVALID_VALUE, // input 150 (0x96)
        INVALID_VALUE, // input 151 (0x97)
        INVALID_VALUE, // input 152 (0x98)
        INVALID_VALUE, // input 153 (0x99)
        INVALID_VALUE, // input 154 (0x9A)
        INVALID_VALUE, // input 155 (0x9B)
        INVALID_VALUE, // input 156 (0x9C)
        INVALID_VALUE, // input 157 (0x9D)
        INVALID_VALUE, // input 158 (0x9E)
        INVALID_VALUE, // input 159 (0x9F)
        INVALID_VALUE, // input 160 (0xA0)
        INVALID_VALUE, // input 161 (0xA1)
        INVALID_VALUE, // input 162 (0xA2)
        INVALID_VALUE, // input 163 (0xA3)
        INVALID_VALUE, // input 164 (0xA4)
        INVALID_VALUE, // input 165 (0xA5)
        INVALID_VALUE, // input 166 (0xA6)
        INVALID_VALUE, // input 167 (0xA7)
        INVALID_VALUE, // input 168 (0xA8)
        INVALID_VALUE, // input 169 (0xA9)
        INVALID_VALUE, // input 170 (0xAA)
        INVALID_VALUE, // input 171 (0xAB)
        INVALID_VALUE, // input 172 (0xAC)
        INVALID_VALUE, // input 173 (0xAD)
        INVALID_VALUE, // input 174 (0xAE)
        INVALID_VALUE, // input 175 (0xAF)
        INVALID_VALUE, // input 176 (0xB0)
        INVALID_VALUE, // input 177 (0xB1)
        INVALID_VALUE, // input 178 (0xB2)
        INVALID_VALUE, // input 179 (0xB3)
        INVALID_VALUE, // input 180 (0xB4)
        INVALID_VALUE, // input 181 (0xB5)
        INVALID_VALUE, // input 182 (0xB6)
        INVALID_VALUE, // input 183 (0xB7)
        INVALID_VALUE, // input 184 (0xB8)
        INVALID_VALUE, // input 185 (0xB9)
        INVALID_VALUE, // input 186 (0xBA)
        INVALID_VALUE, // input 187 (0xBB)
        INVALID_VALUE, // input 188 (0xBC)
        INVALID_VALUE, // input 189 (0xBD)
        INVALID_VALUE, // input 190 (0xBE)
        INVALID_VALUE, // input 191 (0xBF)
        INVALID_VALUE, // input 192 (0xC0)
        INVALID_VALUE, // input 193 (0xC1)
        INVALID_VALUE, // input 194 (0xC2)
        INVALID_VALUE, // input 195 (0xC3)
        INVALID_VALUE, // input 196 (0xC4)
        INVALID_VALUE, // input 197 (0xC5)
        INVALID_VALUE, // input 198 (0xC6)
        INVALID_VALUE, // input 199 (0xC7)
        INVALID_VALUE, // input 200 (0xC8)
        INVALID_VALUE, // input 201 (0xC9)
        INVALID_VALUE, // input 202 (0xCA)
        INVALID_VALUE, // input 203 (0xCB)
        INVALID_VALUE, // input 204 (0xCC)
        INVALID_VALUE, // input 205 (0xCD)
        INVALID_VALUE, // input 206 (0xCE)
        INVALID_VALUE, // input 207 (0xCF)
        INVALID_VALUE, // input 208 (0xD0)
        INVALID_VALUE, // input 209 (0xD1)
        INVALID_VALUE, // input 210 (0xD2)
        INVALID_VALUE, // input 211 (0xD3)
        INVALID_VALUE, // input 212 (0xD4)
        INVALID_VALUE, // input 213 (0xD5)
        INVALID_VALUE, // input 214 (0xD6)
        INVALID_VALUE, // input 215 (0xD7)
        INVALID_VALUE, // input 216 (0xD8)
        INVALID_VALUE, // input 217 (0xD9)
        INVALID_VALUE, // input 218 (0xDA)
        INVALID_VALUE, // input 219 (0xDB)
        INVALID_VALUE, // input 220 (0xDC)
        INVALID_VALUE, // input 221 (0xDD)
        INVALID_VALUE, // input 222 (0xDE)
        INVALID_VALUE, // input 223 (0xDF)
        INVALID_VALUE, // input 224 (0xE0)
        INVALID_VALUE, // input 225 (0xE1)
        INVALID_VALUE, // input 226 (0xE2)
        INVALID_VALUE, // input 227 (0xE3)
        INVALID_VALUE, // input 228 (0xE4)
        INVALID_VALUE, // input 229 (0xE5)
        INVALID_VALUE, // input 230 (0xE6)
        INVALID_VALUE, // input 231 (0xE7)
        INVALID_VALUE, // input 232 (0xE8)
        INVALID_VALUE, // input 233 (0xE9)
        INVALID_VALUE, // input 234 (0xEA)
        INVALID_VALUE, // input 235 (0xEB)
        INVALID_VALUE, // input 236 (0xEC)
        INVALID_VALUE, // input 237 (0xED)
        INVALID_VALUE, // input 238 (0xEE)
        INVALID_VALUE, // input 239 (0xEF)
        INVALID_VALUE, // input 240 (0xF0)
        INVALID_VALUE, // input 241 (0xF1)
        INVALID_VALUE, // input 242 (0xF2)
        INVALID_VALUE, // input 243 (0xF3)
        INVALID_VALUE, // input 244 (0xF4)
        INVALID_VALUE, // input 245 (0xF5)
        INVALID_VALUE, // input 246 (0xF6)
        INVALID_VALUE, // input 247 (0xF7)
        INVALID_VALUE, // input 248 (0xF8)
        INVALID_VALUE, // input 249 (0xF9)
        INVALID_VALUE, // input 250 (0xFA)
        INVALID_VALUE, // input 251 (0xFB)
        INVALID_VALUE, // input 252 (0xFC)
        INVALID_VALUE, // input 253 (0xFD)
        INVALID_VALUE, // input 254 (0xFE)
        INVALID_VALUE, // input 255 (0xFF)
    ]
};

lazy_static! {

    pub static ref SPECIAL_EVENTS: HashMap<Language, SpecialEventStore> = {
        let mut m = HashMap::new();
        m.insert(Language::English, SpecialEventStore::from_csv("datas/special_events_en_US.csv"));
        m.insert(Language::French, SpecialEventStore::from_csv("datas/special_events_fr_FR.csv"));
        m
    };

    pub static ref SPECIAL_EVENT_PERIODS: Vec<SpecialEventPeriod> = {
        get_special_events_time_range()
    };

    pub static ref NICHOLAS_TRAVELER: HashMap<Language, NicholasGiftStore> = {
        let mut m = HashMap::new();
        m.insert(Language::English, NicholasGiftStore::from_csv("datas/nicolas_traveler_en_US.csv"));
        m.insert(Language::French, NicholasGiftStore::from_csv("datas/nicolas_traveler_fr_FR.csv"));
        m
    };

    pub static ref BONUS_PVE_EVENTS: HashMap<Language, BonusEventStore> = {
        let mut m = HashMap::new();
        m.insert(Language::English, BonusEventStore::from_csv("datas/bonus_pve_en_US.csv"));
        m.insert(Language::French, BonusEventStore::from_csv("datas/bonus_pve_fr_FR.csv"));
        m
    };

    pub static ref BONUS_PVP_EVENTS: HashMap<Language, BonusEventStore> = {
        let mut m = HashMap::new();
        m.insert(Language::English, BonusEventStore::from_csv("datas/bonus_pvp_en_US.csv"));
        m.insert(Language::French, BonusEventStore::from_csv("datas/bonus_pvp_fr_FR.csv"));
        m
    };

    pub static ref ZAISHEN_COMBAT_QUESTS: HashMap<Language, ZaishenQuestStore> = {
        let mut m = HashMap::new();
        m.insert(Language::English, ZaishenQuestStore::from_csv("datas/cz_en_US.csv"));
        m.insert(Language::French, ZaishenQuestStore::from_csv("datas/cz_fr_FR.csv"));
        m
    };
    pub static ref ZAISHEN_BOUNTY_QUESTS: HashMap<Language, ZaishenQuestStore> = {
        let mut m = HashMap::new();
        m.insert(Language::English, ZaishenQuestStore::from_csv("datas/bz_en_US.csv"));
        m.insert(Language::French, ZaishenQuestStore::from_csv("datas/bz_fr_FR.csv"));
        m
    };
    pub static ref ZAISHEN_MISSION_QUESTS: HashMap<Language, ZaishenQuestStore> = {
        let mut m = HashMap::new();
        m.insert(Language::English, ZaishenQuestStore::from_csv("datas/mz_en_US.csv"));
        m.insert(Language::French, ZaishenQuestStore::from_csv("datas/mz_fr_FR.csv"));
        m
    };
    pub static ref ZAISHEN_VANQUISH_QUESTS: HashMap<Language, ZaishenQuestStore> = {
        let mut m = HashMap::new();
        m.insert(Language::English, ZaishenQuestStore::from_csv("datas/vz_en_US.csv"));
        m.insert(Language::French, ZaishenQuestStore::from_csv("datas/vz_fr_FR.csv"));
        m
    };

    pub static ref SKILLS_EN: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();

		m.insert(1,"Healing Signet");
		m.insert(2,"Resurrection Signet");
		m.insert(3,"Signet of Capture");
		m.insert(5,"Power Block");
		m.insert(6,"Mantra of Earth");
		m.insert(7,"Mantra of Flame");
		m.insert(8,"Mantra of Frost");
		m.insert(9,"Mantra of Lightning");
		m.insert(10,"Hex Breaker");
		m.insert(11,"Distortion");
		m.insert(13,"Mantra of Recovery");
		m.insert(14,"Mantra of Persistence");
		m.insert(15,"Mantra of Inscriptions");
		m.insert(16,"Mantra of Concentration");
		m.insert(17,"Mantra of Resolve");
		m.insert(18,"Mantra of Signets");
		m.insert(19,"Fragility");
		m.insert(21,"Inspired Enchantment");
		m.insert(22,"Inspired Hex");
		m.insert(23,"Power Spike");
		m.insert(24,"Power Leak");
		m.insert(25,"Power Drain");
		m.insert(26,"Empathy");
		m.insert(27,"Shatter Delusions");
		m.insert(28,"Backfire");
		m.insert(29,"Blackout");
		m.insert(30,"Diversion");
		m.insert(31,"Conjure Phantasm");
		m.insert(32,"Illusion of Weakness");
		m.insert(33,"Illusionary Weaponry");
		m.insert(34,"Sympathetic Visage");
		m.insert(35,"Ignorance");
		m.insert(36,"Arcane Conundrum");
		m.insert(37,"Illusion of Haste");
		m.insert(38,"Channeling");
		m.insert(39,"Energy Surge");
		m.insert(40,"Ether Feast");
		m.insert(41,"Ether Lord");
		m.insert(42,"Energy Burn");
		m.insert(43,"Clumsiness");
		m.insert(44,"Phantom Pain");
		m.insert(45,"Ethereal Burden");
		m.insert(46,"Guilt");
		m.insert(47,"Ineptitude");
		m.insert(48,"Spirit of Failure");
		m.insert(49,"Mind Wrack");
		m.insert(50,"Wastrel's Worry");
		m.insert(51,"Shame");
		m.insert(52,"Panic");
		m.insert(53,"Migraine");
		m.insert(54,"Crippling Anguish");
		m.insert(55,"Fevered Dreams");
		m.insert(56,"Soothing Images");
		m.insert(57,"Cry of Frustration");
		m.insert(58,"Signet of Midnight");
		m.insert(59,"Signet of Weariness");
		m.insert(61,"Leech Signet");
		m.insert(62,"Signet of Humility");
		m.insert(63,"Keystone Signet");
		m.insert(65,"Arcane Mimicry");
		m.insert(66,"Spirit Shackles");
		m.insert(67,"Shatter Hex");
		m.insert(68,"Drain Enchantment");
		m.insert(69,"Shatter Enchantment");
		m.insert(72,"Elemental Resistance");
		m.insert(73,"Physical Resistance");
		m.insert(74,"Echo");
		m.insert(75,"Arcane Echo");
		m.insert(76,"Imagined Burden");
		m.insert(77,"Chaos Storm");
		m.insert(78,"Epidemic");
		m.insert(79,"Energy Drain");
		m.insert(80,"Energy Tap");
		m.insert(81,"Arcane Thievery");
		m.insert(82,"Mantra of Recall");
		m.insert(83,"Animate Bone Horror");
		m.insert(84,"Animate Bone Fiend");
		m.insert(85,"Animate Bone Minions");
		m.insert(86,"Grenth's Balance");
		m.insert(87,"Verata's Gaze");
		m.insert(88,"Verata's Aura");
		m.insert(89,"Deathly Chill");
		m.insert(90,"Verata's Sacrifice");
		m.insert(91,"Well of Power");
		m.insert(92,"Well of Blood");
		m.insert(93,"Well of Suffering");
		m.insert(94,"Well of the Profane");
		m.insert(95,"Putrid Explosion");
		m.insert(96,"Soul Feast");
		m.insert(97,"Necrotic Traversal");
		m.insert(98,"Consume Corpse");
		m.insert(99,"Parasitic Bond");
		m.insert(100,"Soul Barbs");
		m.insert(101,"Barbs");
		m.insert(102,"Shadow Strike");
		m.insert(103,"Price of Failure");
		m.insert(104,"Death Nova");
		m.insert(105,"Deathly Swarm");
		m.insert(106,"Rotting Flesh");
		m.insert(107,"Virulence");
		m.insert(108,"Suffering");
		m.insert(109,"Life Siphon");
		m.insert(110,"Unholy Feast");
		m.insert(111,"Awaken the Blood");
		m.insert(112,"Desecrate Enchantments");
		m.insert(113,"Tainted Flesh");
		m.insert(114,"Aura of the Lich");
		m.insert(115,"Blood Renewal");
		m.insert(116,"Dark Aura");
		m.insert(117,"Enfeeble");
		m.insert(118,"Enfeebling Blood");
		m.insert(119,"Blood is Power");
		m.insert(120,"Blood of the Master");
		m.insert(121,"Spiteful Spirit");
		m.insert(122,"Malign Intervention");
		m.insert(123,"Insidious Parasite");
		m.insert(124,"Spinal Shivers");
		m.insert(125,"Wither");
		m.insert(126,"Life Transfer");
		m.insert(127,"Mark of Subversion");
		m.insert(128,"Soul Leech");
		m.insert(129,"Defile Flesh");
		m.insert(130,"Demonic Flesh");
		m.insert(131,"Barbed Signet");
		m.insert(132,"Plague Signet");
		m.insert(133,"Dark Pact");
		m.insert(134,"Order of Pain");
		m.insert(135,"Faintheartedness");
		m.insert(136,"Shadow of Fear");
		m.insert(137,"Rigor Mortis");
		m.insert(138,"Dark Bond");
		m.insert(139,"Infuse Condition");
		m.insert(140,"Malaise");
		m.insert(141,"Rend Enchantments");
		m.insert(142,"Lingering Curse");
		m.insert(143,"Strip Enchantment");
		m.insert(144,"Chilblains");
		m.insert(145,"Signet of Agony");
		m.insert(146,"Offering of Blood");
		m.insert(147,"Dark Fury");
		m.insert(148,"Order of the Vampire");
		m.insert(149,"Plague Sending");
		m.insert(150,"Mark of Pain");
		m.insert(151,"Feast of Corruption");
		m.insert(152,"Taste of Death");
		m.insert(153,"Vampiric Gaze");
		m.insert(154,"Plague Touch");
		m.insert(155,"Vile Touch");
		m.insert(156,"Vampiric Touch");
		m.insert(157,"Blood Ritual");
		m.insert(158,"Touch of Agony");
		m.insert(159,"Weaken Armor");
		m.insert(160,"Windborne Speed");
		m.insert(162,"Gale");
		m.insert(163,"Whirlwind");
		m.insert(164,"Elemental Attunement");
		m.insert(165,"Armor of Earth");
		m.insert(166,"Kinetic Armor");
		m.insert(167,"Eruption");
		m.insert(168,"Magnetic Aura");
		m.insert(169,"Earth Attunement");
		m.insert(170,"Earthquake");
		m.insert(171,"Stoning");
		m.insert(172,"Stone Daggers");
		m.insert(173,"Grasping Earth");
		m.insert(174,"Aftershock");
		m.insert(175,"Ward Against Elements");
		m.insert(176,"Ward Against Melee");
		m.insert(177,"Ward Against Foes");
		m.insert(178,"Ether Prodigy");
		m.insert(179,"Incendiary Bonds");
		m.insert(180,"Aura of Restoration");
		m.insert(181,"Ether Renewal");
		m.insert(182,"Conjure Flame");
		m.insert(183,"Inferno");
		m.insert(184,"Fire Attunement");
		m.insert(185,"Mind Burn");
		m.insert(186,"Fireball");
		m.insert(187,"Meteor");
		m.insert(188,"Flame Burst");
		m.insert(189,"Rodgort's Invocation");
		m.insert(190,"Mark of Rodgort");
		m.insert(191,"Immolate");
		m.insert(192,"Meteor Shower");
		m.insert(193,"Phoenix");
		m.insert(194,"Flare");
		m.insert(195,"Lava Font");
		m.insert(196,"Searing Heat");
		m.insert(197,"Fire Storm");
		m.insert(198,"Glyph of Elemental Power");
		m.insert(199,"Glyph of Energy");
		m.insert(200,"Glyph of Lesser Energy");
		m.insert(201,"Glyph of Concentration");
		m.insert(202,"Glyph of Sacrifice");
		m.insert(203,"Glyph of Renewal");
		m.insert(204,"Rust");
		m.insert(205,"Lightning Surge");
		m.insert(206,"Armor of Frost");
		m.insert(207,"Conjure Frost");
		m.insert(208,"Water Attunement");
		m.insert(209,"Mind Freeze");
		m.insert(210,"Ice Prison");
		m.insert(211,"Ice Spikes");
		m.insert(212,"Frozen Burst");
		m.insert(213,"Shard Storm");
		m.insert(214,"Ice Spear");
		m.insert(215,"Maelstrom");
		m.insert(216,"Iron Mist");
		m.insert(217,"Crystal Wave");
		m.insert(218,"Obsidian Flesh");
		m.insert(219,"Obsidian Flame");
		m.insert(220,"Blinding Flash");
		m.insert(221,"Conjure Lightning");
		m.insert(222,"Lightning Strike");
		m.insert(223,"Chain Lightning");
		m.insert(224,"Enervating Charge");
		m.insert(225,"Air Attunement");
		m.insert(226,"Mind Shock");
		m.insert(227,"Glimmering Mark");
		m.insert(228,"Thunderclap");
		m.insert(229,"Lightning Orb");
		m.insert(230,"Lightning Javelin");
		m.insert(231,"Shock");
		m.insert(232,"Lightning Touch");
		m.insert(233,"Swirling Aura");
		m.insert(234,"Deep Freeze");
		m.insert(235,"Blurred Vision");
		m.insert(236,"Mist Form");
		m.insert(237,"Water Trident");
		m.insert(238,"Armor of Mist");
		m.insert(239,"Ward Against Harm");
		m.insert(240,"Smite");
		m.insert(241,"Life Bond");
		m.insert(242,"Balthazar's Spirit");
		m.insert(243,"Strength of Honor");
		m.insert(244,"Life Attunement");
		m.insert(245,"Protective Spirit");
		m.insert(246,"Divine Intervention");
		m.insert(247,"Symbol of Wrath");
		m.insert(248,"Retribution");
		m.insert(249,"Holy Wrath");
		m.insert(250,"Essence Bond");
		m.insert(251,"Scourge Healing");
		m.insert(252,"Banish");
		m.insert(253,"Scourge Sacrifice");
		m.insert(254,"Vigorous Spirit");
		m.insert(255,"Watchful Spirit");
		m.insert(256,"Blessed Aura");
		m.insert(257,"Aegis");
		m.insert(258,"Guardian");
		m.insert(259,"Shield of Deflection");
		m.insert(260,"Aura of Faith");
		m.insert(261,"Shield of Regeneration");
		m.insert(262,"Shield of Judgment");
		m.insert(263,"Protective Bond");
		m.insert(264,"Pacifism");
		m.insert(265,"Amity");
		m.insert(266,"Peace and Harmony");
		m.insert(267,"Judge's Insight");
		m.insert(268,"Unyielding Aura");
		m.insert(269,"Mark of Protection");
		m.insert(270,"Life Barrier");
		m.insert(271,"Zealot's Fire");
		m.insert(272,"Balthazar's Aura");
		m.insert(273,"Spell Breaker");
		m.insert(274,"Healing Seed");
		m.insert(275,"Mend Condition");
		m.insert(276,"Restore Condition");
		m.insert(277,"Mend Ailment");
		m.insert(278,"Purge Conditions");
		m.insert(279,"Divine Healing");
		m.insert(280,"Heal Area");
		m.insert(281,"Orison of Healing");
		m.insert(282,"Word of Healing");
		m.insert(283,"Dwayna's Kiss");
		m.insert(284,"Divine Boon");
		m.insert(285,"Healing Hands");
		m.insert(286,"Heal Other");
		m.insert(287,"Heal Party");
		m.insert(288,"Healing Breeze");
		m.insert(289,"Vital Blessing");
		m.insert(290,"Mending");
		m.insert(291,"Live Vicariously");
		m.insert(292,"Infuse Health");
		m.insert(293,"Signet of Devotion");
		m.insert(294,"Signet of Judgment");
		m.insert(295,"Purge Signet");
		m.insert(296,"Bane Signet");
		m.insert(297,"Blessed Signet");
		m.insert(298,"Martyr");
		m.insert(299,"Shielding Hands");
		m.insert(300,"Contemplation of Purity");
		m.insert(301,"Remove Hex");
		m.insert(302,"Smite Hex");
		m.insert(303,"Convert Hexes");
		m.insert(304,"Light of Dwayna");
		m.insert(305,"Resurrect");
		m.insert(306,"Rebirth");
		m.insert(307,"Reversal of Fortune");
		m.insert(308,"Succor");
		m.insert(309,"Holy Veil");
		m.insert(310,"Divine Spirit");
		m.insert(311,"Draw Conditions");
		m.insert(312,"Holy Strike");
		m.insert(313,"Healing Touch");
		m.insert(314,"Restore Life");
		m.insert(315,"Vengeance");
		m.insert(316,"\"To the Limit!\"");
		m.insert(317,"Battle Rage");
		m.insert(318,"Defy Pain");
		m.insert(319,"Rush");
		m.insert(320,"Hamstring");
		m.insert(321,"Wild Blow");
		m.insert(322,"Power Attack");
		m.insert(323,"Desperation Blow");
		m.insert(324,"Thrill of Victory");
		m.insert(325,"Distracting Blow");
		m.insert(326,"Protector's Strike");
		m.insert(327,"Griffon's Sweep");
		m.insert(328,"Pure Strike");
		m.insert(329,"Skull Crack");
		m.insert(330,"Cyclone Axe");
		m.insert(331,"Hammer Bash");
		m.insert(332,"Bull's Strike");
		m.insert(333,"\"I Will Avenge You!\"");
		m.insert(334,"Axe Rake");
		m.insert(335,"Cleave");
		m.insert(336,"Executioner's Strike");
		m.insert(337,"Dismember");
		m.insert(338,"Eviscerate");
		m.insert(339,"Penetrating Blow");
		m.insert(340,"Disrupting Chop");
		m.insert(341,"Swift Chop");
		m.insert(342,"Axe Twist");
		m.insert(343,"\"For Great Justice!\"");
		m.insert(344,"Flurry");
		m.insert(345,"Defensive Stance");
		m.insert(346,"Frenzy");
		m.insert(347,"Endure Pain");
		m.insert(348,"\"Watch Yourself!\"");
		m.insert(349,"Sprint");
		m.insert(350,"Belly Smash");
		m.insert(351,"Mighty Blow");
		m.insert(352,"Crushing Blow");
		m.insert(353,"Crude Swing");
		m.insert(354,"Earth Shaker");
		m.insert(355,"Devastating Hammer");
		m.insert(356,"Irresistible Blow");
		m.insert(357,"Counter Blow");
		m.insert(358,"Backbreaker");
		m.insert(359,"Heavy Blow");
		m.insert(360,"Staggering Blow");
		m.insert(361,"Dolyak Signet");
		m.insert(362,"Warrior's Cunning");
		m.insert(363,"Shield Bash");
		m.insert(364,"\"Charge!\"");
		m.insert(365,"\"Victory Is Mine!\"");
		m.insert(366,"\"Fear Me!\"");
		m.insert(367,"\"Shields Up!\"");
		m.insert(368,"\"I Will Survive!\"");
		m.insert(370,"Berserker Stance");
		m.insert(371,"Balanced Stance");
		m.insert(372,"Gladiator's Defense");
		m.insert(373,"Deflect Arrows");
		m.insert(374,"Warrior's Endurance");
		m.insert(375,"Dwarven Battle Stance");
		m.insert(376,"Disciplined Stance");
		m.insert(377,"Wary Stance");
		m.insert(378,"Shield Stance");
		m.insert(379,"Bull's Charge");
		m.insert(380,"Bonetti's Defense");
		m.insert(381,"Hundred Blades");
		m.insert(382,"Sever Artery");
		m.insert(383,"Galrath Slash");
		m.insert(384,"Gash");
		m.insert(385,"Final Thrust");
		m.insert(386,"Seeking Blade");
		m.insert(387,"Riposte");
		m.insert(388,"Deadly Riposte");
		m.insert(389,"Flourish");
		m.insert(390,"Savage Slash");
		m.insert(391,"Hunter's Shot");
		m.insert(392,"Pin Down");
		m.insert(393,"Crippling Shot");
		m.insert(394,"Power Shot");
		m.insert(395,"Barrage");
		m.insert(396,"Dual Shot");
		m.insert(397,"Quick Shot");
		m.insert(398,"Penetrating Attack");
		m.insert(399,"Distracting Shot");
		m.insert(400,"Precision Shot");
		m.insert(402,"Determined Shot");
		m.insert(403,"Called Shot");
		m.insert(404,"Poison Arrow");
		m.insert(405,"Oath Shot");
		m.insert(406,"Debilitating Shot");
		m.insert(407,"Point Blank Shot");
		m.insert(408,"Concussion Shot");
		m.insert(409,"Punishing Shot");
		m.insert(411,"Charm Animal");
		m.insert(412,"Call of Protection");
		m.insert(415,"Call of Haste");
		m.insert(422,"Revive Animal");
		m.insert(423,"Symbiotic Bond");
		m.insert(424,"Throw Dirt");
		m.insert(425,"Dodge");
		m.insert(426,"Savage Shot");
		m.insert(427,"Antidote Signet");
		m.insert(428,"Incendiary Arrows");
		m.insert(429,"Melandru's Arrows");
		m.insert(430,"Marksman's Wager");
		m.insert(431,"Ignite Arrows");
		m.insert(432,"Read the Wind");
		m.insert(433,"Kindle Arrows");
		m.insert(434,"Choking Gas");
		m.insert(435,"Apply Poison");
		m.insert(436,"Comfort Animal");
		m.insert(437,"Bestial Pounce");
		m.insert(438,"Maiming Strike");
		m.insert(439,"Feral Lunge");
		m.insert(440,"Scavenger Strike");
		m.insert(441,"Melandru's Assault");
		m.insert(442,"Ferocious Strike");
		m.insert(443,"Predator's Pounce");
		m.insert(444,"Brutal Strike");
		m.insert(445,"Disrupting Lunge");
		m.insert(446,"Troll Unguent");
		m.insert(447,"Otyugh's Cry");
		m.insert(448,"Escape");
		m.insert(449,"Practiced Stance");
		m.insert(450,"Whirling Defense");
		m.insert(451,"Melandru's Resilience");
		m.insert(452,"Dryder's Defenses");
		m.insert(453,"Lightning Reflexes");
		m.insert(454,"Tiger's Fury");
		m.insert(455,"Storm Chaser");
		m.insert(456,"Serpent's Quickness");
		m.insert(457,"Dust Trap");
		m.insert(458,"Barbed Trap");
		m.insert(459,"Flame Trap");
		m.insert(460,"Healing Spring");
		m.insert(461,"Spike Trap");
		m.insert(462,"Winter");
		m.insert(463,"Winnowing");
		m.insert(464,"Edge of Extinction");
		m.insert(465,"Greater Conflagration");
		m.insert(466,"Conflagration");
		m.insert(467,"Fertile Season");
		m.insert(468,"Symbiosis");
		m.insert(469,"Primal Echoes");
		m.insert(470,"Predatory Season");
		m.insert(471,"Frozen Soil");
		m.insert(472,"Favorable Winds");
		m.insert(474,"Energizing Wind");
		m.insert(475,"Quickening Zephyr");
		m.insert(476,"Nature's Renewal");
		m.insert(477,"Muddy Terrain");
		m.insert(570,"Mark of Insecurity");
		m.insert(571,"Disrupting Dagger");
		m.insert(572,"Deadly Paradox");
		m.insert(763,"Jaundiced Gaze");
		m.insert(764,"Wail of Doom");
		m.insert(766,"Gaze of Contempt");
		m.insert(769,"Viper's Defense");
		m.insert(770,"Return");
		m.insert(771,"Aura of Displacement");
		m.insert(772,"Generous Was Tsungrai");
		m.insert(773,"Mighty Was Vorizun");
		m.insert(775,"Death Blossom");
		m.insert(776,"Twisting Fangs");
		m.insert(777,"Horns of the Ox");
		m.insert(778,"Falling Spider");
		m.insert(779,"Black Lotus Strike");
		m.insert(780,"Fox Fangs");
		m.insert(781,"Moebius Strike");
		m.insert(782,"Jagged Strike");
		m.insert(783,"Unsuspecting Strike");
		m.insert(784,"Entangling Asp");
		m.insert(785,"Mark of Death");
		m.insert(786,"Iron Palm");
		m.insert(787,"Resilient Weapon");
		m.insert(788,"Blind Was Mingson");
		m.insert(789,"Grasping Was Kuurong");
		m.insert(790,"Vengeful Was Khanhei");
		m.insert(791,"Flesh of My Flesh");
		m.insert(792,"Splinter Weapon");
		m.insert(793,"Weapon of Warding");
		m.insert(794,"Wailing Weapon");
		m.insert(795,"Nightmare Weapon");
		m.insert(799,"Beguiling Haze");
		m.insert(800,"Enduring Toxin");
		m.insert(801,"Shroud of Silence");
		m.insert(802,"Expose Defenses");
		m.insert(803,"Power Leech");
		m.insert(804,"Arcane Languor");
		m.insert(805,"Animate Vampiric Horror");
		m.insert(806,"Cultist's Fervor");
		m.insert(808,"Reaper's Mark");
		m.insert(809,"Shatterstone");
		m.insert(810,"Protector's Defense");
		m.insert(811,"Run as One");
		m.insert(812,"Defiant Was Xinrae");
		m.insert(813,"Lyssa's Aura");
		m.insert(814,"Shadow Refuge");
		m.insert(815,"Scorpion Wire");
		m.insert(816,"Mirrored Stance");
		m.insert(817,"Discord");
		m.insert(818,"Well of Weariness");
		m.insert(819,"Vampiric Spirit");
		m.insert(820,"Depravity");
		m.insert(821,"Icy Veins");
		m.insert(822,"Weaken Knees");
		m.insert(823,"Burning Speed");
		m.insert(824,"Lava Arrows");
		m.insert(825,"Bed of Coals");
		m.insert(826,"Shadow Form");
		m.insert(827,"Siphon Strength");
		m.insert(828,"Vile Miasma");
		m.insert(830,"Ray of Judgment");
		m.insert(831,"Primal Rage");
		m.insert(832,"Animate Flesh Golem");
		m.insert(834,"Reckless Haste");
		m.insert(835,"Blood Bond");
		m.insert(836,"Ride the Lightning");
		m.insert(837,"Energy Boon");
		m.insert(838,"Dwayna's Sorrow");
		m.insert(839,"\"Retreat!\"");
		m.insert(840,"Poisoned Heart");
		m.insert(841,"Fetid Ground");
		m.insert(842,"Arc Lightning");
		m.insert(843,"Gust");
		m.insert(844,"Churning Earth");
		m.insert(845,"Liquid Flame");
		m.insert(846,"Steam");
		m.insert(847,"Boon Signet");
		m.insert(848,"Reverse Hex");
		m.insert(849,"Lacerating Chop");
		m.insert(850,"Fierce Blow");
		m.insert(851,"Sun and Moon Slash");
		m.insert(852,"Splinter Shot");
		m.insert(853,"Melandru's Shot");
		m.insert(854,"Snare");
		m.insert(858,"Dancing Daggers");
		m.insert(859,"Conjure Nightmare");
		m.insert(860,"Signet of Disruption");
		m.insert(862,"Ravenous Gaze");
		m.insert(863,"Order of Apostasy");
		m.insert(864,"Oppressive Gaze");
		m.insert(865,"Lightning Hammer");
		m.insert(866,"Vapor Blade");
		m.insert(867,"Healing Light");
		m.insert(869,"\"Coward!\"");
		m.insert(870,"Pestilence");
		m.insert(871,"Shadowsong");
		m.insert(876,"Signet of Shadows");
		m.insert(877,"Lyssa's Balance");
		m.insert(878,"Visions of Regret");
		m.insert(879,"Illusion of Pain");
		m.insert(880,"Stolen Speed");
		m.insert(881,"Ether Signet");
		m.insert(882,"Signet of Disenchantment");
		m.insert(883,"Vocal Minority");
		m.insert(884,"Searing Flames");
		m.insert(885,"Shield Guardian");
		m.insert(886,"Restful Breeze");
		m.insert(887,"Signet of Rejuvenation");
		m.insert(888,"Whirling Axe");
		m.insert(889,"Forceful Blow");
		m.insert(891,"\"None Shall Pass!\"");
		m.insert(892,"Quivering Blade");
		m.insert(893,"Seeking Arrows");
		m.insert(898,"Overload");
		m.insert(899,"Images of Remorse");
		m.insert(900,"Shared Burden");
		m.insert(901,"Soul Bind");
		m.insert(902,"Blood of the Aggressor");
		m.insert(903,"Icy Prism");
		m.insert(904,"Furious Axe");
		m.insert(905,"Auspicious Blow");
		m.insert(906,"\"On Your Knees!\"");
		m.insert(907,"Dragon Slash");
		m.insert(908,"Marauder's Shot");
		m.insert(909,"Focused Shot");
		m.insert(910,"Spirit Rift");
		m.insert(911,"Union");
		m.insert(913,"Tranquil Was Tanasen");
		m.insert(914,"Consume Soul");
		m.insert(915,"Spirit Light");
		m.insert(916,"Lamentation");
		m.insert(917,"Rupture Soul");
		m.insert(918,"Spirit to Flesh");
		m.insert(919,"Spirit Burn");
		m.insert(920,"Destruction");
		m.insert(921,"Dissonance");
		m.insert(923,"Disenchantment");
		m.insert(925,"Recall");
		m.insert(926,"Sharpen Daggers");
		m.insert(927,"Shameful Fear");
		m.insert(928,"Shadow Shroud");
		m.insert(929,"Shadow of Haste");
		m.insert(930,"Auspicious Incantation");
		m.insert(931,"Power Return");
		m.insert(932,"Complicate");
		m.insert(933,"Shatter Storm");
		m.insert(934,"Unnatural Signet");
		m.insert(935,"Rising Bile");
		m.insert(936,"Envenom Enchantments");
		m.insert(937,"Shockwave");
		m.insert(938,"Ward of Stability");
		m.insert(939,"Icy Shackles");
		m.insert(941,"Blessed Light");
		m.insert(942,"Withdraw Hexes");
		m.insert(943,"Extinguish");
		m.insert(944,"Signet of Strength");
		m.insert(946,"Trapper's Focus");
		m.insert(947,"Brambles");
		m.insert(948,"Desperate Strike");
		m.insert(949,"Way of the Fox");
		m.insert(950,"Shadowy Burden");
		m.insert(951,"Siphon Speed");
		m.insert(952,"Death's Charge");
		m.insert(953,"Power Flux");
		m.insert(954,"Expel Hexes");
		m.insert(955,"Rip Enchantment");
		m.insert(957,"Spell Shield");
		m.insert(958,"Healing Whisper");
		m.insert(959,"Ethereal Light");
		m.insert(960,"Release Enchantments");
		m.insert(961,"Lacerate");
		m.insert(962,"Spirit Transfer");
		m.insert(963,"Restoration");
		m.insert(964,"Vengeful Weapon");
		m.insert(973,"Blinding Powder");
		m.insert(974,"Mantis Touch");
		m.insert(975,"Exhausting Assault");
		m.insert(976,"Repeating Strike");
		m.insert(977,"Way of the Lotus");
		m.insert(978,"Mark of Instability");
		m.insert(979,"Mistrust");
		m.insert(980,"Feast of Souls");
		m.insert(981,"Recuperation");
		m.insert(982,"Shelter");
		m.insert(983,"Weapon of Shadow");
		m.insert(985,"Caltrops");
		m.insert(986,"Nine Tail Strike");
		m.insert(987,"Way of the Empty Palm");
		m.insert(988,"Temple Strike");
		m.insert(989,"Golden Phoenix Strike");
		m.insert(990,"Expunge Enchantments");
		m.insert(991,"Deny Hexes");
		m.insert(992,"Triple Chop");
		m.insert(993,"Enraged Smash");
		m.insert(994,"Renewing Smash");
		m.insert(995,"Tiger Stance");
		m.insert(996,"Standing Slash");
		m.insert(997,"Famine");
		m.insert(1014,"\"Let's Get 'Em!\"");
		m.insert(1018,"Critical Eye");
		m.insert(1019,"Critical Strike");
		m.insert(1020,"Blades of Steel");
		m.insert(1021,"Jungle Strike");
		m.insert(1022,"Wild Strike");
		m.insert(1023,"Leaping Mantis Sting");
		m.insert(1024,"Black Mantis Thrust");
		m.insert(1025,"Disrupting Stab");
		m.insert(1026,"Golden Lotus Strike");
		m.insert(1027,"Critical Defenses");
		m.insert(1028,"Way of Perfection");
		m.insert(1029,"Dark Apostasy");
		m.insert(1030,"Locust's Fury");
		m.insert(1031,"Shroud of Distress");
		m.insert(1032,"Heart of Shadow");
		m.insert(1033,"Impale");
		m.insert(1034,"Seeping Wound");
		m.insert(1035,"Assassin's Promise");
		m.insert(1036,"Signet of Malice");
		m.insert(1037,"Dark Escape");
		m.insert(1038,"Crippling Dagger");
		m.insert(1040,"Spirit Walk");
		m.insert(1041,"Unseen Fury");
		m.insert(1042,"Flashing Blades");
		m.insert(1043,"Dash");
		m.insert(1044,"Dark Prison");
		m.insert(1045,"Palm Strike");
		m.insert(1048,"Revealed Enchantment");
		m.insert(1049,"Revealed Hex");
		m.insert(1052,"Accumulated Pain");
		m.insert(1053,"Psychic Distraction");
		m.insert(1054,"Ancestor's Visage");
		m.insert(1055,"Recurring Insecurity");
		m.insert(1056,"Kitah's Burden");
		m.insert(1057,"Psychic Instability");
		m.insert(1059,"Hex Eater Signet");
		m.insert(1061,"Feedback");
		m.insert(1062,"Arcane Larceny");
		m.insert(1066,"Spoil Victor");
		m.insert(1067,"Lifebane Strike");
		m.insert(1068,"Bitter Chill");
		m.insert(1069,"Taste of Pain");
		m.insert(1070,"Defile Enchantments");
		m.insert(1071,"Shivers of Dread");
		m.insert(1075,"Vampiric Swarm");
		m.insert(1076,"Blood Drinker");
		m.insert(1077,"Vampiric Bite");
		m.insert(1078,"Wallow's Bite");
		m.insert(1079,"Enfeebling Touch");
		m.insert(1081,"Teinai's Wind");
		m.insert(1082,"Shock Arrow");
		m.insert(1083,"Unsteady Ground");
		m.insert(1084,"Sliver Armor");
		m.insert(1085,"Ash Blast");
		m.insert(1086,"Dragon's Stomp");
		m.insert(1088,"Second Wind");
		m.insert(1090,"Smoldering Embers");
		m.insert(1091,"Double Dragon");
		m.insert(1093,"Teinai's Heat");
		m.insert(1094,"Breath of Fire");
		m.insert(1095,"Star Burst");
		m.insert(1096,"Glyph of Essence");
		m.insert(1097,"Teinai's Prison");
		m.insert(1098,"Mirror of Ice");
		m.insert(1099,"Teinai's Crystals");
		m.insert(1113,"Kirin's Wrath");
		m.insert(1114,"Spirit Bond");
		m.insert(1115,"Air of Enchantment");
		m.insert(1117,"Heaven's Delight");
		m.insert(1118,"Healing Burst");
		m.insert(1119,"Karei's Healing Circle");
		m.insert(1120,"Jamei's Gaze");
		m.insert(1121,"Gift of Health");
		m.insert(1123,"Life Sheath");
		m.insert(1126,"Empathic Removal");
		m.insert(1128,"Resurrection Chant");
		m.insert(1129,"Word of Censure");
		m.insert(1130,"Spear of Light");
		m.insert(1131,"Stonesoul Strike");
		m.insert(1133,"Drunken Blow");
		m.insert(1134,"Leviathan's Sweep");
		m.insert(1135,"Jaizhenju Strike");
		m.insert(1136,"Penetrating Chop");
		m.insert(1137,"Yeti Smash");
		m.insert(1140,"Storm of Swords");
		m.insert(1141,"\"You Will Die!\"");
		m.insert(1142,"Auspicious Parry");
		m.insert(1144,"Silverwing Slash");
		m.insert(1146,"Shove");
		m.insert(1191,"Sundering Attack");
		m.insert(1192,"Zojun's Shot");
		m.insert(1194,"Predatory Bond");
		m.insert(1195,"Heal as One");
		m.insert(1196,"Zojun's Haste");
		m.insert(1197,"Needling Shot");
		m.insert(1198,"Broad Head Arrow");
		m.insert(1199,"Glass Arrows");
		m.insert(1200,"Archer's Signet");
		m.insert(1201,"Savage Pounce");
		m.insert(1202,"Enraged Lunge");
		m.insert(1203,"Bestial Mauling");
		m.insert(1205,"Poisonous Bite");
		m.insert(1206,"Pounce");
		m.insert(1209,"Bestial Fury");
		m.insert(1211,"Viper's Nest");
		m.insert(1212,"Equinox");
		m.insert(1213,"Tranquility");
		m.insert(1215,"Clamor of Souls");
		m.insert(1217,"Ritual Lord");
		m.insert(1218,"Cruel Was Daoshen");
		m.insert(1219,"Protective Was Kaolai");
		m.insert(1220,"Attuned Was Songkai");
		m.insert(1221,"Resilient Was Xiko");
		m.insert(1222,"Lively Was Naomei");
		m.insert(1223,"Anguished Was Lingwah");
		m.insert(1224,"Draw Spirit");
		m.insert(1225,"Channeled Strike");
		m.insert(1226,"Spirit Boon Strike");
		m.insert(1227,"Essence Strike");
		m.insert(1228,"Spirit Siphon");
		m.insert(1229,"Explosive Growth");
		m.insert(1230,"Boon of Creation");
		m.insert(1231,"Spirit Channeling");
		m.insert(1232,"Armor of Unfeeling");
		m.insert(1233,"Soothing Memories");
		m.insert(1234,"Mend Body and Soul");
		m.insert(1235,"Dulled Weapon");
		m.insert(1236,"Binding Chains");
		m.insert(1237,"Painful Bond");
		m.insert(1238,"Signet of Creation");
		m.insert(1239,"Signet of Spirits");
		m.insert(1240,"Soul Twisting");
		m.insert(1244,"Ghostly Haste");
		m.insert(1245,"Gaze from Beyond");
		m.insert(1246,"Ancestors' Rage");
		m.insert(1247,"Pain");
		m.insert(1249,"Displacement");
		m.insert(1250,"Preservation");
		m.insert(1251,"Life");
		m.insert(1252,"Earthbind");
		m.insert(1253,"Bloodsong");
		m.insert(1255,"Wanderlust");
		m.insert(1257,"Spirit Light Weapon");
		m.insert(1258,"Brutal Weapon");
		m.insert(1259,"Guided Weapon");
		m.insert(1260,"Meekness");
		m.insert(1261,"Frigid Armor");
		m.insert(1262,"Healing Ring");
		m.insert(1263,"Renew Life");
		m.insert(1264,"Doom");
		m.insert(1265,"Wielder's Boon");
		m.insert(1266,"Soothing");
		m.insert(1267,"Vital Weapon");
		m.insert(1268,"Weapon of Quickening");
		m.insert(1269,"Signet of Rage");
		m.insert(1333,"Extend Conditions");
		m.insert(1334,"Hypochondria");
		m.insert(1335,"Wastrel's Demise");
		m.insert(1336,"Spiritual Pain");
		m.insert(1337,"Drain Delusions");
		m.insert(1338,"Persistence of Memory");
		m.insert(1339,"Symbols of Inspiration");
		m.insert(1340,"Symbolic Celerity");
		m.insert(1341,"Frustration");
		m.insert(1342,"Tease");
		m.insert(1343,"Ether Phantom");
		m.insert(1344,"Web of Disruption");
		m.insert(1345,"Enchanter's Conundrum");
		m.insert(1346,"Signet of Illusions");
		m.insert(1347,"Discharge Enchantment");
		m.insert(1348,"Hex Eater Vortex");
		m.insert(1349,"Mirror of Disenchantment");
		m.insert(1350,"Simple Thievery");
		m.insert(1351,"Animate Shambling Horror");
		m.insert(1352,"Order of Undeath");
		m.insert(1353,"Putrid Flesh");
		m.insert(1354,"Feast for the Dead");
		m.insert(1355,"Jagged Bones");
		m.insert(1356,"Contagion");
		m.insert(1358,"Ulcerous Lungs");
		m.insert(1359,"Pain of Disenchantment");
		m.insert(1360,"Mark of Fury");
		m.insert(1362,"Corrupt Enchantment");
		m.insert(1363,"Signet of Sorrow");
		m.insert(1364,"Signet of Suffering");
		m.insert(1365,"Signet of Lost Souls");
		m.insert(1366,"Well of Darkness");
		m.insert(1367,"Blinding Surge");
		m.insert(1368,"Chilling Winds");
		m.insert(1369,"Lightning Bolt");
		m.insert(1370,"Storm Djinn's Haste");
		m.insert(1371,"Stone Striker");
		m.insert(1372,"Sandstorm");
		m.insert(1373,"Stone Sheath");
		m.insert(1374,"Ebon Hawk");
		m.insert(1375,"Stoneflesh Aura");
		m.insert(1376,"Glyph of Restoration");
		m.insert(1377,"Ether Prism");
		m.insert(1378,"Master of Magic");
		m.insert(1379,"Glowing Gaze");
		m.insert(1380,"Savannah Heat");
		m.insert(1381,"Flame Djinn's Haste");
		m.insert(1382,"Freezing Gust");
		m.insert(1390,"Judge's Intervention");
		m.insert(1391,"Supportive Spirit");
		m.insert(1392,"Watchful Healing");
		m.insert(1393,"Healer's Boon");
		m.insert(1394,"Healer's Covenant");
		m.insert(1395,"Balthazar's Pendulum");
		m.insert(1396,"Words of Comfort");
		m.insert(1397,"Light of Deliverance");
		m.insert(1398,"Scourge Enchantment");
		m.insert(1399,"Shield of Absorption");
		m.insert(1400,"Reversal of Damage");
		m.insert(1401,"Mending Touch");
		m.insert(1402,"Critical Chop");
		m.insert(1403,"Agonizing Chop");
		m.insert(1404,"Flail");
		m.insert(1405,"Charging Strike");
		m.insert(1406,"Headbutt");
		m.insert(1407,"Lion's Comfort");
		m.insert(1408,"Rage of the Ntouka");
		m.insert(1409,"Mokele Smash");
		m.insert(1410,"Overbearing Smash");
		m.insert(1411,"Signet of Stamina");
		m.insert(1412,"\"You're All Alone!\"");
		m.insert(1413,"Burst of Aggression");
		m.insert(1414,"Enraging Charge");
		m.insert(1415,"Crippling Slash");
		m.insert(1416,"Barbarous Slice");
		m.insert(1465,"Prepared Shot");
		m.insert(1466,"Burning Arrow");
		m.insert(1467,"Arcing Shot");
		m.insert(1468,"Strike as One");
		m.insert(1469,"Crossfire");
		m.insert(1470,"Barbed Arrows");
		m.insert(1471,"Scavenger's Focus");
		m.insert(1472,"Toxicity");
		m.insert(1473,"Quicksand");
		m.insert(1474,"Storm's Embrace");
		m.insert(1475,"Trapper's Speed");
		m.insert(1476,"Tripwire");
		m.insert(1478,"Renewing Surge");
		m.insert(1479,"Offering of Spirit");
		m.insert(1480,"Spirit's Gift");
		m.insert(1481,"Death Pact Signet");
		m.insert(1482,"Reclaim Essence");
		m.insert(1483,"Banishing Strike");
		m.insert(1484,"Mystic Sweep");
		m.insert(1485,"Eremite's Attack");
		m.insert(1486,"Reap Impurities");
		m.insert(1487,"Twin Moon Sweep");
		m.insert(1488,"Victorious Sweep");
		m.insert(1489,"Irresistible Sweep");
		m.insert(1490,"Pious Assault");
		m.insert(1491,"Mystic Twister");
		m.insert(1493,"Grenth's Fingers");
		m.insert(1495,"Aura of Thorns");
		m.insert(1496,"Balthazar's Rage");
		m.insert(1497,"Dust Cloak");
		m.insert(1498,"Staggering Force");
		m.insert(1499,"Pious Renewal");
		m.insert(1500,"Mirage Cloak");
		m.insert(1502,"Arcane Zeal");
		m.insert(1503,"Mystic Vigor");
		m.insert(1504,"Watchful Intervention");
		m.insert(1505,"Vow of Piety");
		m.insert(1506,"Vital Boon");
		m.insert(1507,"Heart of Holy Flame");
		m.insert(1508,"Extend Enchantments");
		m.insert(1509,"Faithful Intervention");
		m.insert(1510,"Sand Shards");
		m.insert(1512,"Lyssa's Haste");
		m.insert(1513,"Guiding Hands");
		m.insert(1514,"Fleeting Stability");
		m.insert(1515,"Armor of Sanctity");
		m.insert(1516,"Mystic Regeneration");
		m.insert(1517,"Vow of Silence");
		m.insert(1518,"Avatar of Balthazar");
		m.insert(1519,"Avatar of Dwayna");
		m.insert(1520,"Avatar of Grenth");
		m.insert(1521,"Avatar of Lyssa");
		m.insert(1522,"Avatar of Melandru");
		m.insert(1523,"Meditation");
		m.insert(1524,"Eremite's Zeal");
		m.insert(1525,"Natural Healing");
		m.insert(1526,"Imbue Health");
		m.insert(1527,"Mystic Healing");
		m.insert(1528,"Dwayna's Touch");
		m.insert(1529,"Pious Restoration");
		m.insert(1530,"Signet of Pious Light");
		m.insert(1531,"Intimidating Aura");
		m.insert(1532,"Mystic Sandstorm");
		m.insert(1533,"Winds of Disenchantment");
		m.insert(1534,"Rending Touch");
		m.insert(1535,"Crippling Sweep");
		m.insert(1536,"Wounding Strike");
		m.insert(1537,"Wearying Strike");
		m.insert(1538,"Lyssa's Assault");
		m.insert(1539,"Chilling Victory");
		m.insert(1540,"Conviction");
		m.insert(1541,"Enchanted Haste");
		m.insert(1542,"Pious Concentration");
		m.insert(1543,"Pious Haste");
		m.insert(1544,"Whirling Charge");
		m.insert(1545,"Test of Faith");
		m.insert(1546,"Blazing Spear");
		m.insert(1547,"Mighty Throw");
		m.insert(1548,"Cruel Spear");
		m.insert(1549,"Harrier's Toss");
		m.insert(1550,"Unblockable Throw");
		m.insert(1551,"Spear of Lightning");
		m.insert(1552,"Wearying Spear");
		m.insert(1553,"Anthem of Fury");
		m.insert(1554,"Crippling Anthem");
		m.insert(1555,"Defensive Anthem");
		m.insert(1556,"Godspeed");
		m.insert(1557,"Anthem of Flame");
		m.insert(1558,"\"Go for the Eyes!\"");
		m.insert(1559,"Anthem of Envy");
		m.insert(1560,"Song of Power");
		m.insert(1561,"Zealous Anthem");
		m.insert(1562,"Aria of Zeal");
		m.insert(1563,"Lyric of Zeal");
		m.insert(1564,"Ballad of Restoration");
		m.insert(1565,"Chorus of Restoration");
		m.insert(1566,"Aria of Restoration");
		m.insert(1567,"Song of Concentration");
		m.insert(1568,"Anthem of Guidance");
		m.insert(1569,"Energizing Chorus");
		m.insert(1570,"Song of Purification");
		m.insert(1571,"Hexbreaker Aria");
		m.insert(1572,"\"Brace Yourself!\"");
		m.insert(1573,"Awe");
		m.insert(1574,"Enduring Harmony");
		m.insert(1575,"Blazing Finale");
		m.insert(1576,"Burning Refrain");
		m.insert(1577,"Finale of Restoration");
		m.insert(1578,"Mending Refrain");
		m.insert(1579,"Purifying Finale");
		m.insert(1580,"Bladeturn Refrain");
		m.insert(1581,"Glowing Signet");
		m.insert(1583,"Leader's Zeal");
		m.insert(1584,"Leader's Comfort");
		m.insert(1585,"Signet of Synergy");
		m.insert(1586,"Angelic Protection");
		m.insert(1587,"Angelic Bond");
		m.insert(1588,"Cautery Signet");
		m.insert(1589,"\"Stand Your Ground!\"");
		m.insert(1590,"\"Lead the Way!\"");
		m.insert(1591,"\"Make Haste!\"");
		m.insert(1592,"\"We Shall Return!\"");
		m.insert(1593,"\"Never Give Up!\"");
		m.insert(1594,"\"Help Me!\"");
		m.insert(1595,"\"Fall Back!\"");
		m.insert(1596,"\"Incoming!\"");
		m.insert(1597,"\"They're on Fire!\"");
		m.insert(1598,"\"Never Surrender!\"");
		m.insert(1599,"\"It's Just a Flesh Wound.\"");
		m.insert(1600,"Barbed Spear");
		m.insert(1601,"Vicious Attack");
		m.insert(1602,"Stunning Strike");
		m.insert(1603,"Merciless Spear");
		m.insert(1604,"Disrupting Throw");
		m.insert(1605,"Wild Throw");
		m.insert(1633,"Malicious Strike");
		m.insert(1634,"Shattering Assault");
		m.insert(1635,"Golden Skull Strike");
		m.insert(1636,"Black Spider Strike");
		m.insert(1637,"Golden Fox Strike");
		m.insert(1638,"Deadly Haste");
		m.insert(1639,"Assassin's Remedy");
		m.insert(1640,"Fox's Promise");
		m.insert(1641,"Feigned Neutrality");
		m.insert(1642,"Hidden Caltrops");
		m.insert(1643,"Assault Enchantments");
		m.insert(1644,"Wastrel's Collapse");
		m.insert(1645,"Lift Enchantment");
		m.insert(1646,"Augury of Death");
		m.insert(1647,"Signet of Toxic Shock");
		m.insert(1648,"Signet of Twilight");
		m.insert(1649,"Way of the Assassin");
		m.insert(1650,"Shadow Walk");
		m.insert(1651,"Death's Retreat");
		m.insert(1652,"Shadow Prison");
		m.insert(1653,"Swap");
		m.insert(1654,"Shadow Meld");
		m.insert(1655,"Price of Pride");
		m.insert(1656,"Air of Disenchantment");
		m.insert(1657,"Signet of Clumsiness");
		m.insert(1658,"Symbolic Posture");
		m.insert(1659,"Toxic Chill");
		m.insert(1660,"Well of Silence");
		m.insert(1661,"Glowstone");
		m.insert(1662,"Mind Blast");
		m.insert(1663,"Elemental Flame");
		m.insert(1664,"Invoke Lightning");
		m.insert(1683,"Pensive Guardian");
		m.insert(1684,"Scribe's Insight");
		m.insert(1685,"Holy Haste");
		m.insert(1686,"Glimmer of Light");
		m.insert(1687,"Zealous Benediction");
		m.insert(1688,"Defender's Zeal");
		m.insert(1689,"Signet of Mystic Wrath");
		m.insert(1690,"Signet of Removal");
		m.insert(1691,"Dismiss Condition");
		m.insert(1692,"Divert Hexes");
		m.insert(1693,"Counterattack");
		m.insert(1694,"Magehunter Strike");
		m.insert(1695,"Soldier's Strike");
		m.insert(1696,"Decapitate");
		m.insert(1697,"Magehunter's Smash");
		m.insert(1698,"Soldier's Stance");
		m.insert(1699,"Soldier's Defense");
		m.insert(1700,"Frenzied Defense");
		m.insert(1701,"Steady Stance");
		m.insert(1702,"Steelfang Slash");
		m.insert(1719,"Screaming Shot");
		m.insert(1720,"Keen Arrow");
		m.insert(1721,"Rampage as One");
		m.insert(1722,"Forked Arrow");
		m.insert(1723,"Disrupting Accuracy");
		m.insert(1724,"Expert's Dexterity");
		m.insert(1725,"Roaring Winds");
		m.insert(1726,"Magebane Shot");
		m.insert(1727,"Natural Stride");
		m.insert(1728,"Heket's Rampage");
		m.insert(1729,"Smoke Trap");
		m.insert(1730,"Infuriating Heat");
		m.insert(1731,"Vocal Was Sogolon");
		m.insert(1732,"Destructive Was Glaive");
		m.insert(1733,"Wielder's Strike");
		m.insert(1734,"Gaze of Fury");
		m.insert(1736,"Spirit's Strength");
		m.insert(1737,"Wielder's Zeal");
		m.insert(1738,"Sight Beyond Sight");
		m.insert(1739,"Renewing Memories");
		m.insert(1740,"Wielder's Remedy");
		m.insert(1741,"Ghostmirror Light");
		m.insert(1742,"Signet of Ghostly Might");
		m.insert(1743,"Signet of Binding");
		m.insert(1744,"Caretaker's Charge");
		m.insert(1745,"Anguish");
		m.insert(1747,"Empowerment");
		m.insert(1748,"Recovery");
		m.insert(1749,"Weapon of Fury");
		m.insert(1750,"Xinrae's Weapon");
		m.insert(1751,"Warmonger's Weapon");
		m.insert(1752,"Weapon of Remedy");
		m.insert(1753,"Rending Sweep");
		m.insert(1754,"Onslaught");
		m.insert(1755,"Mystic Corruption");
		m.insert(1756,"Grenth's Grasp");
		m.insert(1757,"Veil of Thorns");
		m.insert(1758,"Harrier's Grasp");
		m.insert(1759,"Vow of Strength");
		m.insert(1760,"Ebon Dust Aura");
		m.insert(1761,"Zealous Vow");
		m.insert(1762,"Heart of Fury");
		m.insert(1763,"Zealous Renewal");
		m.insert(1764,"Attacker's Insight");
		m.insert(1765,"Rending Aura");
		m.insert(1766,"Featherfoot Grace");
		m.insert(1767,"Reaper's Sweep");
		m.insert(1768,"Harrier's Haste");
		m.insert(1769,"Focused Anger");
		m.insert(1770,"Natural Temper");
		m.insert(1771,"Song of Restoration");
		m.insert(1772,"Lyric of Purification");
		m.insert(1773,"Soldier's Fury");
		m.insert(1774,"Aggressive Refrain");
		m.insert(1775,"Energizing Finale");
		m.insert(1776,"Signet of Aggression");
		m.insert(1777,"Remedy Signet");
		m.insert(1778,"Signet of Return");
		m.insert(1779,"\"Make Your Time!\"");
		m.insert(1780,"\"Can't Touch This!\"");
		m.insert(1781,"\"Find Their Weakness!\"");
		m.insert(1782,"\"The Power Is Yours!\"");
		m.insert(1783,"Slayer's Spear");
		m.insert(1784,"Swift Javelin");
		m.insert(1814,"Lightbringer's Gaze");
		m.insert(1815,"Lightbringer Signet");
		m.insert(1816,"Sunspear Rebirth Signet");
		m.insert(1986,"Vampiric Assault");
		m.insert(1987,"Lotus Strike");
		m.insert(1988,"Golden Fang Strike");
		m.insert(1990,"Falling Lotus Strike");
		m.insert(1991,"Sadist's Signet");
		m.insert(1992,"Signet of Distraction");
		m.insert(1993,"Signet of Recall");
		m.insert(1994,"Power Lock");
		m.insert(1995,"Waste Not, Want Not");
		m.insert(1996,"Sum of All Fears");
		m.insert(1997,"Withering Aura");
		m.insert(1998,"Cacophony");
		m.insert(1999,"Winter's Embrace");
		m.insert(2000,"Earthen Shackles");
		m.insert(2001,"Ward of Weakness");
		m.insert(2002,"Glyph of Swiftness");
		m.insert(2003,"Cure Hex");
		m.insert(2004,"Smite Condition");
		m.insert(2005,"Smiter's Boon");
		m.insert(2006,"Castigation Signet");
		m.insert(2007,"Purifying Veil");
		m.insert(2008,"Pulverizing Smash");
		m.insert(2009,"Keen Chop");
		m.insert(2010,"Knee Cutter");
		m.insert(2011,"Grapple");
		m.insert(2012,"Radiant Scythe");
		m.insert(2013,"Grenth's Aura");
		m.insert(2014,"Signet of Pious Restraint");
		m.insert(2015,"Farmer's Scythe");
		m.insert(2016,"Energetic Was Lee Sa");
		m.insert(2017,"Anthem of Weariness");
		m.insert(2018,"Anthem of Disruption");
		m.insert(2052,"Shadow Fang");
		m.insert(2053,"Calculated Risk");
		m.insert(2054,"Shrinking Armor");
		m.insert(2055,"Aneurysm");
		m.insert(2056,"Wandering Eye");
		m.insert(2057,"Foul Feast");
		m.insert(2058,"Putrid Bile");
		m.insert(2059,"Shell Shock");
		m.insert(2060,"Glyph of Immolation");
		m.insert(2061,"Patient Spirit");
		m.insert(2062,"Healing Ribbon");
		m.insert(2063,"Aura of Stability");
		m.insert(2064,"Spotless Mind");
		m.insert(2065,"Spotless Soul");
		m.insert(2066,"Disarm");
		m.insert(2067,"\"I Meant to Do That!\"");
		m.insert(2068,"Rapid Fire");
		m.insert(2069,"Sloth Hunter's Shot");
		m.insert(2070,"Aura Slicer");
		m.insert(2071,"Zealous Sweep");
		m.insert(2072,"Pure Was Li Ming");
		m.insert(2073,"Weapon of Aggression");
		m.insert(2074,"Chest Thumper");
		m.insert(2075,"Hasty Refrain");
		m.insert(2101,"Critical Agility");
		m.insert(2102,"Cry of Pain");
		m.insert(2103,"Necrosis");
		m.insert(2104,"Intensity");
		m.insert(2105,"Seed of Life");
		m.insert(2107,"Whirlwind Attack");
		m.insert(2108,"Never Rampage Alone");
		m.insert(2109,"Eternal Aura");
		m.insert(2110,"Vampirism");
		m.insert(2112,"\"There's Nothing to Fear!\"");
		m.insert(2116,"Sneak Attack");
		m.insert(2135,"Trampling Ox");
		m.insert(2136,"Smoke Powder Defense");
		m.insert(2137,"Confusing Images");
		m.insert(2138,"Hexer's Vigor");
		m.insert(2139,"Masochism");
		m.insert(2140,"Piercing Trap");
		m.insert(2141,"Companionship");
		m.insert(2142,"Feral Aggression");
		m.insert(2143,"Disrupting Shot");
		m.insert(2144,"Volley");
		m.insert(2145,"Expert Focus");
		m.insert(2146,"Pious Fury");
		m.insert(2147,"Crippling Victory");
		m.insert(2148,"Sundering Weapon");
		m.insert(2149,"Weapon of Renewal");
		m.insert(2150,"Maiming Spear");
		m.insert(2186,"Signet of Deadly Corruption");
		m.insert(2187,"Way of the Master");
		m.insert(2188,"Defile Defenses");
		m.insert(2189,"Angorodon's Gaze");
		m.insert(2190,"Magnetic Surge");
		m.insert(2191,"Slippery Ground");
		m.insert(2192,"Glowing Ice");
		m.insert(2193,"Energy Blast");
		m.insert(2194,"Distracting Strike");
		m.insert(2195,"Symbolic Strike");
		m.insert(2196,"Soldier's Speed");
		m.insert(2197,"Body Blow");
		m.insert(2198,"Body Shot");
		m.insert(2199,"Poison Tip Signet");
		m.insert(2200,"Signet of Mystic Speed");
		m.insert(2201,"Shield of Force");
		m.insert(2202,"Mending Grip");
		m.insert(2203,"Spiritleech Aura");
		m.insert(2204,"Rejuvenation");
		m.insert(2205,"Agony");
		m.insert(2206,"Ghostly Weapon");
		m.insert(2207,"Inspirational Speech");
		m.insert(2208,"Burning Shield");
		m.insert(2209,"Holy Spear");
		m.insert(2210,"Spear Swipe");
		m.insert(2211,"Alkar's Alchemical Acid");
		m.insert(2212,"Light of Deldrimor");
		m.insert(2213,"Ear Bite");
		m.insert(2214,"Low Blow");
		m.insert(2215,"Brawling Headbutt");
		m.insert(2216,"\"Don't Trip!\"");
		m.insert(2217,"\"By Ural's Hammer!\"");
		m.insert(2218,"Drunken Master");
		m.insert(2219,"Great Dwarf Weapon");
		m.insert(2220,"Great Dwarf Armor");
		m.insert(2221,"Breath of the Great Dwarf");
		m.insert(2222,"Snow Storm");
		m.insert(2223,"Black Powder Mine");
		m.insert(2224,"Summon Mursaat");
		m.insert(2225,"Summon Ruby Djinn");
		m.insert(2226,"Summon Ice Imp");
		m.insert(2227,"Summon Naga Shaman");
		m.insert(2228,"Deft Strike");
		m.insert(2229,"Signet of Infection");
		m.insert(2230,"Tryptophan Signet");
		m.insert(2231,"Ebon Battle Standard of Courage");
		m.insert(2232,"Ebon Battle Standard of Wisdom");
		m.insert(2233,"Ebon Battle Standard of Honor");
		m.insert(2234,"Ebon Vanguard Sniper Support");
		m.insert(2235,"Ebon Vanguard Assassin Support");
		m.insert(2236,"Well of Ruin");
		m.insert(2237,"Atrophy");
		m.insert(2238,"Spear of Redemption");
		m.insert(2353,"\"Finish Him!\"");
		m.insert(2354,"\"Dodge This!\"");
		m.insert(2355,"\"I Am the Strongest!\"");
		m.insert(2356,"\"I Am Unstoppable!\"");
		m.insert(2357,"A Touch of Guile");
		m.insert(2358,"\"You Move Like a Dwarf!\"");
		m.insert(2359,"\"You Are All Weaklings!\"");
		m.insert(2360,"Feel No Pain");
		m.insert(2361,"Club of a Thousand Bears");
		m.insert(2374,"Ursan Blessing");
		m.insert(2379,"Volfen Blessing");
		m.insert(2384,"Raven Blessing");
		m.insert(2411,"Mindbender");
		m.insert(2412,"Smooth Criminal");
		m.insert(2413,"Technobabble");
		m.insert(2414,"Radiation Field");
		m.insert(2415,"Asuran Scan");
		m.insert(2416,"Air of Superiority");
		m.insert(2417,"Mental Block");
		m.insert(2418,"Pain Inverter");
		m.insert(2420,"Ebon Escape");
		m.insert(2421,"Weakness Trap");
		m.insert(2422,"Winds");
		m.insert(2423,"Dwarven Stability");
		m.insert(2657,"Call of Haste (PvP)");
		m.insert(2683,"Castigation Signet (Saul D'Alessio)");
		m.insert(2684,"Unnatural Signet (Saul D'Alessio)");
		m.insert(2734,"Mind Wrack (PvP)");
		m.insert(2803,"Mind Freeze (PvP)");
		m.insert(2804,"Mind Shock (PvP)");
		m.insert(2805,"Mist Form (PvP)");
		m.insert(2806,"Ward Against Harm (PvP)");
		m.insert(2807,"Ride the Lightning (PvP)");
		m.insert(2808,"Enraged Smash (PvP)");
		m.insert(2809,"Obsidian Flame (PvP)");
		m.insert(2857,"Aegis (PvP)");
		m.insert(2858,"\"Watch Yourself!\" (PvP)");
		m.insert(2859,"Enfeeble (PvP)");
		m.insert(2860,"Ether Renewal (PvP)");
		m.insert(2861,"Penetrating Attack (PvP)");
		m.insert(2862,"Shadow Form (PvP)");
		m.insert(2863,"Discord (PvP)");
		m.insert(2864,"Sundering Attack (PvP)");
		m.insert(2866,"Flesh of My Flesh (PvP)");
		m.insert(2867,"Ancestors' Rage (PvP)");
		m.insert(2868,"Splinter Weapon (PvP)");
		m.insert(2869,"Assassin's Remedy (PvP)");
		m.insert(2871,"Light of Deliverance (PvP)");
		m.insert(2872,"Death Pact Signet (PvP)");
		m.insert(2875,"Harrier's Toss (PvP)");
		m.insert(2876,"Defensive Anthem (PvP)");
		m.insert(2877,"Ballad of Restoration (PvP)");
		m.insert(2878,"Song of Restoration (PvP)");
		m.insert(2879,"\"Incoming!\" (PvP)");
		m.insert(2880,"\"Never Surrender!\" (PvP)");
		m.insert(2883,"\"For Great Justice!\" (PvP)");
		m.insert(2884,"Mystic Regeneration (PvP)");
		m.insert(2885,"Enfeebling Blood (PvP)");
		m.insert(2887,"Signet of Judgment (PvP)");
		m.insert(2891,"Unyielding Aura (PvP)");
		m.insert(2892,"Spirit Bond (PvP)");
		m.insert(2893,"Weapon of Warding (PvP)");
		m.insert(2895,"Smiter's Boon (PvP)");
		m.insert(2925,"Sloth Hunter's Shot (PvP)");
		m.insert(2959,"Expert's Dexterity (PvP)");
		m.insert(2965,"Signet of Spirits (PvP)");
		m.insert(2966,"Signet of Ghostly Might (PvP)");
		m.insert(2969,"Read the Wind (PvP)");
		m.insert(2998,"Fragility (PvP)");
		m.insert(2999,"Strength of Honor (PvP)");
		m.insert(3002,"Warrior's Endurance (PvP)");
		m.insert(3003,"Armor of Unfeeling (PvP)");
		m.insert(3005,"Union (PvP)");
		m.insert(3006,"Shadowsong (PvP)");
		m.insert(3007,"Pain (PvP)");
		m.insert(3008,"Destruction (PvP)");
		m.insert(3009,"Soothing (PvP)");
		m.insert(3010,"Displacement (PvP)");
		m.insert(3011,"Preservation (PvP)");
		m.insert(3012,"Life (PvP)");
		m.insert(3013,"Recuperation (PvP)");
		m.insert(3014,"Dissonance (PvP)");
		m.insert(3015,"Earthbind (PvP)");
		m.insert(3016,"Shelter (PvP)");
		m.insert(3017,"Disenchantment (PvP)");
		m.insert(3018,"Restoration (PvP)");
		m.insert(3019,"Bloodsong (PvP)");
		m.insert(3020,"Wanderlust (PvP)");
		m.insert(3021,"Savannah Heat (PvP)");
		m.insert(3022,"Gaze of Fury (PvP)");
		m.insert(3023,"Anguish (PvP)");
		m.insert(3024,"Empowerment (PvP)");
		m.insert(3025,"Recovery (PvP)");
		m.insert(3026,"\"Go for the Eyes!\" (PvP)");
		m.insert(3027,"\"Brace Yourself!\" (PvP)");
		m.insert(3028,"Blazing Finale (PvP)");
		m.insert(3029,"Bladeturn Refrain (PvP)");
		m.insert(3030,"Signet of Return (PvP)");
		m.insert(3031,"\"Can't Touch This!\" (PvP)");
		m.insert(3032,"\"Stand Your Ground!\" (PvP)");
		m.insert(3033,"\"We Shall Return!\" (PvP)");
		m.insert(3034,"\"Find Their Weakness!\" (PvP)");
		m.insert(3035,"\"Never Give Up!\" (PvP)");
		m.insert(3036,"\"Help Me!\" (PvP)");
		m.insert(3037,"\"Fall Back!\" (PvP)");
		m.insert(3038,"Agony (PvP)");
		m.insert(3039,"Rejuvenation (PvP)");
		m.insert(3040,"Anthem of Disruption (PvP)");
		m.insert(3045,"Comfort Animal (PvP)");
		m.insert(3047,"Melandru's Assault (PvP)");
		m.insert(3048,"Shroud of Distress (PvP)");
		m.insert(3049,"Unseen Fury (PvP)");
		m.insert(3050,"Predatory Bond (PvP)");
		m.insert(3051,"Enraged Lunge (PvP)");
		m.insert(3053,"Signet of Deadly Corruption (PvP)");
		m.insert(3054,"Masochism (PvP)");
		m.insert(3058,"Unholy Feast (PvP)");
		m.insert(3059,"Signet of Agony (PvP)");
		m.insert(3060,"Escape (PvP)");
		m.insert(3061,"Death Blossom (PvP)");
		m.insert(3062,"Finale of Restoration (PvP)");
		m.insert(3063,"Mantra of Resolve (PvP)");
		m.insert(3068,"Charm Animal (Codex)");
		m.insert(3141,"Lightning Reflexes (PvP)");
		m.insert(3143,"Renewing Smash (PvP)");
		m.insert(3144,"Heal as One (PvP)");
		m.insert(3145,"Glass Arrows (PvP)");
		m.insert(3147,"Keen Arrow (PvP)");
		m.insert(3148,"Anthem of Envy (PvP)");
		m.insert(3149,"Mending Refrain (PvP)");
		m.insert(3151,"Empathy (PvP)");
		m.insert(3152,"Crippling Anguish (PvP)");
		m.insert(3156,"Soldier's Stance (PvP)");
		m.insert(3157,"Destructive Was Glaive (PvP)");
		m.insert(3179,"Mantra of Signets (PvP)");
		m.insert(3180,"Shatter Delusions (PvP)");
		m.insert(3181,"Illusionary Weaponry (PvP)");
		m.insert(3183,"Migraine (PvP)");
		m.insert(3184,"Accumulated Pain (PvP)");
		m.insert(3185,"Psychic Instability (PvP)");
		m.insert(3186,"Shared Burden (PvP)");
		m.insert(3187,"Stolen Speed (PvP)");
		m.insert(3188,"Unnatural Signet (PvP)");
		m.insert(3189,"Spiritual Pain (PvP)");
		m.insert(3190,"Frustration (PvP)");
		m.insert(3191,"Mistrust (PvP)");
		m.insert(3192,"Enchanter's Conundrum (PvP)");
		m.insert(3193,"Signet of Clumsiness (PvP)");
		m.insert(3194,"Mirror of Disenchantment (PvP)");
		m.insert(3195,"Wandering Eye (PvP)");
		m.insert(3196,"Calculated Risk (PvP)");
		m.insert(3204,"Defy Pain (PvP)");
		m.insert(3232,"Heal Party (PvP)");
		m.insert(3233,"Spoil Victor (PvP)");
		m.insert(3234,"Visions of Regret (PvP)");
		m.insert(3251,"Fox Fangs (PvP)");
		m.insert(3252,"Wild Strike (PvP)");
		m.insert(3263,"Banishing Strike (PvP)");
		m.insert(3264,"Twin Moon Sweep (PvP)");
		m.insert(3265,"Irresistible Sweep (PvP)");
		m.insert(3266,"Pious Assault (PvP)");
		m.insert(3269,"Guiding Hands (PvP)");
		m.insert(3270,"Avatar of Dwayna (PvP)");
		m.insert(3271,"Avatar of Melandru (PvP)");
		m.insert(3272,"Mystic Healing (PvP)");
		m.insert(3273,"Signet of Pious Restraint (PvP)");
		m.insert(3289,"Fevered Dreams (PvP)");
		m.insert(3346,"Aura of Thorns (PvP)");
		m.insert(3347,"Dust Cloak (PvP)");
		m.insert(3348,"Lyssa's Haste (PvP)");
		m.insert(3365,"Onslaught (PvP)");
		m.insert(3366,"Heart of Fury (PvP)");
		m.insert(3367,"Wounding Strike (PvP)");
		m.insert(3368,"Pious Fury (PvP)");
		m.insert(3373,"Illusion of Haste (PvP)");
		m.insert(3374,"Illusion of Pain (PvP)");
		m.insert(3375,"Aura of Restoration (PvP)");
		m.insert(3386,"Web of Disruption (PvP)");
		m.insert(3396,"Lightning Hammer (PvP)");
		m.insert(3397,"Elemental Flame (PvP)");
		m.insert(3398,"Slippery Ground (PvP)");
		m.insert(3422,"Time Ward");
		m.insert(3423,"Soul Taker");
		m.insert(3424,"Over the Limit");
		m.insert(3425,"Judgment Strike");
		m.insert(3426,"Seven Weapons Stance");
		m.insert(3427,"\"Together as One!\"");
		m.insert(3428,"Shadow Theft");
		m.insert(3429,"Weapons of Three Forges");
		m.insert(3430,"Vow of Revolution");
		m.insert(3431,"Heroic Refrain");
        m
    };
}
