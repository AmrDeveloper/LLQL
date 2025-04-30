use inkwell::llvm_sys::core::LLVMConstIntGetSExtValue;
use inkwell::llvm_sys::core::LLVMGetValueKind;
use inkwell::llvm_sys::core::LLVMIsAConstantExpr;
use inkwell::llvm_sys::core::LLVMIsAConstantFP;
use inkwell::llvm_sys::core::LLVMIsAConstantInt;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use inkwell::llvm_sys::LLVMValueKind;

use super::Matcher;

/// Return instruction matcher to check if current value is a constants expr
#[derive(Clone)]
pub struct ConstExprMatcher;

impl Matcher<LLVMValueRef> for ConstExprMatcher {
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe { !LLVMIsAConstantExpr(*instruction).is_null() }
    }
}

#[derive(Clone)]
enum ConstIntMatcherCondition {
    Specific(i64),
    InRange(i64, i64),
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

    pub fn create_range_int(start: i64, end: i64) -> Self {
        ConstIntMatcher {
            condition: Some(ConstIntMatcherCondition::InRange(start, end)),
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
                    ConstIntMatcherCondition::InRange(s, e) => si64_value >= *s || si64_value <= *e,
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
        unsafe { !LLVMIsAConstantFP(*instruction).is_null() }
    }
}

/// Return instruction matcher to check if current value is a constants number
#[derive(Clone)]
pub struct ConstNumberMatcher;

impl Matcher<LLVMValueRef> for ConstNumberMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, instruction: &LLVMValueRef) -> bool {
        unsafe {
            !LLVMIsAConstantInt(*instruction).is_null()
                || !LLVMIsAConstantFP(*instruction).is_null()
        }
    }
}
