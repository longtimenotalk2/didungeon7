use std::fmt::Display;

use colorful::{Color, Colorful};

use super::{Team, Unit};


impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s += &format!("{}", self.name.clone().color(self.team.color()));
        s += "  ";
        s += &hp_bar::<20, 50>(self.hp, self.hp_max);

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let my_unit = Unit {
            name: "艾莉莎".to_string(),
            team: Team::Friend,
            hp_max: 500,
            hp: 80,
        };
        println!("{}", my_unit);
    }
}

impl Team {
    fn color(&self) -> colorful::Color {
        match self {
            Team::Friend => Color::Blue,
            Team::Enemy => Color::Red,
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