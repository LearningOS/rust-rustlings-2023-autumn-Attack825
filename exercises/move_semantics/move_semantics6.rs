// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(&data);
}

// Should not take ownership
// 用来获取字符串的最后一个字符
fn get_char(data: String) -> char {
    // 这里需要使用clone，因为chars()返回的是一个迭代器，而迭代器是一个消耗性的迭代器，所以需要使用clone
    data.chars().last().unwrap()
    // unwrap是一个宏，用来获取Option中的值，如果Option中没有值，则会panic
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    // 不需要to_uppercase，直接打印data
    // 解引用是指将引用转换为值，这里的data是一个引用，所以需要解引用，但是解引用是不可变的，所以需要使用mut来修饰

    println!("{}", data);
}
