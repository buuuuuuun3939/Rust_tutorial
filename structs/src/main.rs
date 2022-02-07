struct User {   // 構造体
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct Color(i32, i32, i32);    // タプル構造体
struct Point(i32, i32, i32);
struct hoge();  // ユニット様構造体

fn main() {
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        username: String::from("anotheruser"),
        email: String::from("anotheremail2@example.com"),
        ..user1 // 省略記法で、sign_in_countとactiveはuser1と同じ値を設定
    };

    let black = Color(0, 0, 0); // タプル構造体への代入
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        username,   // 構造体のフィールド名と引数が同じ名前なので省略できる
        email,
        active: true,
        sign_in_count: 1,
    }
}