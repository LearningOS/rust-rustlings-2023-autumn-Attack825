// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option = Some(12);
    // 这里我们需要使用if let来判断option是否为Some，否则会报错，因为我们不能直接使用Some来进行判断
    // 或者我们可以使用while let来进行判断
    // if let Some(x) = option {
    //     res += x;
    // }
    while let Some(x) = option {
        res += x;
    }
    // for x in option {
    //     res += x;
    // }
    println!("{}", res);
}
