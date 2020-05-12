//!
//! The Zinc VM bytecode.
//!

pub mod metadata;

use std::collections::HashMap;

use zinc_bytecode::data::values::Value as TemplateValue;
use zinc_bytecode::Instruction;
use zinc_bytecode::Program;

use crate::generator::r#type::Type;
use crate::lexical::token::location::Location;

use self::metadata::Metadata;

///
/// The Zinc VM bytecode, generated by the compiler target code generator.
///
#[derive(Debug, PartialEq)]
pub struct Bytecode {
    entry_metadata_map: HashMap<usize, Metadata>,
    instructions: Vec<Instruction>,

    data_stack_pointer: usize,
    variable_addresses: HashMap<String, usize>,
    function_addresses: HashMap<usize, usize>,

    current_file: String,
    current_location: Location,
}

impl Default for Bytecode {
    fn default() -> Self {
        Self::new()
    }
}

impl Bytecode {
    const INSTRUCTION_VECTOR_INITIAL_SIZE: usize = 1024;
    const FUNCTION_ADDRESSES_HASHMAP_INITIAL_SIZE: usize = 16;
    const VARIABLE_ADDRESSES_HASHMAP_INITIAL_SIZE: usize = 16;
    const ENTRY_METADATA_HASHMAP_INITIAL_SIZE: usize = 4;

    pub fn new() -> Self {
        let mut instructions = Vec::with_capacity(Self::INSTRUCTION_VECTOR_INITIAL_SIZE);
        instructions.push(Instruction::NoOperation(zinc_bytecode::NoOperation));
        instructions.push(Instruction::NoOperation(zinc_bytecode::NoOperation));

        Self {
            entry_metadata_map: HashMap::with_capacity(Self::ENTRY_METADATA_HASHMAP_INITIAL_SIZE),
            instructions,

            data_stack_pointer: 0,
            variable_addresses: HashMap::with_capacity(
                Self::VARIABLE_ADDRESSES_HASHMAP_INITIAL_SIZE,
            ),
            function_addresses: HashMap::with_capacity(
                Self::FUNCTION_ADDRESSES_HASHMAP_INITIAL_SIZE,
            ),

            current_file: String::new(),
            current_location: Location::new_beginning(None),
        }
    }

    pub fn entries(&self) -> Vec<usize> {
        self.entry_metadata_map.keys().copied().collect()
    }

    pub fn start_new_file(&mut self, name: &str) {
        self.current_file = name.to_owned();
    }

    pub fn start_function(&mut self, unique_id: usize, identifier: String) {
        let address = self.instructions.len();
        self.function_addresses.insert(unique_id, address);
        self.data_stack_pointer = 0;

        self.instructions.push(Instruction::FileMarker(
            zinc_bytecode::instructions::FileMarker::new(self.current_file.clone()),
        ));
        self.instructions.push(Instruction::FunctionMarker(
            zinc_bytecode::FunctionMarker::new(identifier),
        ));
    }

    pub fn start_entry_function(
        &mut self,
        identifier: String,
        unique_id: usize,
        input_arguments: Vec<(String, Type)>,
        output_type: Option<Type>,
    ) {
        let metadata = Metadata::new(
            identifier.clone(),
            input_arguments,
            output_type.unwrap_or_else(|| Type::structure(vec![])),
        );
        self.entry_metadata_map.insert(unique_id, metadata);

        self.start_function(unique_id, identifier);
    }

    pub fn define_variable(&mut self, identifier: Option<String>, r#type: Type) -> usize {
        let start_address = self.data_stack_pointer;
        if let Some(identifier) = identifier {
            self.variable_addresses
                .insert(identifier, self.data_stack_pointer);
        }
        self.data_stack_pointer += r#type.size();
        start_address
    }

