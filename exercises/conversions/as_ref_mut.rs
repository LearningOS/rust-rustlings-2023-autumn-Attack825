// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// AsRef是一个trait，它允许类型转换为引用，而不会产生任何运行时开销。这对于在函数中接受引用的函数非常有用，但是您想要接受不同类型的引用。
// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T, U>(arg: &mut T)
// TODO: Implement the function body.
where
    T: AsMut<U>,
    U: std::ops::Mul<Output = U> + Copy,
{
    let mut value = arg.as_mut();
    *value = *value * *value;
}
// where是用来限制泛型的，可以限制泛型的类型，也可以限制泛型的关系，比如T: AsMut<U>，就是限制T必须实现AsMut<U>这个trait，U: std::ops::Mul<Output = U> + Copy，就是限制U必须实现std::ops::Mul<Output = U>和Copy这两个trait
// num_sq是一个函数，它接受一个泛型参数T，这个T必须实现AsMut<U>这个trait，然后在函数体中，我们调用了arg.as_mut()，这个as_mut()方法是AsMut这个trait的一个方法，所以T必须实现AsMut<U>这个trait，然后我们对arg.as_mut()的返回值进行了乘法运算，所以U必须实现std::ops::Mul<Output = U>这个trait，最后我们对arg.as_mut()的返回值进行了赋值，所以U必须实现Copy这个trait

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
