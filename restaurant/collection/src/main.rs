mod study_str;

pub use crate::study_str::create_sample;

fn main() {
    let v1: Vec<i32> = Vec::new();  // 空のベクタ
    let mut v2 = vec![1, 2, 3, 4, 5];   // 初期値のあるベクタ

    /*
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    */

    //let third: &i32 = &v1[2];
    //let third: Option<&i32> = v1.get(2);

    // let does_not_exist = &v[100];    // プログラムはクラッシュする。
    // let does_not_exist = v.get(100); // プログラムはクラッシュせずNoneが返ってくる。

    // 一見動きそうだが、コンパイルエラーとなる。
    // ベクタにpushすると、メモリを新規確保して古い要素を新しい領域にコピーする可能性がある。
    // その場合、最初の要素を指す参照は、解放済メモリを指してしまう。
    /*
    let first = &v2[0];
    v2.push(6);
    println!("The first element is: {}", first);
    */

    // 拡張forを用いたベクタの出力
    for i in &v2 {
        println!("{}", i);
    }
    for i in &mut v2 {
        *i += 50;
        println!("{}", i);
    }

    // enumの列挙子は全てenum型となる。
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // そのため、1つのベクタの中でも異なる型の要素を扱える。
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    create_sample::create();
    create_sample::link();
    create_sample::access();
}
