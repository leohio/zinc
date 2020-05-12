use crate::gadgets;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::{utils, Scalar, ScalarType, ScalarTypeExpectation};
use crate::RuntimeError;
use crate::{Engine, Result};
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;

use crate::gadgets::utils::bigint_to_fr;
use num_bigint::{BigInt, Sign};

pub fn shift_left<E, CS>(cs: CS, num: &Scalar<E>, shift: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    num.get_type().assert_signed(false)?;
    shift.get_type().assert_signed(false)?;

    match shift.get_variant() {
        ScalarVariant::Variable(_) => variable_shift(cs, num, shift),
        ScalarVariant::Constant(_) => match num.get_variant() {
            ScalarVariant::Variable(_) => variable_num(cs, num, shift.get_constant_usize()?),
            ScalarVariant::Constant(_) => {
                let scalar_type = num.get_type();

                let num_value = utils::fr_to_bigint(&num.get_constant()?, scalar_type.is_signed());
                let shift_value = shift.get_constant_usize()?;

                let mask = vec![0xFF; scalar_type.bit_length::<E>() / 8];

                let mut result_value = &num_value << shift_value;
                result_value &= &BigInt::from_bytes_le(Sign::Plus, mask.as_slice());

                let result_fr =
                    bigint_to_fr::<E>(&result_value).ok_or(RuntimeError::ValueOverflow {
                        value: result_value,
                        scalar_type,
                    })?;
                Ok(Scalar::new_constant_fr(result_fr, scalar_type))
            }
        },
    }
}

fn variable_shift<E, CS>(mut cs: CS, num: &Scalar<E>, shift: &Scalar<E>) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    let scalar_type = num.get_type();
    let len = scalar_type.bit_length::<E>();

    let mut bits = num
        .to_expression::<CS>()
        .into_bits_le_fixed(cs.namespace(|| "left bits"), len)?;

    bits.resize(len * 2, Boolean::Constant(false));

    let mut variants = Vec::with_capacity(len);
    variants.push(num.clone());

    for i in 1..len {
        let variant = AllocatedNum::pack_bits_to_element(
            cs.namespace(|| format!("offset {}", i)),
            &bits[i..i + len],
        )?;
        variants.push(variant.into());
    }
    variants.push(Scalar::new_constant_int(0, ScalarType::Field)); // offset `len` will clear all bits.

    let shift_bits_be = shift
        .to_expression::<CS>()
        .into_bits_le_fixed(
            cs.namespace(|| "shift bits"),
            shift.get_type().bit_length::<E>(),
        )?
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, b)| Scalar::from_boolean(cs.namespace(|| format!("bit {}", i)), b))
        .collect::<Result<Vec<_>>>()?;

    let result = gadgets::arrays::recursive_select(cs, &shift_bits_be, &variants)?;

    Ok(result.with_type_unchecked(scalar_type))
}

fn variable_num<E, CS>(mut cs: CS, num: &Scalar<E>, shift: usize) -> Result<Scalar<E>>
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    let scalar_type = num.get_type();
    let len = scalar_type.bit_length::<E>();

    let bits = num
        .to_expression::<CS>()
        .into_bits_le_fixed(cs.namespace(|| "left bits"), len)?;

    let shift_clipped = if shift > len { len } else { shift };

    let mut shifted_bits = vec![Boolean::Constant(false); shift_clipped];
    shifted_bits.extend_from_slice(&bits);

    let result = AllocatedNum::pack_bits_to_element(
        cs.namespace(|| "pack result bits"),
        &shifted_bits[..len],
    )?;

    Ok(Scalar::new_unchecked_variable(
        result.get_value(),
        result.get_variable(),
        scalar_type,
    ))
}
