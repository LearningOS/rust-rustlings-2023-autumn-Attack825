// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// #[test] 是Rust中的注解属性，用于编译环境的注解，类似 java 中的 annotation 和 C# 中的 attribute 。
// 通过#[test]注解，可以在运行时添加-test 参数进行单元测试。cargo test

// use std::fmt::Display;是引入Display trait，Display trait是一个trait，用于实现自定义类型的格式化输出。
use std::fmt::Display;

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

// 实现方法
impl<T: Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

// pub struct ReportCard {
//     pub grade: f32,
//     pub student_name: String,
//     pub student_age: u8,
// }

// 实现方法
// impl ReportCard {
//     pub fn print(&self) -> String {
//         format!(
//             "{} ({}) - achieved a grade of {}",
//             &self.student_name, &self.student_age, &self.grade
//         )
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+".to_string(), // 2.1, "A+"
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };

        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
