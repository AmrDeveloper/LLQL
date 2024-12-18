use std::collections::HashMap;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::exception::InvokeInstMatcher;
use crate::matchers::exception::LandingPadInstMatcher;

use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::base::Value;

#[inline(always)]
pub fn register_exception_inst_matchers_functions(
    map: &mut HashMap<&'static str, StandardFunction>,
) {
    map.insert("m_invoke", match_invoke_inst);
    map.insert("m_landingpad", match_landingpad_inst);
}

#[inline(always)]
pub fn register_exception_inst_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert(
        "m_invoke",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_landingpad",
        Signature {
            parameters: vec![],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_invoke_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(InvokeInstMatcher);
    Box::new(InstMatcherValue { matcher })
}

fn match_landingpad_inst(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let matcher = Box::new(LandingPadInstMatcher);
    Box::new(InstMatcherValue { matcher })
}
