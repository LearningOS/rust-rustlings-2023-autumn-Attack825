// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

// trait是一种行为，可以对任何类型实现trait，trait可以让我们定义一些方法签名，
// 这些方法签名可以用于不同的类型，trait可以让我们以一种抽象的方式定义共享的行为。
trait AppendBar {
    fn append_bar(self) -> Self;
}

// impl是实现，impl后面跟的是trait，这里是AppendBar，然后是for，后面跟的是类型，这里是String
impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    //  这里的self是String类型，表示调用者，这里是s
    fn append_bar(self) -> Self {
        let mut s = self;
        s.push_str("Bar");
        s
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
