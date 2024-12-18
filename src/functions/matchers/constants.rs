use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::constants::ConstFloatMatcher;
use crate::matchers::constants::ConstIntMatcher;
use crate::matchers::constants::ConstPointerNullMatcher;

#[inline(always)]
pub fn register_constants_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_const_int", match_const_int_inst);
    map.insert("m_const_fp", match_const_fp_inst);
    map.insert("m_const_null", match_const_null_inst);
}

#[inline(always)]
pub fn register_constants_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_const_int",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_const_fp",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_const_null",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_const_int_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(InstMatcherValue {
        matcher: Box::new(ConstIntMatcher),
    })
}

fn match_const_fp_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(InstMatcherValue {
        matcher: Box::new(ConstFloatMatcher),
    })
}

fn match_const_null_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(InstMatcherValue {
        matcher: Box::new(ConstPointerNullMatcher),
    })
}
