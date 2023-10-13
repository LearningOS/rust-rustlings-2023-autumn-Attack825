// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let vec0 = Vec::new();

    // 这边不需要使用clone，因为fill_vec函数需要一个vector，而vec0是一个空的vector，所以可以直接传入
    let mut vec1 = fill_vec(vec0);
    // vec1是一个新的vector，所以可以直接push

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // 这里的vec是一个新的vector，所以可以直接push
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
