fn main() {
    // Variables are immutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants are useful
    const MAX_POINTS: u32 = 100_000;

    println!("max points: {}", MAX_POINTS);

    // Shadowing variable usage
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    // This shows how you would show that you kinda want the type conversion
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);
    /* you cannot do this
     * let mut spaces = "   ";
     * spaces = spaces.len();
     * since this breaks type immutability
     */

    // Gotta have types on stuff yo
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // float 32 and 64 are roughly the same speed on modern cpus
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // bool with and without type annotation
    let t = true; // without explicit type annotation

    let f: bool = false; // with explicit type annotation

    // tuples are cool.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // Rust arrays are cool
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let first = months[0];

    // let break_code = months[14];

    another_function();

    function2(5);

    println!("{}", five());

    println!("{}", plus_one(5));

    if sum < 20 {
        println!("heck, sum was small");
    } else {
        println!("super heck!, what the heck!");
    }


    // Assignment with condition
    let condition: bool = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("{}", number);


    loop { // loop loops forever until an explicit break statment
        break;
    }

    let mut counter: u32 = 3;
    while counter != 0 {
        counter = counter -1;
    }

    for month in months.iter() {
        println!("the month is {}", month)
    }

    for count in (1..4).rev() {
        println!("{}!", count);
    }
    println!("LIFTOFF!");

}

fn another_function() {
    println!("Another function");
}

fn function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    // functions evaluate to the last expression
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
