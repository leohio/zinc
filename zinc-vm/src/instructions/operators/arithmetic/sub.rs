use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::ScalarType;
use zinc_bytecode::Sub;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Sub {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let diff_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let unchecked_diff = gadgets::arithmetic::sub::sub(cs.namespace(|| "diff"), &left, &right)?;

        let diff = gadgets::types::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &unchecked_diff,
            diff_type,
        )?;

        vm.push(Cell::Value(diff))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_sub() -> Result<(), TestingError> {
        TestRunner::new()
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Sub)
            .test(&[1])
    }
}