const MAX_POINTS: u32 = 100_000;

fn main() {
    print!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    {
        let x = x * 2;
        print!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    
}
