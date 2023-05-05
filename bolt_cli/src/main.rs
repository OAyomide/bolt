static PORT: u16 = 3344;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    bolt_core::start(args, PORT);
}
