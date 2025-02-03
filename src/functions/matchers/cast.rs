use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::cast::CastInstMatcher;

#[inline(always)]
pub fn register_cast_matchers_function(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_int_to_ptr", match_int_to_ptr_inst);
    map.insert("m_ptr_to_int", match_ptr_to_int_inst);
}

#[inline(always)]
pub fn register_cast_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_int_to_ptr",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_ptr_to_int",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_int_to_ptr_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_int_to_ptr());
    Box::new(InstMatcherValue { matcher })
}

fn match_ptr_to_int_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CastInstMatcher::create_ptr_to_int());
    Box::new(InstMatcherValue { matcher })
}
