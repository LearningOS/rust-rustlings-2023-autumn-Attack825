// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut x = 100;
    // let y = &mut x;
    // let z = &mut x;
    // *y += 100;
    // *z += 1000;
    // 不对，这里的y和z是可变引用，所以不能同时存在，需要将y和z的作用域分开，通过大括号来分开，比如下面这样
    // 大括号作用域内的变量，会在大括号结束后被释放，所以可以通过这种方式来分开y和z的作用域
    {
        let y = &mut x;
        *y += 100;
    }
    {
        let z = &mut x;
        *z += 1000;
    }
    assert_eq!(x, 1200);
}
