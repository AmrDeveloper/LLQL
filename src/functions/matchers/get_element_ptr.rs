use std::collections::HashMap;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::get_element_ptr::GetElementPtrMatcher;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::Value;

#[inline(always)]
pub fn register_get_element_ptr_inst_matchers_functions(
    map: &mut HashMap<&'static str, StandardFunction>,
) {
    map.insert("m_get_element_ptr", match_get_element_ptr_inst);
}

#[inline(always)]
pub fn register_get_element_ptr_inst_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert(
        "m_get_element_ptr",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_get_element_ptr_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(GetElementPtrMatcher);
    Box::new(InstMatcherValue { matcher })
}
