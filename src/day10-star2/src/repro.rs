// question: how can I call this function with an arbitrary number of Vec?
// https://github.com/NattapongSiri/permutator/blob/c192343f507182fc4634e8ea6695f781a5688f16/src/lib.rs#L396
//

// to run the code outside of a sandbox, uncomment the next line

pub fn mock_cartesian_product<'a, T, F>(sets: &'a [&[T]], mut cb: F)
where
    T: 'a,
for<'r> F: FnMut(&'r [&'a T]) + 'a,
{
    permutator::cartesian_product(sets, cb);
}

fn four_products() {
    mock_cartesian_product(&[&[1, 2], &[3, 4]], |p| println!("four products: {:?}", p));

    // four products: [1, 3]
    // four products: [1, 4]
    // four products: [2, 3]
    // four products: [2, 4]
}

fn four_products_take_2() {
    let mut v = Vec::new();

    v.push(vec![1, 2]);
    v.push(vec![3, 4]);

    mock_cartesian_product(&[v.first().unwrap(), v.last().unwrap()], |p| {
        println!("fpt2 {:?}", p);
    });

    // fpt2 [1, 3]
    // fpt2 [1, 4]
    // fpt2 [2, 3]
    // fpt2 [2, 4]
}

fn two_products() {
    let mut v = Vec::new();

    v.push(vec![1, 2]);
    v.push(vec![3, 4]);

    mock_cartesian_product(&[&v], |p| println!("two products: {:?}", p));

    // two products: [[1, 2]]
    // two products: [[3, 4]]
}

// = note: expected reference `&[&[_]]`
//            found reference `&[[{integer}; 2]]`
fn now_it_works_ok_fine(v: &[&[u32]]) {
    mock_cartesian_product(v, |product| {
        println!("does not compile {:?}", product);
    });
}

pub fn run_examples() {
    // compiles, not dynamic enough
    four_products();
    four_products_take_2();

    // dynamic but unwanted output
    two_products();

    // does_not_compile();
    let mut v = Vec::new();
    let one_two = vec![1,2];
    let three_four = vec![3,4];
    let five_six = vec![5,6];
    v.push(&one_two[..]);
    v.push(&three_four[..]);
    v.push(&five_six[..]);
    now_it_works_ok_fine(&v);
}
