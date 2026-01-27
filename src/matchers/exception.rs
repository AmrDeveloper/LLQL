use inkwell::llvm_sys::core::{LLVMGetInstructionOpcode, LLVMIsAResumeInst};
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use super::Matcher;

#[derive(Clone)]
pub struct LandingPadInstMatcher;

impl Matcher<LLVMValueRef> for LandingPadInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { LLVMOpcode::LLVMLandingPad == LLVMGetInstructionOpcode(*instruction) }
    }
}

#[derive(Clone)]
pub struct InvokeInstMatcher;

impl Matcher<LLVMValueRef> for InvokeInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { LLVMOpcode::LLVMInvoke == LLVMGetInstructionOpcode(*instruction) }
    }
}

#[derive(Clone)]
pub struct ResumeInstMatcher;

impl Matcher<LLVMValueRef> for ResumeInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { !LLVMIsAResumeInst(*instruction).is_null() }
    }
}
