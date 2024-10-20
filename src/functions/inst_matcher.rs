use gitql_ast::types::boolean::BoolType;
use gitql_ast::types::optional::OptionType;
use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;
use std::collections::HashMap;

use crate::ir::types::InstMatcherType;
use crate::ir::types::LLVMInstType;
use crate::ir::types::TypeMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::ir::values::LLVMInstValue;
use crate::ir::values::TypeMatcherValue;
use crate::matchers::instruction_matcher::ReturnInstMatcher;
use crate::matchers::type_matcher::AnyTypeMatcher;

#[inline(always)]
pub fn register_inst_matchers_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("m_inst", match_inst);
    map.insert("m_return", match_return_inst);
}

#[inline(always)]
pub fn register_inst_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_inst",
        Signature {
            parameters: vec![Box::new(LLVMInstType), Box::new(InstMatcherType)],
            return_type: Box::new(BoolType),
        },
    );

    map.insert(
        "m_return",
        Signature {
            parameters: vec![Box::new(OptionType {
                base: Some(Box::new(TypeMatcherType)),
            })],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let inst = values[0].as_any().downcast_ref::<LLVMInstValue>().unwrap();
    let matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap();

    let is_match = matcher.matcher.is_match(inst.llvm_value);
    Box::new(BoolValue { value: is_match })
}

fn match_return_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let type_matcher = if values.is_empty() {
        Box::new(AnyTypeMatcher)
    } else {
        values[0]
            .as_any()
            .downcast_ref::<TypeMatcherValue>()
            .unwrap()
            .matcher
            .clone()
    };

    Box::new(InstMatcherValue {
        matcher: Box::new(ReturnInstMatcher { type_matcher }),
    })
}
