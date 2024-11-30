use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;

use super::data_provider::LLVM_CONTEXT;
use super::data_provider::LLVM_MODULES;

pub fn parse_llvm_modules(paths: &[String]) -> Result<(), String> {
    #[allow(static_mut_refs)]
    unsafe {
        LLVM_MODULES.clear();
        for path in paths.iter() {
            let module_result = if path.ends_with(".ll") {
                let memory_buffer_result = MemoryBuffer::create_from_file(path.as_ref());
                if memory_buffer_result.is_err() {
                    let error = memory_buffer_result.err().unwrap();
                    return Err(error.to_string());
                }

                let memory_buffer = memory_buffer_result.ok().unwrap();
                LLVM_CONTEXT.create_module_from_ir(memory_buffer)
            } else {
                Module::parse_bitcode_from_path(path, &*LLVM_CONTEXT)
            };

            if module_result.is_err() {
                let error = module_result.err().unwrap();
                return Err(error.to_string());
            }

            let module = module_result.ok().unwrap();
            LLVM_MODULES.push(module.to_owned());
        }
    }
    Ok(())
}
