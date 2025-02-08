use inkwell::llvm_sys::core::LLVMGetFirstUse;
use inkwell::llvm_sys::core::LLVMGetNextUse;
use inkwell::llvm_sys::prelude::LLVMValueRef;

use super::Matcher;

#[derive(Clone)]
pub struct UsageInstMatcher {
    pub matcher: Box<dyn Matcher<LLVMValueRef>>,
    pub count: usize,
}

impl UsageInstMatcher {
    pub fn create_unused_matcher(
        matcher: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(UsageInstMatcher { matcher, count: 0 })
    }

    pub fn create_has_one_use_matcher(
        matcher: Box<dyn Matcher<LLVMValueRef>>,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(UsageInstMatcher { matcher, count: 1 })
    }

    pub fn create_has_n_uses_matcher(
        matcher: Box<dyn Matcher<LLVMValueRef>>,
        n: usize,
    ) -> Box<dyn Matcher<LLVMValueRef>> {
        Box::new(UsageInstMatcher { matcher, count: n })
    }
}

impl Matcher<LLVMValueRef> for UsageInstMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            // The instruction itself is not matches
            if !self.matcher.is_match(instruction) {
                return false;
            }

            let first_use = LLVMGetFirstUse(*instruction);

            // It's not used
            if first_use.is_null() {
                return self.count == 0;
            }

            let mut number_of_usage = 1;
            let mut next_use = LLVMGetNextUse(first_use);
            while !next_use.is_null() {
                number_of_usage += 1;
                next_use = LLVMGetNextUse(next_use);
            }

            self.count == number_of_usage
        }
    }
}
