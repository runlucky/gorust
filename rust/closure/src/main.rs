fn main() {
    let list = vec![1, 2, 3];

    println!("クロージャの定義前: {:?}", list);

    // 別スレッドに所有権をmoveする
    // moveしないと、元スレッドが破棄された場合に不正な参照になってしまう。
    std::thread::spawn(move || println!("スレッドから: {:?}", list))
        .join()
        .unwrap();

    // listの所有権はmoveしたためもう参照できない
    // println!("クロージャの呼び出し後: {:?}", list);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    /// userが色を指定した場合をそれを、そうでない場合は最も在庫が多い色を返す
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    /// 最も在庫が多い色を返す
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1
            }
        }

        if num_blue < num_red {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 最も在庫が多い色を返すこと() {
        let inventory = Inventory { shirts: vec![
            ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue
        ]};

        let color = inventory.most_stocked();

        assert_eq!(ShirtColor::Red, color);
    }

    #[test]
    fn ユーザが指定した色を返すこと() {
        let inventory = Inventory { shirts: vec![
            ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue
        ]};

        let color = inventory.giveaway(Some(ShirtColor::Blue));

        assert_eq!(ShirtColor::Blue, color);
    }


    #[test]
    fn ユーザが好みを指定していない場合は在庫が多い色を返すこと() {
        let inventory = Inventory { shirts: vec![
            ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue
        ]};

        let color = inventory.giveaway(None);

        assert_eq!(ShirtColor::Red, color);
    }
}
