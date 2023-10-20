// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        // None=>String::new(),是一个表达式，所以不能用分号结尾，用于match的分支
        None => String::new(),
        // 如果c.next()返回的是Some(first)，则将first转换为大写，再加上剩下的字符
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // words.iter()返回一个迭代器，map()方法将迭代器中的每一个元素都应用到capitalize_first()函数上
    words
        .iter()
        .map(|word| capitalize_first(word))
        .collect::<Vec<String>>();
    let mut result = Vec::new();
    for word in words {
        result.push(capitalize_first(word));
    }
    result
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words
        .iter()
        .map(|word| capitalize_first(word))
        .collect::<String>();
    let mut result = String::new();
    for word in words {
        result.push_str(&capitalize_first(word));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
