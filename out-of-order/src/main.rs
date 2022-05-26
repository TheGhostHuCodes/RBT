fn main() {
    let mut floats = vec![3.1, 1.2, 4.5, 0.3];

    // Won't compile because f64 does not implement Ord. That's because f64 can
    // also represent NaN, and Infinity. Neither of those sort well.
    // floats.sort();

    float_sort(&mut floats);

    println!("{:#?}", floats);
}

fn float_sort<T: PartialOrd>(data: &mut [T]) {
    use std::cmp::Ordering::Less;
    data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
}
