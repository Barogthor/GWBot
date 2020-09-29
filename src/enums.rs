use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Hash)]
pub enum Language {
    French,
    English,
}

#[derive(Debug, Copy, Clone)]
pub enum ProfessionType {
    None,
    Warrior,
    Ranger,
    Monk,
    Necromancer,
    Mesmer,
    Elementalist,
    Assassin,
    Ritualist,
    Paragon,
    Dervish,
}

impl Display for ProfessionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u32> for ProfessionType{
    fn from(n: u32) -> Self {
        match n {
            0  => ProfessionType::None,
            1  => ProfessionType::Warrior,
            2  => ProfessionType::Ranger,
            3  => ProfessionType::Monk,
            4  => ProfessionType::Necromancer,
            5  => ProfessionType::Mesmer,
            6  => ProfessionType::Elementalist,
            7  => ProfessionType::Assassin,
            8  => ProfessionType::Ritualist,
            9  => ProfessionType::Paragon,
            10 => ProfessionType::Dervish,
            _ => panic!("unknown profession")
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum AttributeType {
    FastCasting,
    IllusionMagic,
    DominationMagic,
    InspirationMagic,
    BloodMagic,
    DeathMagic,
    SoulReaping,
    Curses,
    AirMagic,
    EarthMagic,
    FireMagic,
    WaterMagic,
    EnergyStorage,
    HealingPrayers,
    SmitingPrayers,
    ProtectionPrayers,
    DivineFavor,
    Strength,
    AxeMastery,
    HammerMastery,
    Swordsmanship,
    Tactics,
    BeastMastery,
    Expertise,
    WildernessSurvival,
    Marksmanship,
    DaggerMastery,
    DeadlyArts,
    ShadowArts,
    Communing,
    RestorationMagic,
    ChannelingMagic,
    CriticalStrikes,
    SpawningPower,
    SpearMastery,
    Command,
    Motivation,
    Leadership,
    ScytheMastery,
    WindPrayers,
    EarthPrayers,
    Mysticism,
}
impl Display for AttributeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<u32> for AttributeType{
    fn from(n: u32) -> Self {
        match n {
            0 => AttributeType::FastCasting,
            1 => AttributeType::IllusionMagic,
            2 => AttributeType::DominationMagic,
            3 => AttributeType::InspirationMagic,
            4 => AttributeType::BloodMagic,
            5 => AttributeType::DeathMagic,
            6 => AttributeType::SoulReaping,
            7 => AttributeType::Curses,
            8 => AttributeType::AirMagic,
            9 => AttributeType::EarthMagic,
            10 => AttributeType::FireMagic,
            11 => AttributeType::WaterMagic,
            12 => AttributeType::EnergyStorage,
            13 => AttributeType::HealingPrayers,
            14 => AttributeType::SmitingPrayers,
            15 => AttributeType::ProtectionPrayers,
            16 => AttributeType::DivineFavor,
            17 => AttributeType::Strength,
            18 => AttributeType::AxeMastery,
            19 => AttributeType::HammerMastery,
            20 => AttributeType::Swordsmanship,
            21 => AttributeType::Tactics,
            22 => AttributeType::BeastMastery,
            23 => AttributeType::Expertise,
            24 => AttributeType::WildernessSurvival,
            25 => AttributeType::Marksmanship,
            29 => AttributeType::DaggerMastery,
            30 => AttributeType::DeadlyArts,
            31 => AttributeType::ShadowArts,
            32 => AttributeType::Communing,
            33 => AttributeType::RestorationMagic,
            34 => AttributeType::ChannelingMagic,
            35 => AttributeType::CriticalStrikes,
            36 => AttributeType::SpawningPower,
            37 => AttributeType::SpearMastery,
            38 => AttributeType::Command,
            39 => AttributeType::Motivation,
            40 => AttributeType::Leadership,
            41 => AttributeType::ScytheMastery,
            42 => AttributeType::WindPrayers,
            43 => AttributeType::EarthPrayers,
            44 => AttributeType::Mysticism,
            _ => panic!("Unknown attribute")
        }
    }
}



// https://wiki.guildwars.com/images/e/e0/
