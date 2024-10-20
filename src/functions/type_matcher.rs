use std::collections::HashMap;

use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;

use crate::ir::types::TypeMatcherType;
use crate::ir::values::TypeMatcherValue;
use crate::matchers::type_matcher::IntTypeMatcher;
use crate::matchers::type_matcher::IntTypeSize;
use crate::matchers::type_matcher::VoidTypeMatcher;

#[inline(always)]
pub fn register_type_matchers_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("m_void", match_void);
    map.insert("m_int1", match_int1);
    map.insert("m_int8", match_int8);
    map.insert("m_int16", match_int16);
    map.insert("m_int32", match_int32);
    map.insert("m_int64", match_int64);
}

#[inline(always)]
pub fn register_type_matchers_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "m_void",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_int1",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_int8",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_int16",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_int32",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
    map.insert(
        "m_int64",
        Signature {
            parameters: vec![],
            return_type: Box::new(TypeMatcherType),
        },
    );
}

pub fn match_void(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    Box::new(TypeMatcherValue {
        matcher: Box::new(VoidTypeMatcher),
    })
}

pub fn match_int1(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = IntTypeMatcher {
        size: IntTypeSize::Size1,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

pub fn match_int8(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = IntTypeMatcher {
        size: IntTypeSize::Size8,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

pub fn match_int16(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = IntTypeMatcher {
        size: IntTypeSize::Size16,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

pub fn match_int32(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = IntTypeMatcher {
        size: IntTypeSize::Size32,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}

pub fn match_int64(_values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let int1_matcher = IntTypeMatcher {
        size: IntTypeSize::Size64,
    };

    Box::new(TypeMatcherValue {
        matcher: Box::new(int1_matcher),
    })
}
