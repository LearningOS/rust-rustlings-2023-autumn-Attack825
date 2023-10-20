// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
// Default trait是一个trait，它允许我们为类型提供一个默认值
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation in order for the line `let p =
// Person::from("Mark,20")` to compile Please note that you'll need to parse the
// age component into a `usize` with something like `"4".parse::<usize>()`. The
// outcome of this needs to be handled appropriately.
// 你的任务是完成这个实现，以便行`let p = Person::from("Mark,20")`编译
// 请注意，您需要使用类似`"4".parse::<usize>()`的东西将年龄组件解析为`usize`。
// 这个结果需要适当地处理。
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of
//    Person.
// 2. Split the given string on the commas present in it.
// 3. Extract the first element from the split operation and use it as the name.
// 4. If the name is empty, then return the default of Person.
// 5. Extract the other element from the split operation and parse it into a
//    `usize` as the age.
// If while parsing the age, something goes wrong, then return the default of
// Person Otherwise, then return an instantiated Person object with the results
// 1. 如果提供的字符串的长度为0，则返回Person的默认值。
// 2. 在其中存在的逗号上分割给定的字符串。
// 3. 从分割操作中提取第一个元素并将其用作名称。
// 4. 如果名称为空，则返回Person的默认值。
// 5. 从分割操作中提取其他元素并将其解析为`usize`作为年龄。
// 如果在解析年龄时出现问题，则返回Person的默认值
// 否则，返回一个实例化的Person对象与结果

// From trait是用于值到值转换的。如果From为类型实现正确，那么Into trait应该相反。
// impl From<&str> for Person 是指实现From trait，将&str类型转换为Person类型
impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.len() == 0 {
            return Person::default();
        }

        let tmp: Vec<&str> = s.split(',').collect();
        //s.split(',')是将字符串s按照','分割成多个字符串，然后collect()将多个字符串收集到一个Vec中
        if tmp.len() < 2 {
            return Person::default();
        }
        if tmp[0].len() == 0 {
            return Person::default();
        }
        if tmp[1].len() == 0 {
            return Person::default();
        }
        if tmp.len() > 2 {
            return Person::default();
        }
        let name = String::from(tmp[0]);
        if let Ok(age) = tmp[1].parse::<usize>() {
            if name.len() == 0 {
                return Person::default();
            }
            return Person {
                name: name,
                age: age,
            };
        }
        return Person::default();
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
