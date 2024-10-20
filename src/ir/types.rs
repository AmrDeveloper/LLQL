use std::any::Any;

use gitql_ast::types::base::DataType;

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
