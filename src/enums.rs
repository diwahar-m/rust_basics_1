pub fn main() {
    // enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        // enums has methods
        fn method_1(&self) {
            //
        }
    }

    let message_1 = Message::Write(String::from("Hello"));
    message_1.method_1();
}
