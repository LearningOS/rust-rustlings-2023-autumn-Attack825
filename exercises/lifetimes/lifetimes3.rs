// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// struct Book {
//     author: &str,
//     title: &str,
// }

// 将Book的生命周期参数和author、title的生命周期参数绑定在一起，
// 这样就可以保证author和title的生命周期不会比Book的生命周期长，就不会出现悬垂指针
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
