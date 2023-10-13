// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

mod delicious_snacks {
    // TODO: Fix these use statements
    // use能够引入模块,需要pub为公共使用
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
    mod fruits {
        // 这里的const是常量，它的值是一个字符串字面量
        // 常量的类型必须显式指定
        // &'static str是一个字符串切片，它的生命周期是静态的
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
