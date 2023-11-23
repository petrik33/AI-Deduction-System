// // context.rs

// use crate::rules::{Rule, Condition};
// use crate::properties_interface::PropertyType;
// use crate::outcomes::Outcome;

// #[derive(Debug)]
// pub struct Context {
//     goals_stack: Vec<Goal>,
//     context_stack: Vec<(PropertyType, PropertyValue, usize)>,
//     reset_deck: Vec<Rule>,
//     logical_flag: bool,
//     properties: Vec<PropertyType>,
//     answers: Vec<(PropertyType, PropertyValue)>,
// }

// #[derive(Debug)]
// pub struct Goal(PropertyType, PropertyValue);
