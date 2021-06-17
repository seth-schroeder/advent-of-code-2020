mod compute;
mod mask;

pub fn run() {
    let v: u64 = 2 << 35;
    println!("v = {:b}", v);
}
