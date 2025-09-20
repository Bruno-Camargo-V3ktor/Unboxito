use unboxit_derive::UnboxitSerialize;

#[test]
fn derive_struct_pointer() {
    #[derive(UnboxitSerialize)]
    struct Pointer {
        x: i32,
        y: i32
    }
}