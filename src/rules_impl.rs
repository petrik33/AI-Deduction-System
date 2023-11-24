use crate::rules::{Rule, ProdRule, PropertyEqualsCondition, AndCondition, OrCondition};
use crate::properties::*;


pub fn create_rules() -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();

    let temp_rule: Box<dyn Rule> = Box::new(ProdRule {
        condition: Box::new(PropertyEqualsCondition {
            value: Property::BodyTemperature(PropBodyTemperature::High),
        }),
        outcome: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::High),
    });

    rules.push(temp_rule);

    // Rule 2: If Pain Location == Abdomen, Set Possible Causes for Abdominal Pain to Indigestion
    let pain_rule: Box<dyn Rule> = Box::new(ProdRule {
        condition: Box::new(PropertyEqualsCondition {
            value: Property::PainLocation(PropPainLocation::Abdomen),
        }),
        outcome: Property::PossibleCausesAbdominalPain(PropPossibleCauses::Indigestion),
    });

    rules.push(pain_rule);

    // Rule 3: If Body Temperature == High AND Pain Location == Abdomen, Set Likelihood of Viral Infection to High
    let temp_pain_rule: Box<dyn Rule> = Box::new(ProdRule {
        condition: Box::new(AndCondition {
            condition1: Box::new(PropertyEqualsCondition {
                value: Property::BodyTemperature(PropBodyTemperature::High),
            }),
            condition2: Box::new(PropertyEqualsCondition {
                value: Property::PainLocation(PropPainLocation::Abdomen),
            }),
        }),
        outcome: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::High),
    });

    rules.push(temp_pain_rule);

    // Rule 4: If Symptom Duration == Long-Term OR Associated Symptoms include Fever, Set Possible Causes for Abdominal Pain to Appendicitis
    let duration_symptoms_rule: Box<dyn Rule> = Box::new(ProdRule {
        condition: Box::new(OrCondition {
            condition1: Box::new(PropertyEqualsCondition {
                value: Property::SymptomDuration(PropSymptomDuration::LongTerm),
            }),
            condition2: Box::new(PropertyEqualsCondition {
                value: Property::AssociatedSymptoms(PropAssociatedSymptopms::Fever),
            }),
        }),
        outcome: Property::PossibleCausesAbdominalPain(PropPossibleCauses::Appendicitis),
    });

    rules.push(duration_symptoms_rule);

    // Rule 5: If Medication History == Not on Medication OR Allergies == Yes, Set Likelihood of Allergic Reaction to High
    let med_allergies_rule: Box<dyn Rule> = Box::new(ProdRule {
        condition: Box::new(OrCondition {
            condition1: Box::new(PropertyEqualsCondition {
                value: Property::MedicationHistory(PropMedicationHistory::NotOnMedication),
            }),
            condition2: Box::new(PropertyEqualsCondition {
                value: Property::Allergies(AllergiesType::Yes),
            }),
        }),
        outcome: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::High),
    });


    rules.push(med_allergies_rule);

    rules
}
