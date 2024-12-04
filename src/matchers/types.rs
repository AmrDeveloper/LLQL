use inkwell::llvm_sys;
use llvm_sys::core::LLVMGetArrayLength;
use llvm_sys::core::LLVMGetElementType;
use llvm_sys::core::LLVMGetIntTypeWidth;
use llvm_sys::core::LLVMGetTypeKind;
use llvm_sys::core::LLVMGetVectorSize;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::LLVMTypeKind;

use super::TypeMatcher;

/// Void Type Matcher used to match against any LLVM Void Type
#[derive(Clone)]
pub struct VoidTypeMatcher;

impl TypeMatcher for VoidTypeMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, llvm_type: LLVMTypeRef) -> bool {
        unsafe {
            let kind = LLVMGetTypeKind(llvm_type);
            kind == LLVMTypeKind::LLVMVoidTypeKind
        }
    }
}

/// Int Type Matcher used to match against LLVM Integer Type with specific size
#[derive(Clone)]
pub enum IntTypeSize {
    Size1,
    Size8,
    Size16,
    Size32,
    Size64,
}

/// Variant of available int type sizes
#[derive(Clone)]
pub struct IntTypeMatcher {
    pub size: IntTypeSize,
}

impl TypeMatcher for IntTypeMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, llvm_type: LLVMTypeRef) -> bool {
        unsafe {
            let kind = LLVMGetTypeKind(llvm_type);
            if kind == LLVMTypeKind::LLVMIntegerTypeKind {
                let type_width = LLVMGetIntTypeWidth(llvm_type);
                return match self.size {
                    IntTypeSize::Size1 => type_width == 1,
                    IntTypeSize::Size8 => type_width == 8,
                    IntTypeSize::Size16 => type_width == 16,
                    IntTypeSize::Size32 => type_width == 32,
                    IntTypeSize::Size64 => type_width == 64,
                };
            }
        }
        false
    }
}

/// Variant of available float type sizes
#[derive(Clone)]
pub enum FloatTypeSize {
    Size32,
    Size64,
}

/// Float Type Matcher used to match against LLVM Float Type with specific size
#[derive(Clone)]
pub struct FloatTypeMatcher {
    pub size: FloatTypeSize,
}

impl TypeMatcher for FloatTypeMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, llvm_type: LLVMTypeRef) -> bool {
        unsafe {
            let kind = LLVMGetTypeKind(llvm_type);
            match self.size {
                FloatTypeSize::Size32 => kind == LLVMTypeKind::LLVMFloatTypeKind,
                FloatTypeSize::Size64 => kind == LLVMTypeKind::LLVMDoubleTypeKind,
            }
        }
    }
}

/// Ha;f Type Matcher used to match against LLVM Half Type
#[derive(Clone)]
pub struct HalfTypeMatcher;

impl TypeMatcher for HalfTypeMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, llvm_type: LLVMTypeRef) -> bool {
        unsafe { LLVMGetTypeKind(llvm_type) == LLVMTypeKind::LLVMHalfTypeKind }
    }
}

/// Pointer Type Matcher used to match against LLVM Pointer Type
#[derive(Clone)]

pub struct PointerTypeMatcher;

impl TypeMatcher for PointerTypeMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, llvm_type: LLVMTypeRef) -> bool {
        unsafe {
            let kind = LLVMGetTypeKind(llvm_type);
            kind == LLVMTypeKind::LLVMPointerTypeKind
        }
    }
}

/// Array Type Matcher used to match against LLVM Array Type with specific base element type and size
#[derive(Clone)]
pub struct ArrayTypeMatcher {
    pub base_matcher: Box<dyn TypeMatcher>,
    pub length: Option<u32>,
}

impl TypeMatcher for ArrayTypeMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, llvm_type: LLVMTypeRef) -> bool {
        unsafe {
            let kind = LLVMGetTypeKind(llvm_type);
            if kind == LLVMTypeKind::LLVMArrayTypeKind {
                let element_type = LLVMGetElementType(llvm_type);

                let is_base_matches = self.base_matcher.is_match(element_type);

                if !is_base_matches {
                    return false;
                }

                if let Some(target_length) = self.length {
                    let length = LLVMGetArrayLength(llvm_type);
                    return target_length == length;
                }

                return true;
            }
        }
        false
    }
}

