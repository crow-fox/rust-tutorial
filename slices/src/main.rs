fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // wordの中身は値5になる

    s.clear(); // Stringを空にする

    println!("The first word is: {}", word); // wordは5のまま

    // wordはまだ値5を保持しているが、もうこの値は意味を持たない
    // wordはsが空になったことを知らない

    let mut s = String::from("hello world");

    let word = first_word_with_slice(&s); // wordの中身は値5になる

    // s.clear(); // 不変参照を返しているため、コンパイルエラーになる

    println!("The first word is: {}", word); // wordは空文字列になる
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
