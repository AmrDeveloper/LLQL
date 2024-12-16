use std::collections::HashMap;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::exception::LandingPadInstMatcher;

use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;

#[inline(always)]
pub fn register_exception_inst_matchers_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("m_landingpad", match_landingpad_inst);
}

#[inline(always)]
pub fn register_exception_inst_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert(
        "m_landingpad",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_landingpad_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(InstMatcherValue {
        matcher: Box::new(LandingPadInstMatcher),
    })
}
