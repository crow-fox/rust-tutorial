fn main() {

    // スカラー型
    // 数値型
    let int: i32 = 1;
    println!("The value of int is: {}", int);
    let int: i32 = -1;
    println!("The value of int is: {}", int);
    let int: u32 = 1;
    println!("The value of int is: {}", int);

    // 浮動小数点型
    let float: f64 = 1.1;
    println!("The value of float is: {}", float);

    // 数値演算
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);
    let product = 4 * 30;
    print!("The value of product is: {}", product);
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);
    let floored = 2 / 3;
    println!("The value of floored is: {}", floored);
    let remainder = 43 % 5;
    print!("The value of remainder is: {}", remainder);

    // 論理値型
    let t = true;
    print!("The value of t is: {}", t);
    let f: bool = false;
    println!("The value of f is: {}", f);

    // 文字型
    let c = 'z';
    println!("The value of c is: {}", c);
    let z = 'ℤ';
    println!("The value of z is: {}", z);
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);


    // 複合型
    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    print!("Values of x, y, z are: {}, {}, {}", x, y, z);
    print!("Values of tup.0, tup.1, tup.2 are: {}, {}, {}", tup.0, tup.1, tup.2);

    // 配列型
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    print!("Values of a[0], a[1], a[2], a[3], a[4] are: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
    let b: [i32; 5] = [3; 5];
    print!("Values of b[0], b[1], b[2], b[3], b[4] are: {}, {}, {}, {}, {}", b[0], b[1], b[2], b[3], b[4]);
}
