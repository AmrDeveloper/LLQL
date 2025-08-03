use std::collections::HashMap;

use gitql_ast::types::optional::OptionType;
use gitql_ast::types::text::TextType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::call::CallInstMatcher;
use crate::matchers::call::IntrinsicInstMatcher;

#[inline(always)]
pub fn register_call_inst_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_call", match_call_inst);
    map.insert("m_intrinsic", match_intrinsic_inst);
}

#[inline(always)]
pub fn register_call_inst_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_call",
        Signature::with_return(Box::new(InstMatcherType))
            .add_parameter(Box::new(OptionType::new(Some(Box::new(TextType))))),
    );

    map.insert(
        "m_intrinsic",
        Signature::with_return(Box::new(InstMatcherType))
            .add_parameter(Box::new(OptionType::new(Some(Box::new(TextType))))),
    );
}

fn match_call_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let function_name = if values.is_empty() {
        None
    } else {
        values[0].as_text()
    };

    let matcher = Box::new(CallInstMatcher::create_call(function_name));
    Box::new(InstMatcherValue { matcher })
}

fn match_intrinsic_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let intrinsic_name = if values.is_empty() {
        None
    } else {
        values[0].as_text()
    };

    let matcher = Box::new(IntrinsicInstMatcher::create_call(intrinsic_name));
    Box::new(InstMatcherValue { matcher })
}
