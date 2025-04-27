use std::collections::HashMap;

use gitql_ast::types::integer::IntType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::Value;

use crate::functions::matcher_signature_without_parameters;
use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::constants::ConstFloatMatcher;
use crate::matchers::constants::ConstIntMatcher;
use crate::matchers::constants::ConstNumberMatcher;
use crate::matchers::constants::ConstPointerNullMatcher;

#[inline(always)]
pub fn register_constants_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_const_num", match_const_num_inst);
    map.insert("m_const_int", match_const_int_inst);
    map.insert("m_zero", match_const_zero_inst);
    map.insert("m_one", match_const_one_inst);
    map.insert("m_power2", match_const_power_of_two_inst);
    map.insert("m_specific_int", match_const_specific_int_inst);
    map.insert("m_range_int", match_range_int_inst);

    map.insert("m_const_fp", match_const_fp_inst);
    map.insert("m_const_null", match_const_null_inst);
}

#[inline(always)]
pub fn register_constants_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert("m_const_num", matcher_signature_without_parameters());
    map.insert("m_const_int", matcher_signature_without_parameters());
    map.insert("m_zero", matcher_signature_without_parameters());
    map.insert("m_one", matcher_signature_without_parameters());
    map.insert("m_power2", matcher_signature_without_parameters());
    map.insert(
        "m_specific_int",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_range_int",
        Signature {
            parameters: vec![Box::new(IntType), Box::new(IntType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert("m_const_fp", matcher_signature_without_parameters());
    map.insert("m_const_null", matcher_signature_without_parameters());
}

fn match_const_num_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(ConstNumberMatcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_const_int_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(ConstIntMatcher::create_const_int());
    Box::new(InstMatcherValue { matcher })
}

fn match_const_zero_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(ConstIntMatcher::create_zero());
    Box::new(InstMatcherValue { matcher })
}

fn match_const_one_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(ConstIntMatcher::create_one());
    Box::new(InstMatcherValue { matcher })
}

fn match_const_power_of_two_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(ConstIntMatcher::create_power_of_two());
    Box::new(InstMatcherValue { matcher })
}

fn match_const_specific_int_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int_value = values[0].as_int().unwrap();
    let matcher = Box::new(ConstIntMatcher::create_specific_int(int_value));
    Box::new(InstMatcherValue { matcher })
}

fn match_range_int_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let range_start = values[0].as_int().unwrap();
    let range_end = values[1].as_int().unwrap();
    let matcher = Box::new(ConstIntMatcher::create_range_int(range_start, range_end));
    Box::new(InstMatcherValue { matcher })
}

fn match_const_fp_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(ConstFloatMatcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_const_null_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(ConstPointerNullMatcher);
    Box::new(InstMatcherValue { matcher })
}
