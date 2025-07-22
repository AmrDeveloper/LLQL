use std::collections::HashMap;

use gitql_ast::types::text::TextType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::operand_bundle::OperandBundleNameMatcher;

#[inline(always)]
pub fn register_operand_bundle_inst_matchers_functions(
    map: &mut HashMap<&'static str, StandardFunction>,
) {
    map.insert("m_operand_bundle", match_operand_bundle);
}

#[inline(always)]
pub fn register_operand_bundle_inst_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert(
        "m_operand_bundle",
        Signature {
            parameters: vec![Box::new(TextType)],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_operand_bundle(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let expected_name = values[0].as_text().unwrap();
    let matcher = Box::new(OperandBundleNameMatcher::create(expected_name));
    Box::new(InstMatcherValue { matcher })
}
