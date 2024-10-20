use std::collections::HashMap;

use gitql_ast::types::base::DataType;
use gitql_ast::types::text::TextType;

use super::types::LLVMInstType;

pub fn llvm_tables_fields_types() -> HashMap<&'static str, Box<dyn DataType>> {
    let mut map: HashMap<&str, Box<dyn DataType>> = HashMap::new();

    // Instructions Table
    map.insert("function_name", Box::new(TextType));
    map.insert("basic_block_name", Box::new(TextType));
    map.insert("instruction", Box::new(LLVMInstType));

    map
}

pub fn llvm_tables_fields_names() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();
    map.insert(
        "instructions",
        vec!["function_name", "basic_block_name", "instruction"],
    );
    map
}
