pub fn main() {
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
