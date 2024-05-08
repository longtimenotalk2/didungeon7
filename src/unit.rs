mod display;

enum Team {
    Friend,
    Enemy,
}

pub struct Unit {
    // 基础
    name : String,
    team : Team,

    // 基础属性
    hp_max : i32,
    
    // 状态
    hp : i32,
}

impl Unit {
    fn new(
        name : String, 
        is_friend : bool, 
        hp_max : i32
    ) -> Self {
        Self {
            name,
            team : if is_friend { Team::Friend } else { Team::Enemy },
            hp_max,
            hp : hp_max,
        }
    }
}