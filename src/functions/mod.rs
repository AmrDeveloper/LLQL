use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_ast::types::optional::OptionType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::Value;

use gitql_std::standard::standard_function_signatures;
use gitql_std::standard::standard_functions;
use inkwell::llvm_sys::prelude::LLVMValueRef;
use matchers::arithmetic::register_arithmetic_matchers_function_signatures;
use matchers::arithmetic::register_arithmetic_matchers_functions;
use matchers::binary::register_binary_inst_matchers_function_signatures;
use matchers::binary::register_binary_inst_matchers_functions;
use matchers::call::register_call_inst_matchers_function_signatures;
use matchers::call::register_call_inst_matchers_functions;
use matchers::cast::register_cast_matchers_function;
use matchers::cast::register_cast_matchers_function_signatures;
use matchers::combine::register_combine_matchers_function;
use matchers::combine::register_combine_matchers_function_signatures;
use matchers::constants::register_constants_matchers_function_signatures;
use matchers::constants::register_constants_matchers_functions;
use matchers::debug::register_debug_inst_matchers_function_signatures;
use matchers::debug::register_debug_inst_matchers_functions;
use matchers::exception::register_exception_inst_matchers_function_signatures;
use matchers::exception::register_exception_inst_matchers_functions;
use matchers::fcmp::register_float_comparisons_matchers_function_signatures;
use matchers::fcmp::register_float_comparisons_matchers_functions;
use matchers::icmp::register_int_comparisons_matchers_function_signatures;
use matchers::icmp::register_int_comparisons_matchers_functions;
use matchers::other::register_other_inst_matchers_function_signatures;
use matchers::other::register_other_inst_matchers_functions;
use matchers::shifts::register_shift_matchers_function_signatures;
use matchers::shifts::register_shift_matchers_functions;
use matchers::types::register_type_matchers_function_signatures;
use matchers::types::register_type_matchers_functions;
use matchers::usage::register_usage_matchers_function_signatures;
use matchers::usage::register_usage_matchers_functions;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::other::AnyInstMatcher;
use crate::matchers::Matcher;

pub(crate) mod matchers;

#[inline(always)]
pub fn llvm_ir_functions() -> &'static HashMap<&'static str, StandardFunction> {
    static HASHMAP: OnceLock<HashMap<&'static str, StandardFunction>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = standard_functions().to_owned();
        register_type_matchers_functions(&mut map);
        register_arithmetic_matchers_functions(&mut map);
        register_int_comparisons_matchers_functions(&mut map);
        register_float_comparisons_matchers_functions(&mut map);
        register_usage_matchers_functions(&mut map);
        register_constants_matchers_functions(&mut map);
        register_other_inst_matchers_functions(&mut map);
        register_shift_matchers_functions(&mut map);
        register_binary_inst_matchers_functions(&mut map);
        register_exception_inst_matchers_functions(&mut map);
        register_combine_matchers_function(&mut map);
        register_call_inst_matchers_functions(&mut map);
        register_cast_matchers_function(&mut map);
        register_debug_inst_matchers_functions(&mut map);
        map
    })
}

#[inline(always)]
pub fn llvm_ir_function_signatures() -> HashMap<&'static str, Signature> {
    let mut map = standard_function_signatures().to_owned();
    register_type_matchers_function_signatures(&mut map);
    register_arithmetic_matchers_function_signatures(&mut map);
    register_int_comparisons_matchers_function_signatures(&mut map);
    register_float_comparisons_matchers_function_signatures(&mut map);
    register_usage_matchers_function_signatures(&mut map);
    register_constants_matchers_function_signatures(&mut map);
    register_other_inst_matchers_function_signatures(&mut map);
    register_shift_matchers_function_signatures(&mut map);
    register_binary_inst_matchers_function_signatures(&mut map);
    register_exception_inst_matchers_function_signatures(&mut map);
    register_combine_matchers_function_signatures(&mut map);
    register_call_inst_matchers_function_signatures(&mut map);
    register_cast_matchers_function_signatures(&mut map);
    register_debug_inst_matchers_function_signatures(&mut map);
    map
}

#[inline]
pub fn binary_matcher_signature() -> Signature {
    Signature {
        parameters: vec![
            Box::new(OptionType::new(Some(Box::new(InstMatcherType)))),
            Box::new(OptionType::new(Some(Box::new(InstMatcherType)))),
        ],
        return_type: Box::new(InstMatcherType),
    }
}

#[inline]
pub fn matcher_signature_without_parameters() -> Signature {
    Signature::with_return(Box::new(InstMatcherType))
}

#[inline]
pub fn binary_matchers_sides(
    values: &[Box<dyn Value>],
) -> (
    Box<dyn Matcher<LLVMValueRef>>,
    Box<dyn Matcher<LLVMValueRef>>,
) {
    let args_len = values.len();

    let lhs_matcher = if args_len > 0 {
        values[0]
            .as_any()
            .downcast_ref::<InstMatcherValue>()
            .unwrap()
            .matcher
            .to_owned()
    } else {
        Box::new(AnyInstMatcher)
    };

    let rhs_matcher = if args_len > 1 {
        values[1]
            .as_any()
            .downcast_ref::<InstMatcherValue>()
            .unwrap()
            .matcher
            .to_owned()
    } else {
        Box::new(AnyInstMatcher)
    };

    (lhs_matcher, rhs_matcher)
}

#[inline]
pub fn single_optional_matcher_value(values: &[Box<dyn Value>]) -> Box<dyn Matcher<LLVMValueRef>> {
    if !values.is_empty() {
        values[0]
            .as_any()
            .downcast_ref::<InstMatcherValue>()
            .unwrap()
            .matcher
            .to_owned()
    } else {
        Box::new(AnyInstMatcher)
    }
}
