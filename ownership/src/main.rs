fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    print!("{}", s);

    {
        let _y = String::from("hello");
        // _y で作業する
    }
    // _y はここでスコープを抜け、解放される

    let x = 5; // 5をxに束縛する スタックに保存
    let _y = x; // xの値をコピーして_yに束縛 スタックに保存

    let s1 = String::from("hello");
    let _s2 = s1; // s1のポインタ、長さ、容量が_s2にコピーされる
                  // 二重解放エラーの対策としてs1はムーズされるので、これ以降は使えない

    // println!("{}, world!", s1); // s1はもう有効ではない

    let s1 = String::from("hello");
    let s2 = s1.clone(); // clone()を使ってデータをdeepコピーする
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
                   // 大丈夫
    print!("{}", x);

    // println!("{}", s); // sの値は関数にムーズされ有効ではない

    let s1 = gives_ownership(); // gives_ownershipは戻り値をs1にムーブする
    print!("{}", s1);

    let s2 = String::from("hello"); // s2がスコープに入る
    let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ
                                       // 戻り値もs3にムーブされる
    print!("{}", s3);
    // println!("{}", s2); // s2はムーブされているので有効ではない

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    // タプル型で元のStringが返されるのでその後も使える
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る
    println!("{}", some_string);
    // ここでsome_stringがスコープを抜け、解放される
}

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
    // ここでsome_integerがスコープを抜けるが、何も特別なことはない
}

fn gives_ownership() -> String {
    // 戻り値を呼び出し元にムーブする
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_stringを呼び出し元にムーブする
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len(); // len()メソッドはStringの長さを返す
    (s, length)
    // もしsを返さなかった場合、呼び出した関数以降でsは使えなくなる
}
