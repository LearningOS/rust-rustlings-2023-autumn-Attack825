// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // let nice_slice = a.split_at(1).1.split_at(3).0;
    // let nice_slice = &a[1..4];
    // 
    // split_at是分割数组的方法，返回一个元组，第一个元素是分割后的前半部分，第二个元素是分割后的后半部分

    assert_eq!([2, 3, 4], nice_slice)
}
