//!
//! The Zinc VM contract type storage field.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::data::r#type::Type;

///
/// The contract type storage field representation.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractField {
    /// The field name.
    pub name: String,
    /// The field type.
    pub r#type: Type,
    /// Whether the field is public.
    pub is_public: bool,
    /// Whether the field is implicit.
    pub is_implicit: bool,
}

impl ContractField {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(name: String, r#type: Type, is_public: bool, is_implicit: bool) -> Self {
        Self {
            name,
            r#type,
            is_public,
            is_implicit,
        }
    }
}
