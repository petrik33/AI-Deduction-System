use crate::properties::{Property, PropMap};

pub trait Condition {
    fn evaluate(&self, props: &PropMap) -> bool;
}


pub trait Rule {
    fn evaluate(&self, props: &mut PropMap);
}

pub struct ProdRule {
    pub outcome: Property,
    pub condition: Box<dyn Condition>
}

impl Rule for ProdRule {
    fn evaluate(&self, props: &mut PropMap) {
        if self.condition.evaluate(props) {
            props.replace(self.outcome.clone());
        }
    }
}


pub struct PropertyEqualsCondition {
    pub value: Property,
}

impl Condition for PropertyEqualsCondition {
    fn evaluate(&self, map: &PropMap) -> bool {
        map.get(&self.value).map_or(false, |prop| prop.eq(&self.value))
    }
}


pub struct AndCondition {
    pub condition1: Box<dyn Condition>,
    pub condition2: Box<dyn Condition>,
}

impl Condition for AndCondition {
    fn evaluate(&self, map: &PropMap) -> bool {
        self.condition1.evaluate(map) && self.condition2.evaluate(map)
    }
}


pub struct OrCondition {
    pub condition1: Box<dyn Condition>,
    pub condition2: Box<dyn Condition>,
}

impl Condition for OrCondition {
    fn evaluate(&self, map: &PropMap) -> bool {
        self.condition1.evaluate(map) || self.condition2.evaluate(map)
    }
}
