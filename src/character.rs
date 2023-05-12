use crate::paths::Paths;
use crate::relic::Relic;

pub struct Character<'a>
{
    pub name: String,
    pub path: Paths,
    pub level: i32,

    pub relics: Vec<&'a Relic>,

    // stats
    pub base_hp: i32,
    pub base_atk: i32,
    pub base_def: i32,
    pub base_speed: i32,

    pub base_crit_rate: f32,
    pub base_crit_dmg: f32,
    pub base_path_damage: f32,
    pub base_break_effect: f32,
    pub base_effect_hit_rate: f32,
    pub base_effect_res: f32,
    pub base_outgoing_healing: f32,
    pub base_energy_regen_rate: f32,
}

impl<'a> Character<'a>
{
    pub fn new(
        name: String,
        path: Paths,
        level: i32,
        relics: Vec<&'a Relic>,
        base_hp: i32,
        base_atk: i32,
        base_def: i32,
        base_speed: i32,
        base_crit_rate: f32,
        base_crit_dmg: f32,
        base_path_damage: f32,
        base_break_effect: f32,
        base_effect_hit_rate: f32,
        base_effect_res: f32,
        base_outgoing_healing: f32,
        base_energy_regen_rate: f32) -> Self
    {
        return Character
        {
            name,
            path,
            level,
            relics,
            base_hp,
            base_atk,
            base_def,
            base_speed,
            base_crit_rate,
            base_crit_dmg,
            base_path_damage,
            base_break_effect,
            base_effect_hit_rate,
            base_effect_res,
            base_outgoing_healing,
            base_energy_regen_rate
        };
    }
}

impl<'a> std::string::ToString for Character<'a>
{
    fn to_string(&self) -> String
    {
        return format!
        (
            "{}\n\
            {}\n\
            Path: {}\n\
            Level: {}\n\
            HP: {}\n\
            ATK: {}\n\
            DEF: {}\n\
            Speed: {}\n\
            Crit Rate: {}\n\
            Crit Damage: {}\n\
            Path Damage Boost: {}\n\
            Break Effect: {}\n\
            Effect Hit Rate: {}\n\
            Effect Res: {}\n\
            Outgoing Healing: {}\n\
            Energy Regeneration Rate: {}\n\
            Relics: {{ {} }}",
            self.name,
            "-".repeat(self.name.len() + 1),
            self.path.to_string(),
            self.level,
            self.base_hp,
            self.base_atk,
            self.base_def,
            self.base_speed,
            self.base_crit_rate,
            self.base_crit_dmg,
            self.base_path_damage,
            self.base_break_effect,
            self.base_effect_hit_rate,
            self.base_effect_res,
            self.base_outgoing_healing,
            self.base_energy_regen_rate,
            self.relics.iter().map(|relic| relic.name.clone()).collect::< Vec<String> >().join(", ")
        );
    }
}
