fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    // sはStringへの参照
    s.len() // 所有権を受け取らないので、引数の値を返す必要がない
} // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので
  // 何も起こらない

// 参照は所有権を借用する

// fn change(some_string: &String) {
//     // 参照しているものを変更しようとするとエラーになる
//     // some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn error() {
    let mut s = String::from("hello");

    // 特定のスコープで、特定のデータに対しては一つしか可変な参照を持てない
    let r1 = &mut s;
    // let r2 = &mut s;

    // print!("{} {}", r1, r2);
}

fn not_error() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1はここでスコープを抜けるので、問題ない

    let r2 = &mut s;
}

fn error2() {
    let mut s = String::from("hello");

    let r1 = &s; // 問題ない
    let r2 = &s; // 問題ない
                 // let r3 = &mut s; // 問題あり
                 // println!("{}, {}, and {}", r1, r2, r3);
}

// fn dangle() -> &String {
//     // dangleはStringへの参照を返す
//     let s = String::from("hello");

//     &s
// } // sはスコープを抜けるので、メモリが解放（ドロップ）される
//   // その後、参照を返そうとするが、参照先のデータはもうない

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
