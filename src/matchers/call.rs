use inkwell::llvm_sys::core::LLVMGetCalledValue;
use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::core::LLVMGetIntrinsicID;
use inkwell::llvm_sys::core::LLVMIsACallInst;
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

#[derive(Clone)]
pub struct IntrinsicInstMatcher;

impl IntrinsicInstMatcher {
    pub fn create_call() -> Self {
        IntrinsicInstMatcher
    }
}

impl Matcher<LLVMValueRef> for IntrinsicInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            if !LLVMIsACallInst(*instruction).is_null() {
                let called_value = LLVMGetCalledValue(*instruction);
                let intrinsic_id = LLVMGetIntrinsicID(called_value);
                return intrinsic_id != 0;
            }
            false
        }
    }
}
