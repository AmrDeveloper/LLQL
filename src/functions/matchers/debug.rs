use std::collections::HashMap;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::debug::DebugInfoColumnMatcher;
use crate::matchers::debug::DebugInfoLineMatcher;

use gitql_ast::types::integer::IntType;
use gitql_core::signature::Signature;
use gitql_core::signature::StandardFunction;
use gitql_core::values::Value;

#[inline(always)]
pub fn register_debug_inst_matchers_functions(map: &mut HashMap<&'static str, StandardFunction>) {
    map.insert("m_dbg_line", match_debug_info_line);
    map.insert("m_dbg_column", match_debug_info_column);
}

#[inline(always)]
pub fn register_debug_inst_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert(
        "m_dbg_line",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_dbg_column",
        Signature {
            parameters: vec![Box::new(IntType)],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_debug_info_line(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let line = values[0].as_int().unwrap() as u32;
    let matcher = Box::new(DebugInfoLineMatcher { line });
    Box::new(InstMatcherValue { matcher })
}

fn match_debug_info_column(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let column = values[0].as_int().unwrap() as u32;
    let matcher = Box::new(DebugInfoColumnMatcher { column });
    Box::new(InstMatcherValue { matcher })
}
