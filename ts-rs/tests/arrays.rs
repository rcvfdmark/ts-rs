use ts_rs::TS;

#[test]
fn interface() {
    #[derive(TS)]
    struct Interface {
        #[allow(dead_code)]
        a: [i32; 10],
    }

    assert_eq!(
        Interface::inline(0),
        "{
    a: Array<number>,
}"
    )
}

#[test]
fn newtype() {
    #[derive(TS)]
    struct Newtype(#[allow(dead_code)] [i32; 10]);

    assert_eq!(Newtype::inline(0), "Array<number>")
}