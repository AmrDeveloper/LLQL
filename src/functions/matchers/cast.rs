use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::cast::CastInstMatcher;

#[inline(always)]
pub fn register_cast_matchers_function(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_zext", match_zext);
    map.insert("m_sext", match_sext);
    map.insert("m_int_to_ptr", match_int_to_ptr_inst);
    map.insert("m_ptr_to_int", match_ptr_to_int_inst);
    map.insert("m_bit_cast", match_bit_cast_inst);
    map.insert("m_addr_space_cast", match_addr_space_cast_inst);
}

#[inline(always)]
pub fn register_cast_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert("m_zext", Signature::with_return(Box::new(InstMatcherType)));

    map.insert("m_sext", Signature::with_return(Box::new(InstMatcherType)));

    map.insert(
        "m_int_to_ptr",
        Signature::with_return(Box::new(InstMatcherType)),
    );

    map.insert(
        "m_ptr_to_int",
        Signature::with_return(Box::new(InstMatcherType)),
    );

    map.insert(
        "m_bit_cast",
        Signature::with_return(Box::new(InstMatcherType)),
    );

    map.insert(
        "m_addr_space_cast",
        Signature::with_return(Box::new(InstMatcherType)),
    );
}

fn match_zext(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_zext());
    Box::new(InstMatcherValue { matcher })
}

fn match_sext(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_sext());
    Box::new(InstMatcherValue { matcher })
}

fn match_int_to_ptr_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_int_to_ptr());
    Box::new(InstMatcherValue { matcher })
}

fn match_ptr_to_int_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_ptr_to_int());
    Box::new(InstMatcherValue { matcher })
}

fn match_bit_cast_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_bit_cast());
    Box::new(InstMatcherValue { matcher })
}

fn match_addr_space_cast_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_addr_space_cast());
    Box::new(InstMatcherValue { matcher })
}
