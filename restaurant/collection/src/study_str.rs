pub mod create_sample {
    pub fn create() {
        // s1とs2は等価
        let mut s1 = String::new();
        let s2 = String::from("initial contents");

        let data = "initial contents";
        let s1 = data.to_string();
        println!("{}", s1);

        let mut s3 = String::from("foo");
        s3.push_str("bar"); // 文字列の追加
        println!("{}", s3);
        s3.push('!');       // 文字の追加
        println!("{}", s3); 
    }

    pub fn link() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;  // s1の所有権は連結時に呼び出されるaddメソッドで奪われる。
        //println!("{}", s1); //これはエラー

        let s4 = String::from("tic");
        let s5 = String::from("tac");
        let s6 = String::from("toe");
        let s7 = format!("{}-{}-{}", s4, s5, s6);   // formatマクロを使用すると、引数の所有権は奪われない。
        let s8 = s4 + "-" + &s5 + "-" + &s6;        // 一方、こちらでは"&"が第一引数以外の引数に必要となり、わかりにくい。
        println!("{}", s7);
        println!("{}", s8);
    }
    
    pub fn access() {
        let s = "𐅉hogefugahogefuga";    // 先頭の"𐅉"だけは4Byte文字。後続は1Byte文字。
        let hoge = &s[0..4];            // よって、"𐅉"だけが表示される。
        println!("{}", hoge);
        
        for b in s.bytes() {
            println!("{}", b);  // 文字列sの各文字のバイトコードを表示
        }
    }
}