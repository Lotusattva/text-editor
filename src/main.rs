include!("counter.rs");

fn main() {
    let mut counter = Counter::new(0);
    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);
    println!("Counter value: {}", counter.val);
}
