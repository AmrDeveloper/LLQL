use inkwell::llvm_sys::core::{LLVMConstIntGetSExtValue, LLVMGetValueKind, LLVMIsAConstantInt};
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMValueKind;

use super::Matcher;

#[derive(Clone)]
enum ConstIntMatcherCondition {
    Specific(i64),
    PowerOfTwo,
}

/// Return instruction matcher to check if current value is a constants integer
#[derive(Clone)]
pub struct ConstIntMatcher {
    condition: Option<ConstIntMatcherCondition>,
}

impl ConstIntMatcher {
    pub fn create_const_int() -> Self {
        ConstIntMatcher { condition: None }
    }

    pub fn create_specific_int(value: i64) -> Self {
        ConstIntMatcher {
            condition: Some(ConstIntMatcherCondition::Specific(value)),
        }
    }

    pub fn create_power_of_two() -> Self {
        ConstIntMatcher {
            condition: Some(ConstIntMatcherCondition::PowerOfTwo),
        }
    }

    pub fn create_one() -> Self {
        ConstIntMatcher {
            condition: Some(ConstIntMatcherCondition::Specific(1)),
        }
    }

    pub fn create_zero() -> Self {
        ConstIntMatcher {
            condition: Some(ConstIntMatcherCondition::Specific(0)),
        }
    }
}

impl Matcher<LLVMValueRef> for ConstIntMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            if LLVMIsAConstantInt(*instruction).is_null() {
                return false;
            }

            if let Some(matcher_condition) = &self.condition {
                let si64_value = LLVMConstIntGetSExtValue(*instruction);
                return match matcher_condition {
                    ConstIntMatcherCondition::Specific(value) => si64_value == *value,
                    ConstIntMatcherCondition::PowerOfTwo => (si64_value & (si64_value - 1)) != 0,
                };
            }

            true
        }
    }
}

/// Return instruction matcher to check if current value is a constants floating point
#[derive(Clone)]
pub struct ConstFloatMatcher;

impl Matcher<LLVMValueRef> for ConstFloatMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(*instruction);
            value_kind == LLVMValueKind::LLVMConstantFPValueKind
        }
    }
}

/// Return instruction matcher to check if current value is a constants pointer null
#[derive(Clone)]
pub struct ConstPointerNullMatcher;

impl Matcher<LLVMValueRef> for ConstPointerNullMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            let value_kind = LLVMGetValueKind(*instruction);
            value_kind == LLVMValueKind::LLVMConstantPointerNullValueKind
        }
    }
}
