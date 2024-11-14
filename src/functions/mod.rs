use std::collections::HashMap;
use std::sync::OnceLock;

use gitql_ast::types::optional::OptionType;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;
use gitql_std::function::standard_function_signatures;
use gitql_std::function::standard_functions;
use matchers::arithmetic::register_arithmetic_matchers_function_signatures;
use matchers::arithmetic::register_arithmetic_matchers_functions;
use matchers::constants::register_constants_matchers_function_signatures;
use matchers::constants::register_constants_matchers_functions;
use matchers::fcmp::register_float_comparisons_matchers_function_signatures;
use matchers::fcmp::register_float_comparisons_matchers_functions;
use matchers::icmp::register_int_comparisons_matchers_function_signatures;
use matchers::icmp::register_int_comparisons_matchers_functions;
use matchers::other::register_other_inst_matchers_function_signatures;
use matchers::other::register_other_inst_matchers_functions;
use matchers::types::register_type_matchers_function_signatures;
use matchers::types::register_type_matchers_functions;
use matchers::usage::register_usage_matchers_function_signatures;
use matchers::usage::register_usage_matchers_functions;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::AnyInstMatcher;
use crate::matchers::InstMatcher;

pub(crate) mod matchers;

pub fn llvm_ir_functions() -> &'static HashMap<&'static str, Function> {
    static HASHMAP: OnceLock<HashMap<&'static str, Function>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = standard_functions().to_owned();
        register_type_matchers_functions(&mut map);
        register_arithmetic_matchers_functions(&mut map);
        register_int_comparisons_matchers_functions(&mut map);
        register_float_comparisons_matchers_functions(&mut map);
        register_usage_matchers_functions(&mut map);
        register_constants_matchers_functions(&mut map);
        register_other_inst_matchers_functions(&mut map);
        map
    })
}

pub fn llvm_ir_function_signatures() -> HashMap<&'static str, Signature> {
    let mut map = standard_function_signatures().to_owned();
    register_type_matchers_function_signatures(&mut map);
    register_arithmetic_matchers_function_signatures(&mut map);
    register_int_comparisons_matchers_function_signatures(&mut map);
    register_float_comparisons_matchers_function_signatures(&mut map);
    register_usage_matchers_function_signatures(&mut map);
    register_constants_matchers_function_signatures(&mut map);
    register_other_inst_matchers_function_signatures(&mut map);
    map
}

#[inline]
pub fn binary_matcher_signature() -> Signature {
    Signature {
        parameters: vec![
            Box::new(OptionType {
                base: Some(Box::new(InstMatcherType)),
            }),
            Box::new(OptionType {
                base: Some(Box::new(InstMatcherType)),
            }),
        ],
        return_type: Box::new(InstMatcherType),
    }
}

#[inline]
pub fn binary_matchers_sides(
    values: &[Box<dyn Value>],
) -> (Box<dyn InstMatcher>, Box<dyn InstMatcher>) {
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
