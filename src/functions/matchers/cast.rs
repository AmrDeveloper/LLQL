use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::cast::CastInstMatcher;

#[inline(always)]
pub fn register_cast_matchers_function(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_trunc", match_trunc);
    map.insert("m_fp_to_ui", match_fp_to_ui);
    map.insert("m_fp_to_si", match_fp_to_si);
    map.insert("m_fp_trunc", match_fp_trunc);
    map.insert("m_int_to_ptr", match_int_to_ptr);
    map.insert("m_ptr_to_int", match_ptr_to_int);
    map.insert("m_zext", match_zext);
    map.insert("m_sext", match_sext);
    map.insert("m_bit_cast", match_bit_cast);
    map.insert("m_addr_space_cast", match_addr_space_cast);
}

#[inline(always)]
pub fn register_cast_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert("m_trunc", cast_function_signature());
    map.insert("m_fp_to_ui", cast_function_signature());
    map.insert("m_fp_to_si", cast_function_signature());
    map.insert("m_fp_trunc", cast_function_signature());
    map.insert("m_int_to_ptr", cast_function_signature());
    map.insert("m_ptr_to_int", cast_function_signature());
    map.insert("m_zext", cast_function_signature());
    map.insert("m_sext", cast_function_signature());
    map.insert("m_bit_cast", cast_function_signature());
    map.insert("m_addr_space_cast", cast_function_signature());
}

fn cast_function_signature() -> Signature {
    Signature::with_return(Box::new(InstMatcherType))
}

fn match_trunc(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_trunc());
    Box::new(InstMatcherValue { matcher })
}

fn match_fp_to_ui(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_fp_to_ui());
    Box::new(InstMatcherValue { matcher })
}

fn match_fp_to_si(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_fp_to_si());
    Box::new(InstMatcherValue { matcher })
}

fn match_fp_trunc(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_fp_trunc());
    Box::new(InstMatcherValue { matcher })
}

fn match_int_to_ptr(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_int_to_ptr());
    Box::new(InstMatcherValue { matcher })
}

fn match_ptr_to_int(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_ptr_to_int());
    Box::new(InstMatcherValue { matcher })
}

fn match_zext(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_zext());
    Box::new(InstMatcherValue { matcher })
}

fn match_sext(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_sext());
    Box::new(InstMatcherValue { matcher })
}

fn match_bit_cast(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_bit_cast());
    Box::new(InstMatcherValue { matcher })
}

fn match_addr_space_cast(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_addr_space_cast());
    Box::new(InstMatcherValue { matcher })
}
