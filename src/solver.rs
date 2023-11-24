use std::collections::{HashSet, VecDeque};

use crate::properties::Property;
use crate::rules::ProdRule;

pub struct Solver {
    target: Property,
    goal_stack: VecDeque<(Property, usize)>, // Stack of goals along with their value and rule number
    context_stack: HashSet<Property>, // HashSet to maintain context (properties)
    reset_deck: HashSet<usize>, // Set to hold rule numbers for reset
    rules: Vec<ProdRule>, // Vector to hold all rules
}

impl Solver {
    pub fn new(target: Property, rules: Vec<ProdRule>) -> Self {
        Solver {
            target,
            goal_stack: VecDeque::new(),
            context_stack: HashSet::new(),
            reset_deck: HashSet::new(),
            rules,
        }
    }

    pub fn solve(&mut self) -> Option<Property> {
        self.goal_stack.push_back((self.target.clone(), 0)); // Push initial goal into the stack

        let mut logical_flag = false;

        while !logical_flag {
            if let Some((current_goal, rule_number)) = self.goal_stack.pop_back() {
                let rule = self.rules.get(rule_number);

                match rule {
                    Some(rule) => {
                        if rule.condition.evaluate(&self.context_stack) {
                            self.context_stack.insert(current_goal.clone());
                            logical_flag = true;
                        } else {
                            self.reset_deck.insert(rule_number);
                            self.goal_stack.push_back((current_goal, rule_number + 1));
                        }
                    }
                    None => {
                        logical_flag = true;
                    }
                }
            } else {
                logical_flag = true;
            }
        }

        if self.context_stack.contains(&self.target) {
            return Some(self.target.clone());
        }

        None
    }

    // Other methods for handling questions and maintaining context can be implemented here
}
