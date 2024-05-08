mod display;

use crate::Id;

enum Team {
    Friend,
    Enemy,
}

pub struct Unit {
    // 基础
    id : Id,
    name : String,
    team : Team,

    // 基础属性
    hp_max : i32,
    
    // 状态
    hp : i32,
}