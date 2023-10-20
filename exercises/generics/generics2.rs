// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// 这个结构体用于存储任意类型的值
struct Wrapper<T> {
    value: T,
}

// 需要两个<T>，Wrapper<T>是一个泛型结构体。impl<T>表示对泛型结构体Wrapper<T>进行实现
impl<T> Wrapper<T> {
    // 这个函数用于创建一个Wrapper<T>的实例
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
