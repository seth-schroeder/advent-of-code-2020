use lucio::day::eleven::*;

pub fn run() {
    let data = lucio::get_input_data(11).unwrap();
    let first_round = seat::parse_input_data(&data).unwrap();
    let area = area::Area { max_neighbors: 4, scope: area::ScopeToScan::NextVisible };
    println!("{}", do_the_thing(&area, first_round));
}
