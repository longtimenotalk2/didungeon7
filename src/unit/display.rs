use std::fmt::Display;

use colorful::{Color, Colorful};

use super::{Team, Unit};


impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        // 姿态
        s += if self.stand {
            "!"
        } else {
            if self.bound_arm > 0 {
                "≈"
            } else {
                " "
            }
        };
        s += if self.stand && self.bound_arm > 0{ 
            "≈"
        } else if !self.stand && self.bound_leg > 0 {
            "≈"
        } else {
            " "
        };
        // 名字
        let color_name = if self.bound_lock {
            None
        } else {
            if self.stand {
                match self.team {
                    Team::Friend => Some(Color::Blue),
                    Team::Enemy => Some(Color::Red),
                }
            } else {
                match self.team {
                    Team::Friend => Some(Color::DarkBlue),
                    Team::Enemy => Some(Color::DarkRed1),
                }
            }
        };
        let mut name = if let Some(color) = color_name {
            self.name.clone().color(color).to_string()
        } else {
            self.name.clone()
        };
        if self.bound_lock {
            name = delete_line(&name);
        }
        s += &name;
        s += " ";
        // 束缚情况
        s += & if self.bound_lock {
            "锁 ".to_string()
        } else {
            if self.bound_arm > 0 {
                if self.bound_leg > 0 {
                    format!("{}+{}", self.bound_arm, self.bound_leg)
                } else {
                    format!("臂{}", self.bound_arm)
                }
            } else {
                if self.bound_leg > 0 {
                    format!("腿{}", self.bound_leg)
                } else {
                    "   ".to_string()
                }
            }
        };
        // 血条
        s += &hp_bar::<20, 50>(self.hp, self.hp_max);

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_unit() {
        let my_unit = Unit {
            name: "艾莉莎".to_string(),
            team: Team::Friend,
            hp_max: 500,
            hp: 80,
            stand : true,
            bound_arm : 4,
            bound_leg : 0,
            bound_lock : false,
        };
        println!("{}", my_unit);
    }

    #[test]
    fn test_delete_line() {
        println!("{}", delete_line(&"流萤".color(Color::Green).to_string()))
    }

    #[test]
    fn test() {
        for color in Color::iterator() {
            println!("{}", format!("{:?}", color).color(*color));
        }
    }
}

fn hp_bar<const RED : i32, const YELLOW : i32>(hp : i32, hp_max : i32) -> String {
    fn block(i : i32) -> &'static str {
      match i {
        ..=0 => " ",
        1 => "▏",
        2 => "▎",
        3 => "▍",
        4 => "▌",
        5 => "▋",
        6 => "▊",
        7 => "▉",
        8.. => "█",
      }
    }
  
    let rate = hp as f64 / hp_max as f64;
    let n = 4;
    let color = if rate <= RED as f64 / 100. {
      Color::Red
    } else if rate <= YELLOW as f64 / 100. {
      Color::Yellow
    } else {
      Color::Green
    };
    let mut txt = String::new();
    txt += "▕";
    let q = (n * 8) as f64;
    for i in 0..n {
      txt += &block((rate * q - i as f64 * 8.) as i32).color(color).to_string()
    }
    txt += "▏";
    txt
}

fn delete_line(s : &str) -> String {
    format!("\x1B[9m{s}\x1B[0m")
}