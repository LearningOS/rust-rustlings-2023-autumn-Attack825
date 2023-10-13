// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

// Option<T> 是 rust 类型安全重要思想的体现之一。它本质是一个 Enum 类型，有两个变体，Some(x) 和 None。
// 当表示没有值的时候，可以使用 None。其语义类似其他语言如 Python的None，Golang 的 nil，
// java 的null。但是又跟其他语言有本质的不同。rust 的 None 是 Option 类型。而不是 其他任何类型。
// 而其他语言的 None nil 可以是任何其他类型或引用类型。Null 可以是 字串，也可以是 指针。
// https://www.zhihu.com/question/389859557

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    // match是rust中的模式匹配语句，类似于其他语言的switch语句
    // match语句的语法是 match x { ... }，其中x是要匹配的值，...是匹配的模式
    // ref 是rust中的引用，类似于其他语言的指针，这里的ref p表示p是一个引用，用来匹配y
    // match 语句中的 Some(ref p) 使用了 ref 关键字。这里的 ref p 表示对 y 中的值进行引用绑定，而不是将整个 Point 结构体移动到 p 中。
    // 通过使用 ref 关键字，可以在 match 分支中访问 y 的内部成员，而无需将 Point 结构体所有权转移给 p。
    // 这对于需要在分支内部对 Point 进行读取操作而不修改它的情况很有用。
    // 在此代码示例中，如果 y 是 Some 类型，则会创建一个名为 p 的引用，它引用 y 中的 Point 结构体。
    // 然后，可以使用 p.x 和 p.y 访问 Point 的坐标，并将其打印出来。
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
