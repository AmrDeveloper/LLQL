use std::sync::LazyLock;

use gitql_core::object::Row;
use gitql_core::values::base::Value;
use gitql_core::values::null::NullValue;
use gitql_core::values::text::TextValue;
use gitql_engine::data_provider::DataProvider;
use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;
use inkwell::values::AsValueRef;

use super::values::LLVMInstValue;

static mut LLVM_CONTEXT: LazyLock<Context> = LazyLock::new(Context::create);
static mut LLVM_MODULES: Vec<Module> = Vec::new();

pub struct LLVMIRDataProvider {
    pub paths: Vec<String>,
}

impl LLVMIRDataProvider {
    pub fn new(paths: Vec<String>) -> Self {
        unsafe {
            for path in paths.iter() {
                let module = if path.ends_with(".ll") {
                    let memory_buffer = MemoryBuffer::create_from_file(path.as_ref()).unwrap();
                    LLVM_CONTEXT.create_module_from_ir(memory_buffer).unwrap()
                } else {
                    Module::parse_bitcode_from_path(path, &*LLVM_CONTEXT).unwrap()
                };
                LLVM_MODULES.push(module.to_owned());
            }
        };

        Self { paths }
    }
}

impl DataProvider for LLVMIRDataProvider {
    fn provide(&self, table: &str, selected_columns: &[String]) -> Result<Vec<Row>, String> {
        let mut rows: Vec<Row> = vec![];

        for (path_index, path) in self.paths.iter().enumerate() {
            let mut selected_rows =
                select_llvm_ir_objects(path, path_index, table, selected_columns)?;
            rows.append(&mut selected_rows);
        }

        Ok(rows)
    }
}

fn select_llvm_ir_objects(
    path: &str,
    path_index: usize,
    table: &str,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let rows = match table {
        "instructions" => select_llvm_ir_instructions(path, path_index, selected_columns)?,
        _ => vec![Row { values: vec![] }],
    };
    Ok(rows)
}

fn select_llvm_ir_instructions(
    path: &str,
    path_index: usize,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];
    let row_width = selected_columns.len();

    unsafe {
        for function in LLVM_MODULES[path_index].get_functions() {
            let function_name = function.get_name().to_str().unwrap().to_string();
            for basic_block in function.get_basic_block_iter() {
                let basic_block_name = basic_block.get_name().to_str().unwrap().to_string();
                for inst in basic_block.get_instructions() {
                    let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(row_width);
                    for field_name in selected_columns {
                        if field_name == "function_name" {
                            values.push(Box::new(TextValue {
                                value: function_name.clone(),
                            }));
                            continue;
                        }

                        if field_name == "basic_block_name" {
                            values.push(Box::new(TextValue {
                                value: basic_block_name.clone(),
                            }));
                            continue;
                        }

                        if field_name == "instruction" {
                            values.push(Box::new(LLVMInstValue {
                                llvm_value: inst.as_value_ref(),
                            }));
                            continue;
                        }

                        if field_name == "file_name" {
                            values.push(Box::new(TextValue {
                                value: path.to_string(),
                            }));
                            continue;
                        }

                        values.push(Box::new(NullValue));
                    }

                    let row = Row { values };
                    rows.push(row);
                }
            }
        }
    }

    Ok(rows)
}
