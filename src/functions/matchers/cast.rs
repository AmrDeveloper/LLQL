use std::collections::HashMap;

use gitql_ast::types::optional::OptionType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;

use crate::functions::single_optional_matcher_value;
use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::cast::CastInstMatcher;

#[inline(always)]
pub fn register_cast_matchers_function(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_cast", match_any_cast);
    map.insert("m_trunc", match_trunc);
    map.insert("m_fp_to_ui", match_fp_to_ui);
    map.insert("m_fp_to_si", match_fp_to_si);
    map.insert("m_fp_trunc", match_fp_trunc);
    map.insert("m_int_to_ptr", match_int_to_ptr);
    map.insert("m_ptr_to_int", match_ptr_to_int);
    map.insert("m_ext", match_ext);
    map.insert("m_fpext", match_fp_ext);
    map.insert("m_zext", match_zext);
    map.insert("m_sext", match_sext);
    map.insert("m_bit_cast", match_bit_cast);
    map.insert("m_addr_space_cast", match_addr_space_cast);
}

#[inline(always)]
pub fn register_cast_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert("m_cast", cast_function_signature());
    map.insert("m_trunc", cast_function_signature());
    map.insert("m_fp_to_ui", cast_function_signature());
    map.insert("m_fp_to_si", cast_function_signature());
    map.insert("m_fp_trunc", cast_function_signature());
    map.insert("m_int_to_ptr", cast_function_signature());
    map.insert("m_ptr_to_int", cast_function_signature());
    map.insert("m_ext", cast_function_signature());
    map.insert("m_fpext", cast_function_signature());
    map.insert("m_zext", cast_function_signature());
    map.insert("m_sext", cast_function_signature());
    map.insert("m_bit_cast", cast_function_signature());
    map.insert("m_addr_space_cast", cast_function_signature());
}

#[inline(always)]
fn cast_function_signature() -> Signature {
    Signature::with_return(Box::new(InstMatcherType))
        .add_parameter(Box::new(OptionType::new(Some(Box::new(InstMatcherType)))))
}

fn match_any_cast(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_any(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_trunc(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_trunc(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_fp_to_ui(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_fp_to_ui(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_fp_to_si(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_fp_to_si(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_fp_trunc(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_fp_trunc(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_int_to_ptr(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_int_to_ptr(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_ptr_to_int(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_ptr_to_int(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_ext(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_ext(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_fp_ext(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_fpext(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_zext(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_zext(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_sext(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_sext(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_bit_cast(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_bit_cast(value_matcher));
    Box::new(InstMatcherValue { matcher })
}

fn match_addr_space_cast(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let value_matcher = single_optional_matcher_value(values);
    let matcher = Box::new(CastInstMatcher::create_addr_space_cast(value_matcher));
    Box::new(InstMatcherValue { matcher })
}
