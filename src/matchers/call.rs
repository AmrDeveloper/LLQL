use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::Matcher;

#[derive(Clone)]
pub struct CallInstMatcher;

impl CallInstMatcher {
    pub fn create_call() -> Self {
        CallInstMatcher
    }
}

impl Matcher<LLVMValueRef> for CallInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { LLVMGetInstructionOpcode(*instruction) == LLVMOpcode::LLVMCall }
    }
}
