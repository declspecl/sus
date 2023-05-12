pub mod paths;
pub mod relic;
pub mod character;

use paths::Paths;
use character::Character;

fn main()
{
    let asta: Character = Character::new(String::from("asta"), Paths::FIRE, 1, vec![], 0, 0, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    println!("{}", asta.to_string());
}
