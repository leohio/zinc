extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::PushConst;

impl<E, O> VMInstruction<E, O> for PushConst
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let op = vm.get_operator();
        let value = op.constant_bigint(&self.value)?;
        vm.memory()?.push(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};

    #[test]
    fn test_push() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PushConst { value: 42.into() })
            .add(PushConst {
                value: 0xABCD.into(),
            })
            .add(PushConst { value: (-1).into() })
            .add(PushConst {
                value: (-1000).into(),
            })
            .test(&[-1000, -1, 0xABCD, 42, 0])
    }
}
