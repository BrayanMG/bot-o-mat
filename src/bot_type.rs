
#[derive(Debug, Clone, Copy, PartialEq)]

pub enum BotType {
    Unipedal,
    Bipedal,
    Quadrupedal,
    Arachnid,
    Radial,
    Aeronautical
}


impl BotType {
    /// Getter for the string value of the enum RobotType
    ///
    /// Return 
    ///
    /// The string value for the enum 
    pub fn get_value(&self) -> &str {
        match self {
            BotType::Unipedal => "Unipedal",
            BotType::Bipedal => "Bipedal",
            BotType::Quadrupedal => "Quadrupedal",
            BotType::Arachnid => "Arachnid",
            BotType::Radial => "Radial",
            BotType::Aeronautical => "Aeronautical"
        }
    }
}

pub fn from_str(val: &str) -> BotType {
    match val {
        "Unipedal" => BotType::Unipedal,
        "Bipedal" => BotType::Bipedal,
        "Quadrupedal" => BotType::Quadrupedal,
        "Arachnid" => BotType::Arachnid,
        "Radial" => BotType::Radial,
        "Aeronautical" => BotType::Aeronautical,
        _ => {
            println!("\nEnter a valid type!\n");
            BotType::Aeronautical
        }
    }
}
