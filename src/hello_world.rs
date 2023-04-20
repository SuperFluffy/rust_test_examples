pub struct HelloWorld;

impl HelloWorld {
    pub fn say_hello_happy() -> &'static str {
        "hello world ( ˶ˆᗜˆ˵ )"
    }

    #[allow(dead_code)]
    fn say_hello_angry() -> &'static str {
        "HELLO WORLD! •`_´•"
    }
}

#[cfg(test)]
mod tests {
    use super::HelloWorld;
    #[test]
    fn hello_is_indeed_angry() {
        assert_eq!("HELLO WORLD! •`_´•", HelloWorld::say_hello_angry());
    }
}
