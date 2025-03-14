pub fn main() {
    // struct data type
    struct User {
        name: String,
        email: String,
        active: bool,
        sign_in_count: u64,
    }

    // order not needed
    let user_1 = User {
        email: String::from("johnde@gmail.com"),
        name: String::from("johndoe"),
        active: true,
        sign_in_count: 3,
    };

    fn create_user(email: String, name: String) -> User {
        User {
            email,
            name,
            active: true,
            sign_in_count: 3,
        }
    };

    let user_2 = create_user(String::from("johnde@gmail.com"), String::from("johndoe"));

    let user_3 = User {
        email: String::from("johnde@gmail.com"),
        name: String::from("johndoe"),
        ..user_1 // using previous instance
    };

    // Tuple structs
    tuple_structs();
    //
    println!("The area of rectangle is {}", calc_rect((2, 5)));

    // method
    method();

    // asociated_functions
    asso_func();
}

fn tuple_structs() {
    #[derive(Debug)] // to print the struct
    struct Color(i32, i32, i32);

    let black = Color(9, 78, -90);
    println!("Black: {:#?}", black); // :? to print the struct
}

fn calc_rect(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn method() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // implementation block
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    println!("The area of rect is {}", rect1.area());
}

fn asso_func() {
    #[derive(Debug)] // to print the struct
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            //  not related with instance of struct.
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let sq_1 = Rectangle::square(32);
    println!(" The square is {:#?}", sq_1);
}
