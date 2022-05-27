const fn fib(n: u128) -> u128 {
    let mut a = 1;
    let mut b = 1;

    // for loops are not allowed in a `const fn`. This is because a for loop
    // uses a `Range`, but you can't use Generic bounds (other than Sized) on
    // parameters inside of a `const fn`.
    // for _ in 2..n {
    //     let tmp = a + b;
    //     a = b;
    //     b = tmp;
    // }
    let mut counter = 2;
    while counter < n {
        let tmp = a + b;
        a = b;
        b = tmp;
        counter += 1;
    }

    b
}

fn main() {
    for i in 0..5 {
        println!("Fib {} = {}", i, fib(i));
    }
}
