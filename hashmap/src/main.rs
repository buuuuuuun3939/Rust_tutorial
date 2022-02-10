use std::collections::HashMap;

fn main() {
    // ハッシュマップの作成と挿入
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    // ハッシュマップの別の生成方法
    // タプルのベクタに対してcollectメソッドを使用する。
    // zipメソッドを使用してタプルのベクタを作成できる。
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // ハッシュマップに一度挿入されたキーと値の所有権は、ハッシュマップが所有する。
    // 値への参照をHashMapに挿入すると、値はハッシュマップにムーブされない。
    // しかしながら、当然HashMapが有効な間は参照値が有効である必要がある。
    let field_name  = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //println!("{}", field_name);   // 所有権が無いからエラー

    // ハッシュマップの出力
    for (key, value) in &scores1 {
        println!("{}: {}", key, value);
    }
    // ハッシュマップの更新
    // 同じキーで値を更新できる。
    scores1.insert(String::from("Blue"), 70);
    for (key, value) in &scores1 {
        println!("{}: {}", key, value);
    }

    // entryメソッドを使って、キーに値がなかった場合のみ値を挿入する。
    scores1.entry(String::from("Yellow")).or_insert(100);   // キーYellowは既に値が存在するので更新されない。
    scores1.entry(String::from("Red")).or_insert(100);      // キーRedは値が存在しないので新規に挿入される。
    println!("{:?}", scores1);

    // ハッシュマップのキーに単語を格納し、単語の出現回数をカウントする。
    let text = "hello world wonderful world";
    let mut map2 = HashMap::new();
    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);  // or_insert関数はキーに対する値への可変参照(&mut V)を返却する。
        *count += 1;    // 参照外し
    }
    println!("{:?}", map2);
}
