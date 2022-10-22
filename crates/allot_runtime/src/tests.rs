#[test]
fn operations() {
    use crate::{bytecode::ByteForm, ArithmeticOperation, Operation};

    assert_eq!(
        Operation::from_byte(0),
        Operation::Arithmetic(ArithmeticOperation::Add)
    );
}
