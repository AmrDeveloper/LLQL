use dyn_clone::DynClone;
use inkwell::llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};

dyn_clone::clone_trait_object!(TypeMatcher);

/// Type matcher used to create matcher that check if rules match [`LLVMTypeRef`] or not
pub trait TypeMatcher: DynClone {
    /// Return true if this matcher match the [`LLVMTypeRef`]
    fn is_match(&self, llvm_type: LLVMTypeRef) -> bool;
}

/// Any Type Matcher used to match againts any [`LLVMTypeRef`]
#[derive(Clone)]
pub struct AnyTypeMatcher;

impl TypeMatcher for AnyTypeMatcher {
    fn is_match(&self, _llvm_type: LLVMTypeRef) -> bool {
        true
    }
}

dyn_clone::clone_trait_object!(InstMatcher);

/// Instruction matcher used to create matcher that check if rules match the instruction or not
pub trait InstMatcher: DynClone {
    fn is_match(&self, instruction: LLVMValueRef) -> bool;
}

/// Instruction Matcher to match to any instruction, used mostly as default matcher
#[derive(Clone)]
pub struct AnyInstMatcher;

impl InstMatcher for AnyInstMatcher {
    fn is_match(&self, _instruction: LLVMValueRef) -> bool {
        true
    }
}

pub mod arithmetic;
pub mod constants;
pub mod fcmp;
pub mod icmp;
pub mod other;
pub mod types;
pub mod usage;
