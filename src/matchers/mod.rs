use dyn_clone::DynClone;

pub mod binary;
pub mod call;
pub mod cast;
pub mod combine;
pub mod constants;
pub mod exception;
pub mod fcmp;
pub mod icmp;
pub mod other;
pub mod types;
pub mod usage;

dyn_clone::clone_trait_object!(<T> Matcher<T>);

pub trait Matcher<T: Clone>: DynClone {
    fn is_match(&self, node: &T) -> bool;
}
