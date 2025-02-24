use std::any::Any;

use gitql_ast::types::DataType;

#[derive(Clone)]
pub struct LLVMInstType;

impl DataType for LLVMInstType {
    fn literal(&self) -> String {
        "LLVMValue".to_string()
    }

    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let self_type: Box<dyn DataType> = Box::new(LLVMInstType);
        other.is_any()
            || other.is_variant_contains(&self_type)
            || other.as_any().downcast_ref::<LLVMInstType>().is_some()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct LLVMDataType;

impl DataType for LLVMDataType {
    fn literal(&self) -> String {
        "LLVMType".to_string()
    }

    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let self_type: Box<dyn DataType> = Box::new(LLVMDataType);
        other.is_any()
            || other.is_variant_contains(&self_type)
            || other.as_any().downcast_ref::<LLVMDataType>().is_some()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct InstMatcherType;

impl DataType for InstMatcherType {
    fn literal(&self) -> String {
        "InstMatcher".to_string()
    }

    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let self_type: Box<dyn DataType> = Box::new(InstMatcherType);
        other.is_any()
            || other.is_variant_contains(&self_type)
            || other.as_any().downcast_ref::<InstMatcherType>().is_some()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn can_perform_bang_op(&self) -> bool {
        true
    }

    fn bang_op_result_type(&self) -> Box<dyn DataType> {
        Box::new(InstMatcherType)
    }

    fn can_perform_logical_or_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(InstMatcherType)]
    }

    fn logical_or_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(InstMatcherType)
    }

    fn can_perform_logical_and_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(InstMatcherType)]
    }

    fn logical_and_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(InstMatcherType)
    }

    fn can_perform_logical_xor_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(InstMatcherType)]
    }

    fn logical_xor_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(InstMatcherType)
    }
}

#[derive(Clone)]
pub struct TypeMatcherType;

impl DataType for TypeMatcherType {
    fn literal(&self) -> String {
        "TypeMatcherType".to_string()
    }

    #[allow(clippy::borrowed_box)]
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let self_type: Box<dyn DataType> = Box::new(TypeMatcherType);
        other.is_any()
            || other.is_variant_contains(&self_type)
            || other.as_any().downcast_ref::<TypeMatcherType>().is_some()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
