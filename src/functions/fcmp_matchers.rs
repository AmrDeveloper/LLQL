use std::collections::HashMap;

use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::values::base::Value;

use crate::ir::types::InstMatcherType;
use crate::ir::values::InstMatcherValue;
use crate::matchers::instruction_matcher::FloatComparisonInstMatcher;

#[inline(always)]
pub fn register_float_comparisons_matchers_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("m_fcmp_eq", match_fcmp_eq_inst);
    map.insert("m_fcmp_ne", match_fcmp_ne_inst);
    map.insert("m_fcmp_gt", match_fcmp_gt_inst);
    map.insert("m_fcmp_ge", match_fcmp_ge_inst);
    map.insert("m_fcmp_lt", match_fcmp_lt_inst);
    map.insert("m_fcmp_le", match_fcmp_le_inst);
    map.insert("m_fcmp_ord", match_fcmp_ord_inst);
    map.insert("m_fcmp_uno", match_fcmp_uno_inst);
    map.insert("m_fcmp_ueq", match_fcmp_ueg_inst);
    map.insert("m_fcmp_une", match_fcmp_une_inst);
    map.insert("m_fcmp_ugt", match_fcmp_ugt_inst);
    map.insert("m_fcmp_uge", match_fcmp_uge_inst);
    map.insert("m_fcmp_ult", match_fcmp_ult_inst);
    map.insert("m_fcmp_ule", match_fcmp_ule_inst);

    map.insert("m_c_fcmp_eq", match_commutatively_fcmp_eq_inst);
    map.insert("m_c_fcmp_ne", match_commutatively_fcmp_ne_inst);
    map.insert("m_c_fcmp_gt", match_commutatively_fcmp_gt_inst);
    map.insert("m_c_fcmp_ge", match_commutatively_fcmp_ge_inst);
    map.insert("m_c_fcmp_lt", match_commutatively_fcmp_lt_inst);
    map.insert("m_c_fcmp_le", match_commutatively_fcmp_le_inst);
    map.insert("m_c_fcmp_ord", match_commutatively_fcmp_ord_inst);
    map.insert("m_c_fcmp_uno", match_commutatively_fcmp_uno_inst);
    map.insert("m_c_fcmp_ueq", match_commutatively_fcmp_ueg_inst);
    map.insert("m_c_fcmp_une", match_commutatively_fcmp_une_inst);
    map.insert("m_c_fcmp_ugt", match_commutatively_fcmp_ugt_inst);
    map.insert("m_c_fcmp_uge", match_commutatively_fcmp_uge_inst);
    map.insert("m_c_fcmp_ult", match_commutatively_fcmp_ult_inst);
    map.insert("m_c_fcmp_ule", match_commutatively_fcmp_ule_inst);
}

#[inline(always)]
pub fn register_float_comparisons_matchers_function_signatures(
    map: &mut HashMap<&'static str, Signature>,
) {
    map.insert(
        "m_fcmp_eq",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_ne",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_gt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_ge",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_lt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_le",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_ord",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_uno",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_ueq",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_une",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_ugt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_uge",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_ult",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_fcmp_ule",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    //
    map.insert(
        "m_c_fcmp_eq",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_ne",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_gt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_ge",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_lt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_le",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_ord",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_uno",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_ueq",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_une",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_ugt",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_uge",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_ult",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );

    map.insert(
        "m_c_fcmp_ule",
        Signature {
            parameters: vec![Box::new(InstMatcherType), Box::new(InstMatcherType)],
            return_type: Box::new(InstMatcherType),
        },
    );
}

fn match_fcmp_eq_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_eq(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_ne_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_ne(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_gt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_gt(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_ge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_ge(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_lt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_lt(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_le_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_le(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_ord_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_ord(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_uno_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_uno(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_ueg_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_ueq(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_une_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_une(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_ugt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_ugt(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_uge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_uge(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_ult_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_ult(lhs_matcher, rhs_matcher),
    })
}

fn match_fcmp_ule_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_fcmp_ule(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_fcmp_eq_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_eq(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_fcmp_ne_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_ne(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_fcmp_gt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_gt(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_fcmp_ge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_ge(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_fcmp_lt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_lt(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_fcmp_le_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_le(lhs_matcher, rhs_matcher),
    })
}

fn match_commutatively_fcmp_ord_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_ord(
            lhs_matcher,
            rhs_matcher,
        ),
    })
}

fn match_commutatively_fcmp_uno_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_uno(
            lhs_matcher,
            rhs_matcher,
        ),
    })
}

fn match_commutatively_fcmp_ueg_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_ueq(
            lhs_matcher,
            rhs_matcher,
        ),
    })
}

fn match_commutatively_fcmp_une_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_une(
            lhs_matcher,
            rhs_matcher,
        ),
    })
}

fn match_commutatively_fcmp_ugt_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_ugt(
            lhs_matcher,
            rhs_matcher,
        ),
    })
}

fn match_commutatively_fcmp_uge_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_uge(
            lhs_matcher,
            rhs_matcher,
        ),
    })
}

fn match_commutatively_fcmp_ult_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_ult(
            lhs_matcher,
            rhs_matcher,
        ),
    })
}

fn match_commutatively_fcmp_ule_inst(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let lhs_matcher = values[0]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    let rhs_matcher = values[1]
        .as_any()
        .downcast_ref::<InstMatcherValue>()
        .unwrap()
        .matcher
        .to_owned();

    Box::new(InstMatcherValue {
        matcher: FloatComparisonInstMatcher::create_commutatively_fcmp_ule(
            lhs_matcher,
            rhs_matcher,
        ),
    })
}
