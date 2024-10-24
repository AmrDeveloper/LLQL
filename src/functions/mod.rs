use std::{collections::HashMap, sync::OnceLock};

use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_std::function::standard_function_signatures;
use gitql_std::function::standard_functions;
use inst_matcher::register_inst_matchers_function_signatures;
use inst_matcher::register_inst_matchers_functions;
use type_matcher::register_type_matchers_function_signatures;
use type_matcher::register_type_matchers_functions;

pub(crate) mod arithmetic_matchers;
pub(crate) mod icmp_matchers;
pub(crate) mod inst_matcher;
pub(crate) mod type_matcher;

pub fn llvm_ir_functions() -> &'static HashMap<&'static str, Function> {
    static HASHMAP: OnceLock<HashMap<&'static str, Function>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = standard_functions().to_owned();
        register_inst_matchers_functions(&mut map);
        register_type_matchers_functions(&mut map);
        map
    })
}

pub fn llvm_ir_function_signatures() -> HashMap<&'static str, Signature> {
    let mut map = standard_function_signatures().to_owned();
    register_inst_matchers_function_signatures(&mut map);
    register_type_matchers_function_signatures(&mut map);
    map
}
