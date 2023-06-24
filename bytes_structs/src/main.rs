#![allow(dead_code)]
#![allow(unused_variables)]
enum Class {
    Warrior(Race),
    Paladin(Race),
    Hunter(Race),
    Rogue(Race),
    Priest(Race),
    DeathKnight(Race),
    Shaman(Race),
    Mage(Race),
    Warlock(Race),
    Monk(Race),
    Druid(Race),
    DemonHunter(Race),
}
impl Class {
    fn createPries
    
}


enum Faction {
    Alliance,
    Horde,
}

    

    


struct RaceTraits{
    racial:String,
    racial_description:String,
    racial_ability:String,
    racial_ability_description:String,
    racial_ability_cooldown:u8,
    racial_ability_duration:u8,
    faction:Faction,  
}


enum Race {
    BloodElf(RaceTraits),
    Orc(RaceTraits),
    Tauren(RaceTraits),
    Troll(RaceTraits),
    Undead(RaceTraits),
    Goblin(RaceTraits),
    NightElf(RaceTraits),
    Draenei(RaceTraits),
    Dwarf(RaceTraits),
    DarkIronDwarf(RaceTraits),
    Gnome(RaceTraits),
    Worgen(RaceTraits),
    Human(RaceTraits),
    Pandaren(RaceTraits),
    Nightborne(RaceTraits),
    HighmountainTauren(RaceTraits),
    VoidElf(RaceTraits),
    LightforgedDraenei(RaceTraits),
    MagharOrc(RaceTraits),
    ZandalariTroll(RaceTraits),
    KulTiran(RaceTraits),
}

struct Character {
    name: String,
    level: u8,
    experience: u32,
    specialization: Specialization,
    specializations: Vec<Specialization>,
    spells: Vec<Spell>,
    class: Class,
    health: u32,
    secondary_resource: Resource,
    secondary_resource_amount: u32,
    secondary_resource_max: u32,
    secondary_resource_regen: u32,
    primary_resource: Resource,
    primary_resource_amount: u32,
    primary_resource_max: u32,
    primary_resource_regen: u32,
    primary_stat: PrimaryStat,
    primary_stat_amount: u32,
    haste : u32,
    mastery : u32,
    crit : u32,
    versatility : u32,
    leech : u32,
    avoidance : u32,
    stamina : u32,
    armor : u32,
    inventory: Vec<Item>,
    equipped: Vec<Item>,
    inventory_size: u8,
} 
impl Character {
    fn createPriest(name:String , level: u8, experience: u32) -> Character {

       let priest:Character = Character {
        name: name,
        level: level,
        experience: experience,
        

       
       
    };
    return priest;
}
}

struct Item {
    name: String,
    description: String,
    item_level: u8,
    min_level: u8,
}
enum PrimaryStat {
    Strength,
    Agility,
    Intellect,
}
enum Role {
    Tank,
    Healer,
    Damage
}
struct Specialization{
    role: Role,
    name: String,
    description: String,
    talents: Vec<Talent>,
   
    spells: Vec<Spell>,
}

struct Talent {
    name: String,
    description: String,
    level: u8,
    selected: bool,
}
struct Spell {
    name: String,
    description: String,
    level: u8,
    cooldown: u8,
    duration: u8,
    cost: u8,
    range: u8,
    damage: u32,
    healing: u32,
    resource: Resource,
    resource_cost: u32,
    resource_gain: u32,
    resource_gain_on_crit: u32,
    resource_gain_on_hit: u32,
    resource_gain_on_cast: u32,
    resource_gain_on_kill: u32,
    resource_gain_on_spell_cast: u32,
    resource_gain_on_spell_crit: u32,
    resource_gain_on_spell_hit: u32,
    resource_gain_on_spell_kill: u32,
    resource_gain_on_spell_tick: u32,
    resource_gain_on_tick: u32,
    resource_gain_on_hit_taken: u32,
    resource_gain_on_crit_taken: u32,
    resource_gain_on_dodge: u32,
    resource_gain_on_parry: u32,
    resource_gain_on_block: u32,
    resource_gain_on_resist: u32,
    resource_gain_on_evade: u32,
    resource_gain_on_immunity: u32,
    resource_gain_on_deflect: u32,
    resource_gain_on_absorb: u32,
    resource_gain_on_interrupt: u32,
    resource_gain_on_dispel: u32,
    resource_gain_on_death: u32,

    resource_gain_on_assist: u32,
    resource_gain_on_spell_interrupt: u32,
    resource_gain_on_spell_dispel: u32,
    resource_gain_on_spell_death: u32,
   
    resource_gain_on_spell_assist: u32,
    resource_gain_on_spell_resist: u32,
    resource_gain_on_spell_evade: u32,
    resource_gain_on_spell_deflect: u32,
    resource_gain_on_spell_absorb: u32,
    resource_gain_on_spell_immunity: u32,
    resource_gain_on_spell_block: u32,
    resource_gain_on_spell_parry: u32,
    resource_gain_on_spell_dodge: u32,
    resource_gain_on_spell_crit_taken: u32,
    resource_gain_on_spell_hit_taken: u32,

    resource_gain_on_tick_taken: u32,
    resource_gain_on_spell_damage_taken: u32,
    resource_gain_on_spell_healing_taken: u32,
    resource_gain_on_damage_taken: u32,
    resource_gain_on_healing_taken: u32,
    resource_gain_on_damage: u32,
    resource_gain_on_healing: u32,
    resource_gain_on_spell_damage: u32,
    resource_gain_on_spell_healing: u32,

}
enum Resource {
    Rage,
    Energy,
    Mana,
    Focus,
    RunicPower,
    Fury,
    Pain,
    Insanity,
    AstralPower,
    Maelstrom,
    Chi,
    HolyPower,
    SoulShards,
    ArcaneCharges,
    ComboPoints,
    Runes,
    None,
}
    







fn main() {
    let x = Class::Mage(Race::BloodElf(RaceTraits { racial: String::from("Increase crit damage by 1%") , racial_description: String::from("Increase crit damage by 1"), racial_ability: String::from("Arcane Torrent"), racial_ability_description: String::from("Removes 1 enemy buff"), racial_ability_cooldown: 60 , racial_ability_duration: 0, faction: (Faction::Horde) }));

}
