use crate::shirt_color::ShirtColor;

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    /// userが色を指定した場合をそれを、そうでない場合は最も在庫が多い色を返す
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    /// 最も在庫が多い色を返す
    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_blue < num_red {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
