use colorful::{Color, Colorful};

use crate::game701::board::unit::Team;

use super::Unit;

impl<'a> Unit<'a> {
    pub fn show(&self) {
        // active state
        let dft = || {
            if self.is_active() {
                print!("|");
            } else {
                print!(" ");
            }
        };

        if let Some(id) = self.board.id_now {
            if id == self.id {
                print!(">");
            }else{
                dft();
            }
        }else{
            dft();
        }
        
        
        // name
        print!("{}", self.colored_name());

        // hp bar
        print!("{}", hp_bar::<20, 50>(self.hp(), self.max_hp()));

        // hp state
        print!("{:>2}/{:>2}", self.hp(), self.max_hp());

        // spd
        print!(", {:>2}", self.spd());
    }
}

#[cfg(test)]
mod test {
    use crate::game701::board::{unit::UnitData, Board};

    #[test]
    fn test_show() {
        let mut board = Board::new(114514);
        let unit_data = UnitData::new_noal();
        board.add_unit(unit_data);
        let id_a = 0;

        macro_rules! unit_a {
            () => {
                board.unit(id_a)
            };
        }

        unit_a!().show();
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