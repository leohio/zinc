//!
//! The constant element tests.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

#[test]
fn error_element_constant_range_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true .. 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::OperatorRangeFirstOperandExpectedInteger {
            found: Constant::Boolean(true).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_range_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 .. true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::OperatorRangeSecondOperandExpectedInteger {
            found: Constant::Boolean(true).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_range_inclusive_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true ..= 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(
            ConstantError::OperatorRangeInclusiveFirstOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_range_inclusive_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 ..= true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(
            ConstantError::OperatorRangeInclusiveSecondOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_or_1st_expected_boolean() {
    let input = r#"
fn main() {
    let value = 42 || true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::OperatorOrFirstOperandExpectedBoolean {
            found: Constant::Integer(IntegerConstant::new(
                BigInt::from(42),
                false,
                crate::BITLENGTH_BYTE,
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_or_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let value = true || 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::OperatorOrSecondOperandExpectedBoolean {
            found: Constant::Integer(IntegerConstant::new(
                BigInt::from(42),
                false,
                crate::BITLENGTH_BYTE,
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_xor_1st_expected_boolean() {
    let input = r#"
fn main() {
    let value = 42 ^^ true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::OperatorXorFirstOperandExpectedBoolean {
            found: Constant::Integer(IntegerConstant::new(
                BigInt::from(42),
                false,
                crate::BITLENGTH_BYTE,
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_xor_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let value = true ^^ 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::OperatorXorSecondOperandExpectedBoolean {
            found: Constant::Integer(IntegerConstant::new(
                BigInt::from(42),
                false,
                crate::BITLENGTH_BYTE,
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_and_1st_expected_boolean() {
    let input = r#"
fn main() {
    let value = 42 && true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::OperatorAndFirstOperandExpectedBoolean {
            found: Constant::Integer(IntegerConstant::new(
                BigInt::from(42),
                false,
                crate::BITLENGTH_BYTE,
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_and_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let value = true && 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::OperatorAndSecondOperandExpectedBoolean {
            found: Constant::Integer(IntegerConstant::new(
                BigInt::from(42),
                false,
                crate::BITLENGTH_BYTE,
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_equals_1st_expected_primitive() {
    let input = r#"
fn main() {
    let value = "string" == 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Constant(
            ConstantError::OperatorEqualsFirstOperandExpectedPrimitiveType {
                found: Constant::String("string".to_owned()).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_equals_2nd_expected_unit() {
    let input = r#"
fn main() {
    let value = () == 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedUnit {
            found: Constant::Integer(IntegerConstant::new(
                BigInt::from(42),
                false,
                crate::BITLENGTH_BYTE,
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_equals_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let value = true == 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedBoolean {
            found: Constant::Integer(IntegerConstant::new(
                BigInt::from(42),
                false,
                crate::BITLENGTH_BYTE,
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 == true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedInteger {
            found: Constant::Boolean(true).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_not_equals_1st_expected_primitive() {
    let input = r#"
fn main() {
    let value = "string" != 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Constant(
            ConstantError::OperatorNotEqualsFirstOperandExpectedPrimitiveType {
                found: Constant::String("string".to_owned()).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_not_equals_2nd_expected_unit() {
    let input = r#"
fn main() {
    let value = () != 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::OperatorNotEqualsSecondOperandExpectedUnit {
            found: Constant::Integer(IntegerConstant::new(
                BigInt::from(42),
                false,
                crate::BITLENGTH_BYTE,
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_not_equals_2nd_expected_boolean() {
    let input = r#"
fn main() {
    let value = true != 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(
            ConstantError::OperatorNotEqualsSecondOperandExpectedBoolean {
                found: Constant::Integer(IntegerConstant::new(
                    BigInt::from(42),
                    false,
                    crate::BITLENGTH_BYTE,
                ))
                .to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_not_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 != true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(
            ConstantError::OperatorNotEqualsSecondOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_greater_equals_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true >= 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(
            ConstantError::OperatorGreaterEqualsFirstOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_greater_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 >= true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(
            ConstantError::OperatorGreaterEqualsSecondOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_lesser_equals_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true <= 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(
            ConstantError::OperatorLesserEqualsFirstOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_lesser_equals_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 <= true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(
            ConstantError::OperatorLesserEqualsSecondOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_greater_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true > 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::OperatorGreaterFirstOperandExpectedInteger {
            found: Constant::Boolean(true).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_greater_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 > true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::OperatorGreaterSecondOperandExpectedInteger {
            found: Constant::Boolean(true).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_lesser_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true < 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::OperatorLesserFirstOperandExpectedInteger {
            found: Constant::Boolean(true).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_lesser_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 < true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(ConstantError::OperatorLesserSecondOperandExpectedInteger {
            found: Constant::Boolean(true).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_addition_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true + 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::OperatorAdditionFirstOperandExpectedInteger {
            found: Constant::Boolean(true).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_addition_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 + true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(
            ConstantError::OperatorAdditionSecondOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_subtraction_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true - 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(
            ConstantError::OperatorSubtractionFirstOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_subtraction_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 - true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(
            ConstantError::OperatorSubtractionSecondOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_multiplication_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true * 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(
            ConstantError::OperatorMultiplicationFirstOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_multiplication_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 * true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(
            ConstantError::OperatorMultiplicationSecondOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_division_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true / 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(ConstantError::OperatorDivisionFirstOperandExpectedInteger {
            found: Constant::Boolean(true).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_division_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 / true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(
            ConstantError::OperatorDivisionSecondOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_remainder_1st_expected_integer() {
    let input = r#"
fn main() {
    let value = true % 42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 22),
        ElementError::Constant(
            ConstantError::OperatorRemainderFirstOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_remainder_2nd_expected_integer() {
    let input = r#"
fn main() {
    let value = 42 % true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Constant(
            ConstantError::OperatorRemainderSecondOperandExpectedInteger {
                found: Constant::Boolean(true).to_string(),
            },
        ),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_casting_to_invalid_type_const() {
    let input = r#"
fn main() {
    const VALUE: u8 = 42;
    const RESULT: bool = VALUE;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 19),
        ElementError::Constant(ConstantError::Casting(CasterError::CastingToInvalidType {
            from: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            to: Type::boolean().to_string(),
        })),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_casting_to_invalid_type_static() {
    let input = r#"
static VALUE: u8 = 42;
static RESULT: bool = VALUE;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 16),
        ElementError::Constant(ConstantError::Casting(CasterError::CastingToInvalidType {
            from: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            to: Type::boolean().to_string(),
        })),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_not_expected_boolean() {
    let input = r#"
fn main() {
    let value = !42;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::OperatorNotExpectedBoolean {
            found: Constant::Integer(IntegerConstant::new(
                BigInt::from(42),
                false,
                crate::BITLENGTH_BYTE,
            ))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}

#[test]
fn error_element_constant_negation_expected_integer() {
    let input = r#"
fn main() {
    let value = -true;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 17),
        ElementError::Constant(ConstantError::OperatorNegationExpectedInteger {
            found: Constant::Boolean(true).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry_point(input);

    assert_eq!(result, expected);
}