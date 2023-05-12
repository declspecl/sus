pub enum RelicType
{
    HEAD,
    BODY,
    HANDS,
    FEET,
    PLANARSPHERE,
    LINKROPE,
}

// TODO: change structure to have base main flat and percent stats, expand relictype enum to cover
// every type

pub struct Relic
{
    pub relic_type: RelicType,
    pub name: String,
    pub rarity: i32,

    /*
     *  MAIN STATS
    */

    // head exclusive
    pub base_main_flat_hp: i32,

    // hands exclusive
    pub base_main_flat_atk: i32,

    // body exclusive
    pub base_main_crit_rate: f32,
    pub base_main_crit_dmg: f32,
    pub base_main_outgoing_healing_boost: f32,
    pub base_main_effect_hit_rate: f32,

    // feet exclusive
    pub base_main_flat_speed: i32,

    // planar sphere exclusive
    pub base_main_percent_path_dmg_boost: f32,

    // link rope exclusive
    pub base_main_energy_regen_rate: f32,

    // body, feet, planar sphere, and link rope
    pub base_main_percent_hp: f32,
    pub base_main_percent_atk: f32,
    pub base_main_percent_def: f32,

    // feet and link rope
    pub base_main_break_effect: f32,

    /*
     *  SUB STATS
    */

    pub base_sub_flat_hp: i32,
    pub base_sub_flat_atk: i32,
    pub base_sub_flat_def: i32,
    pub base_sub_flat_speed: i32,

    pub base_sub_percent_hp: f32,
    pub base_sub_percent_atk: f32,
    pub base_sub_percent_def: f32,

    pub base_sub_crit_rate: f32,
    pub base_sub_crit_dmg: f32,
    pub base_sub_effect_res: f32,
    pub base_sub_break_effect: f32,
}

pub type Head = Relic;
pub type Hands = Relic;
pub type Body = Relic;
pub type Feet = Relic;
pub type PlanarSphere = Relic;
pub type LinkRope = Relic;

