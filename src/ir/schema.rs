use std::collections::HashMap;

use gitql_ast::types::text::TextType;
use gitql_ast::types::DataType;
use gitql_core::environment::Environment;
use gitql_core::schema::Schema;
use gitql_std::aggregation::aggregation_function_signatures;
use gitql_std::aggregation::aggregation_functions;
use gitql_std::window::window_function_signatures;
use gitql_std::window::window_functions;

use crate::functions::{llvm_ir_function_signatures, llvm_ir_functions};

use super::types::LLVMInstType;

fn llvm_tables_fields_types() -> HashMap<&'static str, Box<dyn DataType>> {
    let mut map: HashMap<&str, Box<dyn DataType>> = HashMap::new();

    // Instructions Table
    map.insert("function_name", Box::new(TextType));
    map.insert("basic_block_name", Box::new(TextType));
    map.insert("instruction", Box::new(LLVMInstType));

    map
}

fn llvm_tables_fields_names() -> HashMap<&'static str, Vec<&'static str>> {
    let mut map = HashMap::new();
    map.insert(
        "instructions",
        vec!["function_name", "basic_block_name", "instruction"],
    );
    map
}

pub fn create_llql_environment() -> Environment {
    let schema = Schema {
        tables_fields_names: llvm_tables_fields_names().to_owned(),
        tables_fields_types: llvm_tables_fields_types().to_owned(),
    };

    let std_signatures = llvm_ir_function_signatures();
    let std_functions = llvm_ir_functions();

    let aggregation_signatures = aggregation_function_signatures();
    let aggregation_functions = aggregation_functions();

    let window_signatures = window_function_signatures();
    let window_function = window_functions();

    let mut env = Environment::new(schema);
    env.with_standard_functions(&std_signatures, std_functions);
    env.with_aggregation_functions(&aggregation_signatures, aggregation_functions);
    env.with_window_functions(&window_signatures, window_function);

    env
}
