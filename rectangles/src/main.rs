#[derive(Debug)]    // Debugトレイトの注釈をつけることで、デバッグ用整形機で出力可能になる
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
    println!("{}", rect1.height);
    println!("{:?}", rect1);    // デバッグ用整形機で出力
    println!("{:#?}", rect1);   // デバッグ用整形機で出力2

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels",
        rect1.area()    // メソッドの呼び出し
    );

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 関連関数の呼び出し
    let sq = Rectangle::square(3);
    println!("{}", sq.width);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    fn area(&self) -> u32 { // 関数ではなくメソッド
        self.width * self.height
    }
}

impl Rectangle {    // 複数のimplブロックはコンパイルが通る
    fn can_hold(&self, other: &Rectangle) -> bool { // メソッド
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle { // メソッドではなく関連関数
        Rectangle { width: size, height: size }
    }
}