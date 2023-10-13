// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

// 这是一个模块，它包含了一个函数和一个私有函数
mod sausage_factory {
    // 这是一个私有函数
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 私有函数和公有函数的不同在于，私有函数只能在模块内部使用，而公有函数可以在模块外部使用
    // 公有函数需要使用pub关键字来修饰
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
