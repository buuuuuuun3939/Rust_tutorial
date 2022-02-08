mod front_of_house;

//use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;  // こちらは再公開

// 次の書き方だと、関数はadd_to_waitlist()だけで呼び出せる。
// しかしながら、hosting::が抜けることでどこに定義されたモジュールから呼び出しているのかが不明になる。
// よって、慣例的ではない。
// use crate::front_of_house::hosting::add_to_waitlist; 

// 一方で、構造体やenumその他の要素を名前空間で定義する場合には
// 次のようにフルパスで指定するのが慣例的である。
use std::collections::HashMap;

// 同じ名前のResult型を2つ名前空間に持ち込む場合には
// 親モジュールから指定する以外にも、別名をつけるのも良い。
use std::fmt::Result;
use std::io::Result as IoResult;

mod back_of_house {
    pub enum Appetizer { // enumのヴァリアントは全て公開される
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,      // 公開
        seasonal_fruit: String, // 非公開
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // 絶対パスでのモジュール内関数の呼び出し
    crate::front_of_house::hosting::add_to_waitlist();
    // 相対パスでのモジュール内関数の呼び出し
    front_of_house::hosting::add_to_waitlist();

    // 夏にライ麦パン朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // やっぱり別のパン
    meal.toast =String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // back_of_house::Breakfastのseasonal_fruitは非公開なので、書き込みできない。
    //meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;    
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}