/// Vector Type Matcher used to match against LLVM Vector Type with specific base element type and size
#[derive(Clone)]
pub struct VectorTypeMatcher {
    pub base_matcher: Box<dyn TypeMatcher>,
    pub length: Option<u32>,
}

impl TypeMatcher for VectorTypeMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, llvm_type: LLVMTypeRef) -> bool {
        unsafe {
            let kind = LLVMGetTypeKind(llvm_type);
            if kind == LLVMTypeKind::LLVMVectorTypeKind {
                let element_type = LLVMGetElementType(llvm_type);
                let is_base_matches = self.base_matcher.is_match(element_type);
                if !is_base_matches {
                    return false;
                }

                if let Some(target_length) = self.length {
                    let length = LLVMGetVectorSize(llvm_type);
                    return target_length == length;
                }

                return true;
            }
        }
        false
    }
}

/// Scalable Vector Type Matcher used to match against LLVM scalable Vector Type
#[derive(Clone)]
pub struct ScalableVectorTypeMatcher;

impl TypeMatcher for ScalableVectorTypeMatcher {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn is_match(&self, llvm_type: LLVMTypeRef) -> bool {
        unsafe {
            let kind = LLVMGetTypeKind(llvm_type);
            kind == LLVMTypeKind::LLVMScalableVectorTypeKind
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::matchers::AnyTypeMatcher;

    use super::*;
    use llvm_sys::core::LLVMArrayType;
    use llvm_sys::core::LLVMContextCreate;
    use llvm_sys::core::LLVMDoubleTypeInContext;
    use llvm_sys::core::LLVMFloatTypeInContext;
    use llvm_sys::core::LLVMInt16TypeInContext;
    use llvm_sys::core::LLVMInt32TypeInContext;
    use llvm_sys::core::LLVMInt64TypeInContext;
    use llvm_sys::core::LLVMInt8TypeInContext;
    use llvm_sys::core::LLVMPointerType;
    use llvm_sys::core::LLVMVectorType;
    use llvm_sys::core::LLVMVoidType;

    #[test]
    fn test_any_type_matcher() {
        let context = unsafe { LLVMContextCreate() };

        let matcher = AnyTypeMatcher;

        let voidt = unsafe { LLVMVoidType() };
        let i64t = unsafe { LLVMInt64TypeInContext(context) };

        assert!(matcher.is_match(voidt));
        assert!(matcher.is_match(i64t));
    }

    #[test]
    fn test_void_type_matcher() {
        let context = unsafe { LLVMContextCreate() };

        let voidt = unsafe { LLVMVoidType() };
        let i64t = unsafe { LLVMInt64TypeInContext(context) };

        let matcher = VoidTypeMatcher;

        assert!(matcher.is_match(voidt));
        assert!(!matcher.is_match(i64t));
    }

    #[test]
    fn test_int_type_matcher() {
        let context = unsafe { LLVMContextCreate() };

        let voidt = unsafe { LLVMVoidType() };
        let i8t = unsafe { LLVMInt8TypeInContext(context) };
        let i16t = unsafe { LLVMInt16TypeInContext(context) };
        let i32t = unsafe { LLVMInt32TypeInContext(context) };
        let i64t = unsafe { LLVMInt64TypeInContext(context) };
        let f32t = unsafe { LLVMFloatTypeInContext(context) };
        let f64t = unsafe { LLVMDoubleTypeInContext(context) };

        let i8_matcher: IntTypeMatcher = IntTypeMatcher {
            size: IntTypeSize::Size8,
        };

        let i16_matcher: IntTypeMatcher = IntTypeMatcher {
            size: IntTypeSize::Size16,
        };

        let i32_matcher: IntTypeMatcher = IntTypeMatcher {
            size: IntTypeSize::Size32,
        };

        let i64_matcher: IntTypeMatcher = IntTypeMatcher {
            size: IntTypeSize::Size64,
        };

        assert!(!i8_matcher.is_match(voidt));
        assert!(i8_matcher.is_match(i8t));
        assert!(!i8_matcher.is_match(i16t));
        assert!(!i8_matcher.is_match(i32t));
        assert!(!i8_matcher.is_match(i64t));
        assert!(!i8_matcher.is_match(f32t));
        assert!(!i8_matcher.is_match(f64t));

        assert!(!i16_matcher.is_match(voidt));
        assert!(!i16_matcher.is_match(i8t));
        assert!(i16_matcher.is_match(i16t));
        assert!(!i16_matcher.is_match(i32t));
        assert!(!i16_matcher.is_match(i64t));
        assert!(!i16_matcher.is_match(f32t));
        assert!(!i16_matcher.is_match(f64t));

        assert!(!i32_matcher.is_match(voidt));
        assert!(!i32_matcher.is_match(i8t));
        assert!(!i32_matcher.is_match(i16t));
        assert!(i32_matcher.is_match(i32t));
        assert!(!i32_matcher.is_match(i64t));
        assert!(!i32_matcher.is_match(f32t));
        assert!(!i32_matcher.is_match(f64t));

        assert!(!i64_matcher.is_match(voidt));
        assert!(!i64_matcher.is_match(i8t));
        assert!(!i64_matcher.is_match(i16t));
        assert!(!i64_matcher.is_match(i32t));
        assert!(i64_matcher.is_match(i64t));
        assert!(!i64_matcher.is_match(f32t));
        assert!(!i64_matcher.is_match(f64t));
    }

    #[test]
    fn test_float_type_matcher() {
        let context = unsafe { LLVMContextCreate() };

        let voidt = unsafe { LLVMVoidType() };
        let i32t = unsafe { LLVMInt32TypeInContext(context) };
        let i64t = unsafe { LLVMInt64TypeInContext(context) };
        let f32t = unsafe { LLVMFloatTypeInContext(context) };
        let f64t = unsafe { LLVMDoubleTypeInContext(context) };

        let f32_matcher = FloatTypeMatcher {
            size: FloatTypeSize::Size32,
        };

        let f64_matcher = FloatTypeMatcher {
            size: FloatTypeSize::Size64,
        };

        assert!(!f32_matcher.is_match(voidt));
        assert!(!f32_matcher.is_match(i32t));
        assert!(!f32_matcher.is_match(i64t));
        assert!(f32_matcher.is_match(f32t));
        assert!(!f32_matcher.is_match(f64t));

        assert!(!f64_matcher.is_match(voidt));
        assert!(!f64_matcher.is_match(i32t));
        assert!(!f64_matcher.is_match(i64t));
        assert!(!f64_matcher.is_match(f32t));
        assert!(f64_matcher.is_match(f64t));
    }

    #[test]
    fn test_pointer_type_matcher() {
        let context = unsafe { LLVMContextCreate() };

        let voidt = unsafe { LLVMVoidType() };
        let void_ptr_ty = unsafe { LLVMPointerType(voidt, 0) };

        let i32t = unsafe { LLVMInt32TypeInContext(context) };
        let i32t_ptr_ty = unsafe { LLVMPointerType(i32t, 0) };

        let i64t = unsafe { LLVMInt64TypeInContext(context) };
        let i64t_ptr_ty = unsafe { LLVMPointerType(i64t, 0) };

        let f32t = unsafe { LLVMFloatTypeInContext(context) };
        let f32t_ptr_ty = unsafe { LLVMPointerType(f32t, 0) };

        let f64t = unsafe { LLVMDoubleTypeInContext(context) };
        let f64t_ptr_ty = unsafe { LLVMPointerType(f64t, 0) };

        let pointer_matcher = PointerTypeMatcher;

        assert!(!pointer_matcher.is_match(voidt));
        assert!(pointer_matcher.is_match(void_ptr_ty));
        assert!(!pointer_matcher.is_match(i32t));
        assert!(pointer_matcher.is_match(i32t_ptr_ty));
        assert!(!pointer_matcher.is_match(i64t));
        assert!(pointer_matcher.is_match(i64t_ptr_ty));
        assert!(!pointer_matcher.is_match(f32t));
        assert!(pointer_matcher.is_match(f32t_ptr_ty));
        assert!(!pointer_matcher.is_match(f64t));
        assert!(pointer_matcher.is_match(f64t_ptr_ty));
    }

    #[test]
    fn test_array_type_matcher() {
        let context = unsafe { LLVMContextCreate() };

        let i32t = unsafe { LLVMInt32TypeInContext(context) };
        let i32t_array_10_n = unsafe { LLVMArrayType(i32t, 10) };
        let i32t_array_20_n = unsafe { LLVMArrayType(i32t, 20) };

        let i64t = unsafe { LLVMInt64TypeInContext(context) };
        let i64t_array_10_n = unsafe { LLVMArrayType(i64t, 10) };
        let i64t_array_20_n = unsafe { LLVMArrayType(i64t, 20) };

        let i32_matcher = Box::new(IntTypeMatcher {
            size: IntTypeSize::Size32,
        });

        let un_sized_i32_array_matcher = ArrayTypeMatcher {
            base_matcher: i32_matcher,
            length: None,
        };

        let i64_matcher = Box::new(IntTypeMatcher {
            size: IntTypeSize::Size64,
        });

        let sized_i64_array_matcher = ArrayTypeMatcher {
            base_matcher: i64_matcher,
            length: Some(10),
        };

        assert!(un_sized_i32_array_matcher.is_match(i32t_array_10_n));
        assert!(un_sized_i32_array_matcher.is_match(i32t_array_20_n));
        assert!(!un_sized_i32_array_matcher.is_match(i64t_array_10_n));
        assert!(!un_sized_i32_array_matcher.is_match(i64t_array_20_n));

        assert!(!sized_i64_array_matcher.is_match(i32t_array_10_n));
        assert!(!sized_i64_array_matcher.is_match(i32t_array_20_n));
        assert!(sized_i64_array_matcher.is_match(i64t_array_10_n));
        assert!(!sized_i64_array_matcher.is_match(i64t_array_20_n));
    }

    #[test]
    fn test_vector_type_matcher() {
        let context = unsafe { LLVMContextCreate() };

        let i32t = unsafe { LLVMInt32TypeInContext(context) };
        let i32t_vec_10_n = unsafe { LLVMVectorType(i32t, 10) };
        let i32t_vec_20_n = unsafe { LLVMVectorType(i32t, 20) };

        let i64t = unsafe { LLVMInt64TypeInContext(context) };
        let i64t_vec_10_n = unsafe { LLVMVectorType(i64t, 10) };
        let i64t_vec_20_n = unsafe { LLVMVectorType(i64t, 20) };

        let i32_matcher = Box::new(IntTypeMatcher {
            size: IntTypeSize::Size32,
        });

        let un_sized_i32_vec_matcher = VectorTypeMatcher {
            base_matcher: i32_matcher,
            length: None,
        };

        let i64_matcher = Box::new(IntTypeMatcher {
            size: IntTypeSize::Size64,
        });

        let sized_i64_vec_matcher = VectorTypeMatcher {
            base_matcher: i64_matcher,
            length: Some(10),
        };

        assert!(un_sized_i32_vec_matcher.is_match(i32t_vec_10_n));
        assert!(un_sized_i32_vec_matcher.is_match(i32t_vec_20_n));
        assert!(!un_sized_i32_vec_matcher.is_match(i64t_vec_10_n));
        assert!(!un_sized_i32_vec_matcher.is_match(i64t_vec_20_n));

        assert!(!sized_i64_vec_matcher.is_match(i32t_vec_10_n));
        assert!(!sized_i64_vec_matcher.is_match(i32t_vec_20_n));
        assert!(sized_i64_vec_matcher.is_match(i64t_vec_10_n));
        assert!(!sized_i64_vec_matcher.is_match(i64t_vec_20_n));
    }
}
