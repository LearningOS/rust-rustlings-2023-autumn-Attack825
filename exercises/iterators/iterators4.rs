// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use: (尽量不要使用以下方法)
    // - imperative style loops (for, while)
    // - additional variables (除了num之外，不要使用其他变量)
    // For an extra challenge, don't use: (额外的挑战，不要使用以下方法)
    // - recursion (递归)
    // Execute `rustlings hint iterators4` for hints.
    // 使用迭代器的方式来实现阶乘
    // 1. 生成一个从1到num的迭代器，包含num，使用collect()函数将迭代器转换成一个集合，类型是Vec<u64>
    // 2. 使用fold()函数来计算阶乘
    // 代码
    (1..=num).collect::<Vec<u64>>().into_iter().fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
