//!
//! The generator expression operand.
//!

pub mod array;
pub mod block;
pub mod conditional;
pub mod constant;
pub mod group;
pub mod list;
pub mod r#match;
pub mod place;

use std::cell::RefCell;
use std::rc::Rc;

use num::BigInt;

use zinc_build::Instruction;

use crate::generator::expression::operand::constant::integer::Integer as IntegerConstant;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::semantic::element::access::dot::contract_field::ContractField as ContractFieldAccess;
use crate::semantic::element::place::element::Element as SemanticPlaceElement;
use crate::semantic::element::place::memory_type::MemoryType;

use self::array::Expression as ArrayExpression;
use self::block::Expression as BlockExpression;
use self::conditional::Expression as ConditionalExpression;
use self::constant::Constant;
use self::group::Expression as GroupExpression;
use self::list::Expression as ListExpression;
use self::place::Place;
use self::r#match::Expression as MatchExpression;

///
/// The expression operand which is translated to Zinc VM data.
///
#[derive(Debug, Clone)]
pub enum Operand {
    /// The constant operand.
    Constant(Constant),
    /// The memory descriptor operand.
    Place(Place),
    /// The array literal expression operand.
    Array(ArrayExpression),
    /// The variable group (usually tuple or structure fields) expression operand.
    Group(GroupExpression),
    /// The function argument list expression.
    List(ListExpression),
    /// The block expression.
    Block(BlockExpression),
    /// The conditional expression.
    Conditional(ConditionalExpression),
    /// The `match` expression.
    Match(MatchExpression),
}

impl IBytecodeWritable for Operand {
    fn write_all(self, state: Rc<RefCell<State>>) {
        match self {
            Self::Constant(inner) => inner.write_all(state),
            Self::Place(mut inner) => match inner.memory_type {
                MemoryType::Stack => {
                    let location = inner.identifier.location;
                    let element_size = inner.element_size;
                    let total_size = inner.total_size;
                    let address = state
                        .borrow()
                        .get_variable_address(inner.identifier.name.as_str())
                        .expect(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS);

                    let is_indexed = !inner.elements.is_empty();

                    if is_indexed {
                        inner.write_all(state.clone());
                        state.borrow_mut().push_instruction(
                            Instruction::LoadByIndex(zinc_build::LoadByIndex::new(
                                address,
                                element_size,
                                total_size,
                            )),
                            Some(location),
                        );
                    } else {
                        state.borrow_mut().push_instruction(
                            Instruction::Load(zinc_build::Load::new(address, total_size)),
                            Some(location),
                        );
                    }
                }
                MemoryType::ContractStorage => {
                    let location = inner.identifier.location;
                    let element_size = inner.element_size;
                    let total_size = inner.total_size;

                    if let Some(SemanticPlaceElement::ContractField {
                        access:
                            ContractFieldAccess {
                                position,
                                element_size,
                                is_mtreemap,
                                ..
                            },
                    }) = inner.elements.first()
                    {
                        IntegerConstant::new(
                            BigInt::from(*position),
                            false,
                            zinc_const::bitlength::FIELD,
                        )
                        .write_all(state.clone());

                        if !is_mtreemap {
                            state.borrow_mut().push_instruction(
                                Instruction::StorageLoad(zinc_build::StorageLoad::new(
                                    *element_size,
                                )),
                                Some(inner.identifier.location),
                            );
                        }

                        inner.elements.remove(0);
                    }

                    let is_indexed = !inner.elements.is_empty();

                    if is_indexed {
                        inner.write_all(state.clone());
                        state.borrow_mut().push_instruction(
                            Instruction::Slice(zinc_build::Slice::new(element_size, total_size)),
                            Some(location),
                        );
                    }
                }
            },
            Self::Array(inner) => inner.write_all(state),
            Self::Group(inner) => inner.write_all(state),
            Self::List(inner) => inner.write_all(state),
            Self::Block(inner) => inner.write_all(state),
            Self::Conditional(inner) => inner.write_all(state),
            Self::Match(inner) => inner.write_all(state),
        }
    }
}
