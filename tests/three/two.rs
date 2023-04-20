use rust_testing::HelloWorld;
#[test]
fn public_test() {
    assert_eq!("hello world ( ˶ˆᗜˆ˵ )", HelloWorld::say_hello_happy(),);
}
