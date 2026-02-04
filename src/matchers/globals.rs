use inkwell::llvm_sys::core::LLVMIsAGlobalValue;
use inkwell::llvm_sys::core::LLVMIsAGlobalVariable;
use inkwell::llvm_sys::prelude::LLVMValueRef;

use super::Matcher;

#[derive(Clone)]
pub struct GlobalValueExprMatcher;

impl Matcher<LLVMValueRef> for GlobalValueExprMatcher {
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { !LLVMIsAGlobalValue(*instruction).is_null() }
    }
}

#[derive(Clone)]
pub struct GlobalVariableExprMatcher;

impl Matcher<LLVMValueRef> for GlobalVariableExprMatcher {
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { !LLVMIsAGlobalVariable(*instruction).is_null() }
    }
}
