//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::Store;
use zinc_bytecode::StoreArray;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

use crate::semantic::BinaryAnalyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn main () {
    let mut array_double: [[u8; 4]; 4] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ];

    array_double[1][3] = 42;
    array_double[2][2] = 111;
    array_double[3][1] = 255;
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(3), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(4), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(5), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(6), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(7), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(8), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(9), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(10), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(11), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(12), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(13), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(14), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(15), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(16), false, 8)),
        Instruction::StoreArray(StoreArray::new(0, 16)),
        Instruction::PushConst(PushConst::new(BigInt::from(42), false, 8)),
        Instruction::Store(Store::new(7)),
        Instruction::PushConst(PushConst::new(BigInt::from(111), false, 8)),
        Instruction::Store(Store::new(10)),
        Instruction::PushConst(PushConst::new(BigInt::from(255), false, 8)),
        Instruction::Store(Store::new(13)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::instructions(input);

    assert_eq!(expected, result);
}
