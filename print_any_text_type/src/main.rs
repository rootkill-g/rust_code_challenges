fn info_as_ref<T: AsRef<str>>(a: &T) {
    println!("info_as_ref: {}", a.as_ref());
}

fn info_display<T: std::fmt::Display>(a: &T) {
    println!("info__display: {}", a);
}

fn main() {
    let a = String::from("Hello Ferris!");
    info_as_ref(&a);
    info_display(&a);

    let b = "Hello";
    info_as_ref(&b);
    info_display(&b);
}

#[test]
fn test_str_as_ref() {
    let a = "this is a str";
    info_as_ref(&a)
}

#[test]
fn test_string_as_ref() {
    let a = String::from("This is a String");
    info_as_ref(&a)
}

#[test]
fn test_str_display() {
    let a = "hey this is str";
    info_display(&a);
}

#[test]
fn test_string_display() {
    let a = String::from("hi this is string");
    info_display(&a);
}
