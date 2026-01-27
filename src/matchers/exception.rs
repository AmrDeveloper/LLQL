use inkwell::llvm_sys::core::LLVMGetInstructionOpcode;
use inkwell::llvm_sys::core::LLVMIsAResumeInst;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMOpcode;

use crate::matchers::matchers_helper::is_call_or_invoke_inst_with_specific_name;

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

#[derive(Clone)]
pub struct ThrowInstMatcher;

impl Matcher<LLVMValueRef> for ThrowInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        is_call_or_invoke_inst_with_specific_name(instruction, "__cxa_throw")
    }
}

#[derive(Clone)]
pub struct RethrowInstMatcher;

impl Matcher<LLVMValueRef> for RethrowInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        is_call_or_invoke_inst_with_specific_name(instruction, "__cxa_rethrow")
    }
}
