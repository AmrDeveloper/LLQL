use inkwell::llvm_sys::core::LLVMGetDebugLocColumn;
use inkwell::llvm_sys::core::LLVMGetDebugLocLine;
use inkwell::llvm_sys::prelude::LLVMValueRef;

use super::Matcher;

#[derive(Clone)]
pub struct DebugInfoLineMatcher {
    pub line: u32,
}

impl Matcher<LLVMValueRef> for DebugInfoLineMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { self.line == LLVMGetDebugLocLine(*instruction) }
    }
}

#[derive(Clone)]
pub struct DebugInfoColumnMatcher {
    pub column: u32,
}

impl Matcher<LLVMValueRef> for DebugInfoColumnMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { self.column == LLVMGetDebugLocColumn(*instruction) }
    }
}