    pub fn push_instruction(&mut self, instruction: Instruction, location: Option<Location>) {
        if let Some(location) = location {
            if self.current_location != location {
                if self.current_location.line != location.line {
                    self.instructions.push(Instruction::LineMarker(
                        zinc_bytecode::LineMarker::new(location.line),
                    ));
                }
                if self.current_location.column != location.column {
                    self.instructions.push(Instruction::ColumnMarker(
                        zinc_bytecode::ColumnMarker::new(location.column),
                    ));
                }
                self.current_location = location;
            }
        }

        self.instructions.push(instruction)
    }

    pub fn get_function_address(&self, unique_id: usize) -> Option<usize> {
        self.function_addresses.get(&unique_id).copied()
    }

    pub fn get_variable_address(&self, name: &str) -> Option<usize> {
        self.variable_addresses.get(name).copied()
    }

    pub fn entry_name(&self, entry_id: usize) -> &str {
        self.entry_metadata_map
            .get(&entry_id)
            .expect(crate::panic::ENSURED_WHILE_RETURNING_ENTRIES)
            .entry_name
            .as_str()
    }

    pub fn input_template_bytes(&self, entry_id: usize) -> Vec<u8> {
        let input_type = self
            .entry_metadata_map
            .get(&entry_id)
            .expect(crate::panic::ENSURED_WHILE_RETURNING_ENTRIES)
            .input_fields_as_struct()
            .into();
        let input_template_value = TemplateValue::default_from_type(&input_type);

        match serde_json::to_string_pretty(&input_template_value.to_json()) {
            Ok(json) => (json + "\n").into_bytes(),
            Err(error) => panic!(
                crate::panic::JSON_TEMPLATE_SERIALIZATION.to_owned() + error.to_string().as_str()
            ),
        }
    }

    pub fn output_template_bytes(&self, entry_id: usize) -> Vec<u8> {
        let output_bytecode_type = self
            .entry_metadata_map
            .get(&entry_id)
            .expect(crate::panic::ENSURED_WHILE_RETURNING_ENTRIES)
            .output_type
            .to_owned()
            .into();
        let output_value_template = TemplateValue::default_from_type(&output_bytecode_type);

        match serde_json::to_string_pretty(&output_value_template.to_json()) {
            Ok(json) => (json + "\n").into_bytes(),
            Err(error) => panic!(
                crate::panic::JSON_TEMPLATE_SERIALIZATION.to_owned() + error.to_string().as_str()
            ),
        }
    }

    pub fn entry_to_bytes(&mut self, entry_id: usize) -> Vec<u8> {
        let metadata = self
            .entry_metadata_map
            .remove(&entry_id)
            .expect(crate::panic::ENSURED_WHILE_RETURNING_ENTRIES);
        let mut instructions = self.instructions.clone();
        let entry_address = self
            .get_function_address(entry_id)
            .expect(crate::panic::ENSURED_WHILE_RETURNING_ENTRIES);
        instructions[0] = Instruction::Call(zinc_bytecode::Call::new(
            entry_address,
            metadata.input_size(),
        ));
        instructions[1] = Instruction::Exit(zinc_bytecode::Exit::new(metadata.output_size()));

        for (index, instruction) in instructions.iter().enumerate() {
            match instruction {
                instruction @ Instruction::FileMarker(_)
                | instruction @ Instruction::FunctionMarker(_)
                | instruction @ Instruction::LineMarker(_)
                | instruction @ Instruction::ColumnMarker(_) => {
                    log::trace!("{:03} {:?}", index, instruction)
                }
                instruction => log::debug!("{:03} {:?}", index, instruction),
            }
        }

        Program::new(
            metadata.input_fields_as_struct().into(),
            metadata.output_type.into(),
            instructions,
        )
        .to_bytes()
    }

    pub fn function_name_to_entry_id(&self, name: &str) -> Option<usize> {
        for (id, metadata) in self.entry_metadata_map.iter() {
            if name == metadata.entry_name.as_str() {
                return Some(*id);
            }
        }

        None
    }

    pub fn into_instructions(self) -> Vec<Instruction> {
        self.instructions
    }
}
