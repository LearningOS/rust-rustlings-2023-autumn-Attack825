// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

// 本练习探索了宏，宏是一种编写代码的方式，它可以在编译时生成代码
// 本练习的目的是让你知道如何定义一个宏
// 宏是通过macro_rules!关键字定义的，后面跟着宏的名称和宏的定义
// 宏（英语：Macro）是一种批量处理的称谓。
// 计算机科学里的宏是一种抽象（Abstraction），它根据一系列预定义的规则替换一定的文本模式。
// 解释器或编译器在遇到宏时会自动进行这一模式替换。
// 对于编译语言，宏展开在编译时发生，进行宏展开的工具常被称为宏展开器。

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro();
}
