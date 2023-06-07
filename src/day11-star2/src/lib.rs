pub fn run() {
    let data = lucio::get_input_data(11).unwrap();
    let first_round = lucio::day::eleven::parse_input_data(&data).unwrap();
    println!("{}", lucio::day::eleven::roll_tape2(&first_round));
}
