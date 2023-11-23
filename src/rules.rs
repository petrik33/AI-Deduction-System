use std::collections::HashSet;
use crate::properties::HealthProperty;
use crate::properties_interface::Property;


type PropMap = HashSet<HealthProperty>;

pub trait Condition {
    fn evaluate(&self, props: &PropMap) -> bool;
}


pub struct Rule {
    condition: ConditionType,
    outcome: dyn Property,
}


pub enum ConditionType {
    Equals(Box<PropertyEqualsCondition>),
    And(AndCondition),
    Or(OrCondition)
}


pub struct PropertyEqualsCondition {
    value: HealthProperty
}

impl Condition for PropertyEqualsCondition {
    fn evaluate(&self, map: &PropMap) -> bool {
        map.contains(&self.value)
    }
}


pub struct AndCondition {
    condition1: Box<dyn Condition>,
    condition2: Box<dyn Condition>,
}

impl Condition for AndCondition {
    fn evaluate(&self, map: &PropMap) -> bool {
        self.condition1.evaluate(map) && self.condition2.evaluate(map)
    }
}


pub struct OrCondition {
    condition1: Box<dyn Condition>,
    condition2: Box<dyn Condition>,
}

impl Condition for OrCondition {
    fn evaluate(&self, map: &PropMap) -> bool {
        self.condition1.evaluate(map) || self.condition2.evaluate(map)
    }
}
