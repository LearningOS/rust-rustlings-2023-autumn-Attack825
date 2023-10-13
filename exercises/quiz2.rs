// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    // super是父模块，这里的父模块是quiz2
    use super::Command;

    // TODO: Complete the function signature!
    // Vec<(String, Command)>表示一个元组的vector，元组的第一个元素是String，第二个元素是Command
    // Vec和vec!是什么关系？vec!是一个宏，用于创建一个vector
    // transformer是一个函数，接收一个Vec<(String, Command)>，返回一个Vec<String>
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            // 通过match匹配command，然后对string进行处理
            // 如果command是Uppercase，就将string转换为大写
            // 如果command是Trim，就将string去掉首尾的空格
            // 如果command是Append，就将string追加n个bar
            let mut s = string.clone();
            match command {
                Command::Uppercase => {
                    s = s.to_uppercase();
                }
                Command::Trim => {
                    s = s.trim().to_string();
                }
                Command::Append(n) => {
                    for _ in 0..*n {
                        s.push_str("bar");
                    }
                }
            }
            // 通过push将处理后的string添加到output中
            output.push(s);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
