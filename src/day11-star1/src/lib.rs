pub fn run() {
    let first_round = lucio::day::eleven::parse_input_data().unwrap();
    println!("{}", lucio::day::eleven::roll_tape(&first_round));
}
