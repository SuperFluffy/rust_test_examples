use super::buf_to_string;
#[test]
fn hello_world_bytes_are_utf8() {
    let hello_world = buf_to_string(b"hello world").unwrap();
    assert_eq!("hello world", hello_world);
}
