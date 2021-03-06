use franklin_crypto::bellman::pairing::ff::Field;
use franklin_crypto::bellman::ConstraintSystem;

use zinc_build::ScalarType;

use crate::auto_const;
use crate::error::RuntimeError;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

pub fn sub<E, CS>(cs: CS, left: &Scalar<E>, right: &Scalar<E>) -> Result<Scalar<E>, RuntimeError>
where
    E: IEngine,
    CS: ConstraintSystem<E>,
{
    fn inner<E, CS>(
        mut cs: CS,
        left: &Scalar<E>,
        right: &Scalar<E>,
    ) -> Result<Scalar<E>, RuntimeError>
    where
        E: IEngine,
        CS: ConstraintSystem<E>,
    {
        let mut value = None;

        let variable = cs.alloc(
            || "variable",
            || {
                let mut tmp = left.grab_value()?;
                tmp.sub_assign(&right.grab_value()?);
                value = Some(tmp);
                Ok(tmp)
            },
        )?;

        cs.enforce(
            || "sub",
            |lc| lc + &left.to_linear_combination::<CS>() - &right.to_linear_combination::<CS>(),
            |lc| lc + CS::one(),
            |lc| lc + variable,
        );

        Ok(Scalar::new_unchecked_variable(
            value,
            variable,
            ScalarType::Field,
        ))
    }

    auto_const!(inner, cs, left, right)
}
