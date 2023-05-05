static PORT: u16 = 3344;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    boltserver::start(args, PORT);
}
