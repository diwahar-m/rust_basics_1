extern crate rand;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world1!");

    // mutability
    let mut a = "5";
    println!("A is {}", a);
    a = "6";
    println!("A is {}", a);

    // constants
    const VALUE: i32 = 5;

    // shadowing
    // shadowing allows us to change the data type ;
    let x = 5;
    let x = x * 10;
    println!("Shadow value x is{}", x);

    // 4 primary scalr types - int, string, boolean, char
    // Integer
    // i -> signed = both positive and negative
    // u -> unsigned = positive
    // floating
    // char  - ''
    // string  - ""

    // Compund types - arrays & tuples
    let tuple = (500, 6.4, 1);
    // destructured tuple
    let (x, y, z) = tuple;
    // indexing in tuple
    let second_tuple = tuple.1;
    println!("Tuple destructured, {} {} {}", x, y, z);
    println!("indexing in tuple {}", second_tuple);

    // arrays - same data type, fixed length
    let array_one = [1, 2, 3];
    println!("First element in array {}", array_one[0]);

    // second_function(5, 6);
    // control_flow()
    // guess_num()
    ownership();
}

fn second_function(x: u32, y: u32) {
    println!("This is second function {}, {}", x, y);
    let number = five();
    println!("Returned function {}", number);
}

fn five() -> i32 {
    5 //  here expression is returned
    // placing semicolon here makes it as a statement & it will not return.
}

fn control_flow() {
    let num = 3;
    if num > 5 {
        // condition must be boolean
        println!("Num greater than 5")
    } else if num > 0 {
        println!("Num less than 5")
    } else {
        println!("Num is negative")
    }
    if_as_statement()
}

fn if_as_statement() {
    let condition = true;
    // if can be used as a statement
    let number = if condition { 5 } else { 2 };
    println!("Number: {}", number);
    loop_types();
}

fn loop_types() {
    // Loop types -> loop, while , for

    // run for infinite times
    // loop {
    //     print!("HI !")
    // }

    let mut number = 2;
    while number > 0 {
        println!("{}", number);
        number = number - 1;
    }
    for_loop();
}

fn for_loop() {
    for number in 1..4 {
        println!("For Loop-1, {}", number)
    }
    //
    for number in (1..4).rev() {
        println!("For Loop-2, {}", number)
    }
    //
    let array = [10, 20, 30];
    for element in array.iter() {
        println!("For Loop-3, {}", element)
    }
}

fn guess_num() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret num is {}", secret_number);
    println!("Start Guess game");

    loop {
        println!("Print the guess number: ");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess) // mutable reference
            .expect("Failed the guess!");
        // let guess: u32 = guess.trim().parse().expect("Please type num"); // will break if there is error;
        let guess: u32 = match guess.trim().parse() {
            // pattern matching
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("The guess num is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn ownership() {
    let s1 = String::from("Hello");
    // let s2 = s1 //  here ownership of s1 is went to s2.
    // returning values also transfer ownership.
    let s2 = s1.clone();
    println!("S1: {}", s1);

    // borrowing
    let val_1 = 33;
    let returned_value = transfering_ownership(val_1);
    println!("The value of Val_1 is: {}", returned_value);

    // referencing - immutable
    let val_2 = String::from("hello");
    let returned_value_2 = referencing(&val_2);
    println!("The value of Val_2 is: {}", returned_value_2);

    // data race (mutating value)- can be avoided by using scope;
    let mut val_3 = String::from("HI");
    {
        let mut_val_3 = &mut val_3;
    }
    let mut_val_4 = &mut val_3;

    // dangle reference
    // dangle();
    // slice type
    let s = String::from("Hello World!");
    slice_type(&s);

    // string slice
    let u = String::from("Hi");
    let u_len = u.len();
    let u_2 = &s[..u_len];
    let u_3 = &s[3..];
    let u_all = &s[..];

    // array slice
    let arr_1 = [1, 2, 3, 4, 5];
    let slice = &arr_1[0..4];
    println!("Array slice: {}", slice[0]);
}

fn transfering_ownership(val: u32) -> u32 {
    val
}
fn referencing(val: &String) -> &String {
    val
}

fn dangle() -> String {
    let s = String::from("ss");
    s // returning ownership here to avoid dangle reference here.
} // scope of s is dropped here.

fn slice_type(s: &String) -> &str {
    let bytes = s.as_bytes(); // converting to byte slice
    for (i, &item) in bytes.iter().enumerate() {
        // converting to tuple to get index
        if item == b' ' {
            // checks for a byte of space.
            return &s[..i];
        }
    }
    return &s[..];
}
// cargo build --release
