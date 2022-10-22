#[test]
fn operations() {
    use crate::{bytecode::ByteForm, OpPrim1, Operation};

    assert_eq!(
        Operation::from_byte(0),
        Operation::Prim1(OpPrim1::Increment)
    );
}
