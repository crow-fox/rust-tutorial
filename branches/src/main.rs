fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero"); // 数値は0以外の何かです
    }

    if number % 4 == 0 {
        println!("number is divisible by 4"); // 数値は4で割り切れます
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // 数値は3で割り切れます
    } else if number % 2 == 0 {
        println!("number is divisible by 2"); // 数値は2で割り切れます
    } else {
        println!("number is not divisible by 4, 3, or 2"); // 数値は4、3、2で割り切れません
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
