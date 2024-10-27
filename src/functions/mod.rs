use std::{collections::HashMap, sync::OnceLock};

use gitql_ast::types::optional::OptionType;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;
use gitql_std::function::standard_function_signatures;
use gitql_std::function::standard_functions;
use inst_matcher::register_inst_matchers_function_signatures;
use inst_matcher::register_inst_matchers_functions;
use type_matcher::register_type_matchers_function_signatures;
use type_matcher::register_type_matchers_functions;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::instruction_matcher::AnyInstMatcher;
use crate::matchers::instruction_matcher::InstMatcher;

pub(crate) mod arithmetic_matchers;
pub(crate) mod fcmp_matchers;
pub(crate) mod icmp_matchers;
pub(crate) mod inst_matcher;
pub(crate) mod type_matcher;

pub fn llvm_ir_functions() -> &'static HashMap<&'static str, Function> {
    static HASHMAP: OnceLock<HashMap<&'static str, Function>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = standard_functions().to_owned();
        register_inst_matchers_functions(&mut map);
        register_type_matchers_functions(&mut map);
        map
    })
}

pub fn llvm_ir_function_signatures() -> HashMap<&'static str, Signature> {
    let mut map = standard_function_signatures().to_owned();
    register_inst_matchers_function_signatures(&mut map);
    register_type_matchers_function_signatures(&mut map);
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
