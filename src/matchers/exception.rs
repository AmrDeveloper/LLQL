use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::InstMatcher;

#[derive(Clone)]
pub struct LandingPadInstMatcher;

impl InstMatcher for LandingPadInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe { LLVMOpcode::LLVMLandingPad == LLVMGetInstructionOpcode(instruction) }
    }
}

#[derive(Clone)]
pub struct InvokeInstMatcher;

impl InstMatcher for InvokeInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: LLVMValueRef) -> bool {
        unsafe { LLVMOpcode::LLVMInvoke == LLVMGetInstructionOpcode(instruction) }
    }
}
