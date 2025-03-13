use std::array;

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

    second_function(5, 6);
    control_flow()
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
    for number in (1..4) {
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
// cargo build --release
