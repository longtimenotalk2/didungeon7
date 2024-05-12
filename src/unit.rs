mod display;

use miniserde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
enum Team {
    Friend,
    Enemy,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Unit {
    // 基础
    name : String,
    team : Team,

    // 基础属性
    hp_max : i32,
    
    // 状态
    hp : i32,
    stand : bool,
    bound_arm : i32,
    bound_leg : i32,
    bound_lock : bool,
}

// impl Unit {
//     fn new(
//         name : String, 
//         is_friend : bool, 
//         hp_max : i32
//     ) -> Self {
//         Self {
//             name,
//             team : if is_friend { Team::Friend } else { Team::Enemy },
//             hp_max,
//             hp : hp_max,
//         }
//     }
// }