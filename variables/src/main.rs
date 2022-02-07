fn main() {
    //const MAX_POINTS: u32 = 100_000;

    let mut x = 5;  // mutable
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    let y = 6;      // immutable
    let y = y + 1;  // immutableだからシャドーイングが可能
    let y = y * 2;
    println!("The value of y is: {}", y);


    let tup: (i32, f64, u8) = (500, 6.4, 1);    // タプル型
    let (tup_x, tup_y, tup_z) = tup;            // パターンマッチングでタプルを分解
    println!("The value of tup_y: {}", tup_y);
    let five_hundred   = tup.0; // 添字でタプルの各要素に直接アクセス
    let six_point_four = tup.1;
    let one = tup.2;


    let a = [1, 2, 3, 4, 5];    // 配列
    let first  = a[0];
    let second = a[1];


    another_function(5, 6);
    println!("The value of returned_x is: {}", five());
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", y);
}

fn five() -> i32 {
    5   // 型がi32で返却値は5
}
