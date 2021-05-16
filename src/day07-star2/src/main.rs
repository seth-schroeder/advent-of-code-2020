use parser::RawData;
use std::error::Error;

mod parser;
mod test_data;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = test_data::read_test_data("day07-star2/small.txt")?;
    let results = parser::parse(&lines);

    let dg = RawData::digraph(&results);
    // println!("{:#?}", dg);
    println!(
        "{:?}",
        dg[(&String::from("shiny gold"), &String::from("dark red"))]
    );
    let tg = RawData::trigraph(&results);
    // println!("{:#?}", tg);
    println!(
        "{:?}",
        tg[("shiny gold", "dark red")]
    );

    let mut g = RawData::graph(&results);
    // println!("{:#?}", g[&String::from("shiny gold")]);

    // let node = g.add_node(&String::from("yolo"));
    // println!("{}", node.0);

    Ok(())
}
