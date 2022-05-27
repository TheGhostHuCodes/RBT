// The same function name can not be defined multiple times.
// fn double_it(n: i32) -> i32 {
//     n * 2
// }
//
// fn double_it(n: f32) -> f32 {
//     n * 2.0
// }

fn double_it<T>(n: T) -> T
where
    T: std::ops::Mul<Output = T> + From<i32>,
{
    n * 2.into()
}

fn main() {
    println!("2 * 4 = {}", double_it(4));
}
