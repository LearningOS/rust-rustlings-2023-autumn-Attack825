// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

// 在这个练习中，我们有一个u32类型的Vec，名为"numbers"，它的值范围是0到99——[ 0, 1, 2, ..., 98, 99 ]
// 我们想要在8个不同的线程中同时使用这组数字
// 每个线程都会得到每8个值的和，有一个偏移量
// 第一个线程（偏移量为0）会计算0, 8, 16, ...的和
// 第二个线程（偏移量为1）会计算1, 9, 17, ...的和
// 第三个线程（偏移量为2）会计算2, 10, 18, ...的和
// ...
// 第八个线程（偏移量为7）会计算7, 15, 23, ...的和
//
// 因为我们使用了线程，所以我们的值需要是线程安全的，因此我们使用Arc
// 我们需要在两个TODO中填入值
// 在第一个TODO中填入`shared_numbers`的值
// 在第二个TODO中创建`child_numbers`的初始绑定
// 尽量不要创建`numbers`的副本

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    // numbers是一个包含100个元素的Vec，元素类型是u32，值范围是0到99
    let numbers: Vec<_> = (0..100u32).collect();
    // shared_numbers是一个Arc，它包含了numbers的所有权
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        // child_numbers用于存储shared_numbers的引用
        let child_numbers = Arc::clone(&shared_numbers);
        // joinhandles.push()函数用于将线程的句柄存储到joinhandles中
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
