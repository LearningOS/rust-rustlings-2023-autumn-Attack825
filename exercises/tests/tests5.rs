// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.


/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    unsafe {
        let p = address as *mut u32;
        // 用来将address转换为一个指向u32类型的可变指针的，
        // 然后我们对这个指针进行了解引用，然后对它进行了赋值，所以这个指针必须指向一个合法的u32类型的值，
        // 所以我们需要将address转换为一个指向u32类型的可变指针
        // *p表示对指针p进行解引用，然后对它进行赋值
        *p = 0xAABBCCDD;
    }
}
// unsafe是一个关键字，它可以用来标记一个函数或者一个代码块，
// unsafe的作用是告诉编译器，这里的代码是不安全的，需要程序员自己来保证它的安全性

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        // 上一句是用来调用modify_by_address函数的，这个函数接受一个usize类型的参数，
        // 然后在函数体中，我们对这个参数进行了解引用，然后对它进行了赋值，
        // 所以这个参数必须是一个合法的地址，而且这个地址必须包含一个u32类型的值，
        // 所以我们需要将t的地址转换为usize类型，然后传递给modify_by_address函数
        assert!(t == 0xAABBCCDD);
    }
}
