use std::any::Any;
use std::cmp::Ordering;
use std::ffi::CString;

use gitql_ast::types::base::DataType;
use gitql_core::values::base::Value;
use inkwell::llvm_sys;
use llvm_sys::core::LLVMPrintValueToString;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;

use crate::matchers::combine::CombineBinaryInstMatcher;
use crate::matchers::combine::CombineUnaryInstMatcher;
use crate::matchers::InstMatcher;
use crate::matchers::TypeMatcher;

use super::types::InstMatcherType;
use super::types::LLVMDataType;
use super::types::LLVMInstType;
use super::types::TypeMatcherType;

#[derive(Clone)]
pub struct LLVMInstValue {
    pub llvm_value: LLVMValueRef,
}

impl Value for LLVMInstValue {
    fn literal(&self) -> String {
        unsafe {
            let instruction_str = LLVMPrintValueToString(self.llvm_value);
            CString::from_raw(instruction_str)
                .into_string()
                .unwrap_or("LLVMInstValue".to_string())
        }
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_inst) = other.as_any().downcast_ref::<LLVMInstValue>() {
            return self.llvm_value.eq(&other_inst.llvm_value);
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_inst) = other.as_any().downcast_ref::<LLVMInstValue>() {
            if self.llvm_value.eq(&other_inst.llvm_value) {
                return Some(Ordering::Equal);
            }
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(LLVMInstType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct LLVMTypeValue {
    pub llvm_type: LLVMTypeRef,
}

impl Value for LLVMTypeValue {
    fn literal(&self) -> String {
        format!("{:?}", self.llvm_type)
    }

    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_inst) = other.as_any().downcast_ref::<LLVMTypeValue>() {
            return self.llvm_type.eq(&other_inst.llvm_type);
        }
        false
    }

    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        if let Some(other_type) = other.as_any().downcast_ref::<LLVMTypeValue>() {
            if self.llvm_type.eq(&other_type.llvm_type) {
                return Some(Ordering::Equal);
            }
        }
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(LLVMDataType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct InstMatcherValue {
    pub matcher: Box<dyn InstMatcher>,
}

impl Value for InstMatcherValue {
    fn literal(&self) -> String {
        "InstMatcherValue".to_string()
    }

    fn equals(&self, _other: &Box<dyn Value>) -> bool {
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(InstMatcherType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn bang_op(&self) -> Result<Box<dyn Value>, String> {
        let matcher = Box::new(CombineUnaryInstMatcher::create_not(self.matcher.clone()));
        Ok(Box::new(InstMatcherValue { matcher }))
    }

    fn logical_or_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        let lhs = self.matcher.clone();
        let rhs = other
            .as_any()
            .downcast_ref::<InstMatcherValue>()
            .unwrap()
            .matcher
            .to_owned();
        let matcher = Box::new(CombineBinaryInstMatcher::create_or(lhs, rhs));
        Ok(Box::new(InstMatcherValue { matcher }))
    }

    fn logical_and_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        let lhs = self.matcher.clone();
        let rhs = other
            .as_any()
            .downcast_ref::<InstMatcherValue>()
            .unwrap()
            .matcher
            .to_owned();
        let matcher = Box::new(CombineBinaryInstMatcher::create_and(lhs, rhs));
        Ok(Box::new(InstMatcherValue { matcher }))
    }

    fn logical_xor_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        let lhs = self.matcher.clone();
        let rhs = other
            .as_any()
            .downcast_ref::<InstMatcherValue>()
            .unwrap()
            .matcher
            .to_owned();
        let matcher = Box::new(CombineBinaryInstMatcher::create_xor(lhs, rhs));
        Ok(Box::new(InstMatcherValue { matcher }))
    }
}

#[derive(Clone)]
pub struct TypeMatcherValue {
    pub matcher: Box<dyn TypeMatcher>,
}

impl Value for TypeMatcherValue {
    fn literal(&self) -> String {
        "TypeMatcherValue".to_string()
    }

    fn equals(&self, _other: &Box<dyn Value>) -> bool {
        false
    }

    fn compare(&self, _other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(TypeMatcherType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