impl Relic
{
    fn new_head(name: String, rarity: i32, base_main_flat_hp: i32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Head
    {
        return Head
        {
            relic_type: RelicType::HEAD,
            name,
            rarity,
            base_main_flat_hp,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk: 0.0,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_hands(name: String, rarity: i32, base_main_flat_atk: i32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Hands
    {
        return Hands
        {
            relic_type: RelicType::HANDS,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk: 0.0,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_body_with_percent_hp(name: String, rarity: i32, base_main_percent_hp: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32,
        base_sub_flat_def: i32, base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32,
        base_sub_crit_rate: f32, base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Body
    {
        return Body
        {
            relic_type: RelicType::BODY,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp,
            base_main_percent_atk: 0.0,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_body_with_percent_atk(name: String, rarity: i32, base_main_percent_atk: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32,
        base_sub_flat_def: i32, base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32,
        base_sub_crit_rate: f32, base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Body
    {
        return Body
        {
            relic_type: RelicType::BODY,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_body_with_percent_def(name: String, rarity: i32, base_main_percent_def: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32,
        base_sub_flat_def: i32, base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32,
        base_sub_crit_rate: f32, base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Body
    {
        return Body
        {
            relic_type: RelicType::BODY,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk: 0.0,
            base_main_percent_def,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_body_with_crit_rate(name: String, rarity: i32, base_main_crit_rate: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32,
        base_sub_flat_def: i32, base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32,
        base_sub_crit_rate: f32, base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Body
    {
        return Body
        {
            relic_type: RelicType::BODY,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk: 0.0,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_body_with_crit_dmg(name: String, rarity: i32, base_main_crit_dmg: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32,
        base_sub_flat_def: i32, base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32,
        base_sub_crit_rate: f32, base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Body
    {
        return Body
        {
            relic_type: RelicType::BODY,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk: 0.0,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_body_with_outgoing_healing_boost(name: String, rarity: i32, base_main_outgoing_healing_boost: f32, base_sub_flat_hp: i32,
        base_sub_flat_atk: i32, base_sub_flat_def: i32, base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32,
        base_sub_percent_def: f32, base_sub_crit_rate: f32, base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Body
    {
        return Body
        {
            relic_type: RelicType::BODY,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk: 0.0,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_body_with_effect_hit_rate(name: String, rarity: i32, base_main_effect_hit_rate: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32, base_sub_flat_speed: i32, base_sub_percent_hp: f32,
        base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32, base_sub_crit_dmg: f32, base_sub_effect_res: f32,
        base_sub_break_effect: f32) -> Body
    {
        return Body
        {
            relic_type: RelicType::BODY,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk: 0.0,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_feet_with_percent_hp(name: String, rarity: i32, base_main_percent_hp: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Feet
    {
        return Feet
        {
            relic_type: RelicType::FEET,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp,
            base_main_percent_atk: 0.0,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_feet_with_percent_atk(name: String, rarity: i32, base_main_percent_atk: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Feet
    {
        return Feet
        {
            relic_type: RelicType::FEET,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_feet_with_percent_def(name: String, rarity: i32, base_main_percent_def: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Feet
    {
        return Feet
        {
            relic_type: RelicType::FEET,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk: 0.0,
            base_main_percent_def,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_feet_with_break_effect(name: String, rarity: i32, base_main_break_effect: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Feet
    {
        return Feet
        {
            relic_type: RelicType::FEET,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed: 0,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk: 0.0,
            base_main_percent_def: 0.0,
            base_main_break_effect,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_feet_with_flat_speed(name: String, rarity: i32, base_main_flat_speed: i32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> Feet
    {
        return Feet
        {
            relic_type: RelicType::FEET,
            name,
            rarity,
            base_main_flat_hp: 0,
            base_main_flat_atk: 0,
            base_main_crit_rate: 0.0,
            base_main_crit_dmg: 0.0,
            base_main_outgoing_healing_boost: 0.0,
            base_main_effect_hit_rate: 0.0,
            base_main_flat_speed,
            base_main_percent_path_dmg_boost: 0.0,
            base_main_energy_regen_rate: 0.0,
            base_main_percent_hp: 0.0,
            base_main_percent_atk: 0.0,
            base_main_percent_def: 0.0,
            base_main_break_effect: 0.0,
            base_sub_flat_hp,
            base_sub_flat_atk,
            base_sub_flat_def,
            base_sub_flat_speed,
            base_sub_percent_hp,
            base_sub_percent_atk,
            base_sub_percent_def,
            base_sub_crit_rate,
            base_sub_crit_dmg,
            base_sub_effect_res,
            base_sub_break_effect
        };
    }
    
    fn new_planar_sphere_with_percent_hp(name: String, rarity: i32, base_main_percent_hp: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> PlanarSphere
    {
    }
    
    fn new_planar_sphere_with_percent_atk(name: String, rarity: i32, base_main_percent_atk: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> PlanarSphere
    {
    }
    
    fn new_planar_sphere_with_percent_def(name: String, rarity: i32, base_main_percent_def: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> PlanarSphere
    {
    }
    
    fn new_planar_sphere_with_percent_path_dmg_boost(name: String, rarity: i32, base_main_percent_path_dmg_boost: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> PlanarSphere
    {
    }
    
    fn new_link_rope_with_percent_hp(name: String, rarity: i32, base_main_percent_hp: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> LinkRope
    {
    }
    
    fn new_link_rope_with_percent_atk(name: String, rarity: i32, base_main_percent_atk: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> LinkRope
    {
    }
    
    fn new_link_rope_with_percent_def(name: String, rarity: i32, base_main_percent_def: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> LinkRope
    {
    }
    
    fn new_link_rope_with_break_effect(name: String, rarity: i32, base_main_break_effect: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> LinkRope
    {
    }
    
    fn new_link_rope_with_energy_regen_rate(name: String, rarity: i32, base_main_energy_regen_rate: f32, base_sub_flat_hp: i32, base_sub_flat_atk: i32, base_sub_flat_def: i32,
        base_sub_flat_speed: i32, base_sub_percent_hp: f32, base_sub_percent_atk: f32, base_sub_percent_def: f32, base_sub_crit_rate: f32,
        base_sub_crit_dmg: f32, base_sub_effect_res: f32, base_sub_break_effect: f32) -> LinkRope
    {
    }
}
