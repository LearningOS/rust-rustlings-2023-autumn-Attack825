// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`

// 把上面的话翻译成中文就是：
// 递归类型在编译时，Rust需要知道这个类型占用多少空间，这对于递归类型来说是个问题
// 因为递归类型的值可以包含另一个相同类型的值，为了解决这个问题，我们可以使用Box
// Box是一个智能指针，用来在堆上存储数据，同时也允许我们包装一个递归类型
// 我们在这个练习中要实现的递归类型是cons list，这是一个在函数式编程语言中经常出现的数据结构
// cons list中的每个元素包含两个元素：当前元素的值和下一个元素，最后一个元素是一个叫做Nil的值
// 第一步：在enum定义中使用Box来使代码编译通过
// 第二步：通过替换todo!()来创建空的和非空的cons list

//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

// 创建一个空的cons list
pub fn create_empty_list() -> List {
    List::Nil
}

// 创建一个非空的cons list
pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
