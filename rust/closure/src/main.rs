mod inventory;
mod shirt_color;

use inventory::Inventory;
use shirt_color::ShirtColor;

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

    get_color();
}

fn get_color() {
    let color = Inventory { shirts: vec![] }
    .giveaway(None);

    println!("シャツの色: {:?}", color);

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
