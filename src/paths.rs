pub enum Paths
{
    PHYSICAL,
    FIRE,
    ICE,
    LIGHTNING,
    WIND,
    QUANTUM,
    IMAGINARY
}

impl std::string::ToString for Paths
{
    fn to_string(&self) -> String
    {
        return match self
        {
            Self::PHYSICAL => String::from("Physical"),
            Self::FIRE => String::from("Physical"),
            Self::ICE => String::from("Physical"),
            Self::LIGHTNING => String::from("Physical"),
            Self::WIND => String::from("Physical"),
            Self::QUANTUM => String::from("Physical"),
            Self::IMAGINARY => String::from("Physical")
        };
    }
}
