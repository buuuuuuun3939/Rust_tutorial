fn main() {
    /*
    loop {
        println!("again!"); // 無限ループ
    }
    */
    
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
    for element in a.iter() {       // 拡張for
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {    // range
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}
