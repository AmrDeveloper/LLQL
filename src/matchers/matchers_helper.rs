use inkwell::llvm_sys::core::LLVMGetCalledValue;
use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::core::LLVMGetValueName2;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

pub(crate) fn is_call_or_invoke_inst_with_specific_name(
    instruction: &LLVMValueRef,
    name: &'static str,
) -> bool {
    unsafe {
        if matches!(
            LLVMGetInstructionOpcode(*instruction),
            LLVMOpcode::LLVMCall | LLVMOpcode::LLVMInvoke
        ) {
            let mut len: usize = 0;
            let called_value = LLVMGetCalledValue(*instruction);
            let name_ptr = LLVMGetValueName2(called_value, &mut len);
            let name_slice = std::slice::from_raw_parts(name_ptr as *const u8, len);
            let called_name = std::str::from_utf8_unchecked(name_slice).to_string();
            return called_name.eq(name);
        }
        false
    }
}
