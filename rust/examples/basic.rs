use stalin_sort::stalin_sort;

fn main() {
    let input = [3, 1, 4, 2, 5, 0, 2, 6];
    println!("input: {input:?}");
    println!("Stalin sort: {:?}", stalin_sort(&input));
}
