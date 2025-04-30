struct Counter {
    val: i32,
}

enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.val += 1,
            Message::Decrement => self.val -= 1,
        }
    }

    fn new(val: i32) -> Self {
        Self { val }
    }
}
