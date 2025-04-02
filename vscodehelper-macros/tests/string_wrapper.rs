use std::rc::Rc;

use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
struct MyStringWrapper {
    inner: Rc<str>,
}

#[test]
fn main() {
    let wrapper = MyStringWrapper::new("Hello, world!".to_string());

    // All the implementations are available:
    println!("{}", wrapper);  // Uses Display trait
    println!("{:?}", wrapper);  // Uses Debug trait

    let string_value: String = wrapper.to_string();
    let wrapper_from_str = "test".parse::<MyStringWrapper>().unwrap();
    println!("{}", string_value);
    println!("{:?}", wrapper_from_str);
}