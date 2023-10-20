// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.
// Type casting (类型转换)是通过使用as操作符来完成的。请注意，as操作符不仅用于类型转换。它还有助于重命名导入。
// 目标是确保除法不会失败编译并返回正确的类型。

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // total是f64类型，values.len()是usize类型，我们需要将usize类型转换为f64类型
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
