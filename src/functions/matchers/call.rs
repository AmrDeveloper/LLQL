use std::collections::HashMap;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::call::CallInstMatcher;

#[inline(always)]
pub fn register_call_inst_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_call", match_call_inst);
}

#[inline(always)]
pub fn register_call_inst_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert("m_call", Signature::with_return(Box::new(InstMatcherType)));
}

fn match_call_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(CallInstMatcher::create_call());
    Box::new(InstMatcherValue { matcher })
}
