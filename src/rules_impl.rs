use crate::properties::*;
use crate::rules::{AndCondition, OrCondition, ProdRule, PropertyEqualsCondition};

pub fn create_rules() -> Vec<ProdRule> {
    let mut rules: Vec<ProdRule> = Vec::new();

    let age_rule1 = ProdRule {
        condition: Box::new(PropertyEqualsCondition {
            value: Property::AgeGroup(PropAgeGroup::Adult),
        }),
        outcome: Property::AgeGroupDangerous(PropAgeGroupDangerous::No),
    };

    rules.push(age_rule1);

    let age_rule2 = ProdRule {
        condition: Box::new(OrCondition {
            condition1: Box::new(PropertyEqualsCondition {
                value: Property::AgeGroup(PropAgeGroup::Child),
            }),
            condition2: Box::new(PropertyEqualsCondition {
                value: Property::AgeGroup(PropAgeGroup::Senior),
            }),
        }),
        outcome: Property::AgeGroupDangerous(PropAgeGroupDangerous::No),
    };

    rules.push(age_rule2);

    // Rule 1: If Body Temperature == High, Set Likelihood of Viral Infection to High
    let temp_rule = ProdRule {
        condition: Box::new(PropertyEqualsCondition {
            value: Property::BodyTemperature(PropBodyTemperature::High),
        }),
        outcome: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::High),
    };
    rules.push(temp_rule);

    // Rule 2: If Pain Location == Abdomen, Set Possible Causes for Abdominal Pain to Indigestion
    let pain_rule = ProdRule {
        condition: Box::new(PropertyEqualsCondition {
            value: Property::PainLocation(PropPainLocation::Abdomen),
        }),
        outcome: Property::PossibleCausesAbdominalPain(PropPossibleCauses::Indigestion),
    };
    rules.push(pain_rule);

    // Rule 3: If Body Temperature == High AND Pain Location == Abdomen, Set Likelihood of Viral Infection to High
    let temp_pain_rule = ProdRule {
        condition: Box::new(AndCondition {
            condition1: Box::new(PropertyEqualsCondition {
                value: Property::BodyTemperature(PropBodyTemperature::High),
            }),
            condition2: Box::new(PropertyEqualsCondition {
                value: Property::PainLocation(PropPainLocation::Abdomen),
            }),
        }),
        outcome: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::High),
    };
    rules.push(temp_pain_rule);

    let duration_symptoms_rule = ProdRule {
        condition: Box::new(OrCondition {
            condition1: Box::new(PropertyEqualsCondition {
                value: Property::SymptomDuration(PropSymptomDuration::LongTerm),
            }),
            condition2: Box::new(PropertyEqualsCondition {
                value: Property::AssociatedSymptoms(PropAssociatedSymptopms::Fever),
            }),
        }),
        outcome: Property::PossibleCausesAbdominalPain(PropPossibleCauses::Appendicitis),
    };
    rules.push(duration_symptoms_rule);
    
    // Rule 5: If Medication History == Not on Medication OR Allergies == Yes,
    // Set Likelihood of Allergic Reaction to High
    let med_allergies_rule = ProdRule {
        condition: Box::new(OrCondition {
            condition1: Box::new(PropertyEqualsCondition {
                value: Property::MedicationHistory(PropMedicationHistory::NotOnMedication),
            }),
            condition2: Box::new(PropertyEqualsCondition {
                value: Property::Allergies(AllergiesType::Yes),
            }),
        }),
        outcome: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::High),
    };
    rules.push(med_allergies_rule);
    
    // Rule 6: If Body Temperature == High AND Pain Location == Abdomen AND Associated Symptoms include Nausea,
    // Set Likelihood of Viral Infection to High (Consolidated from complex_rule_1)
    // This rule overwrites complex_rule_1 and avoids conflict with Rule 3
    let temp_pain_nausea_rule = ProdRule {
        condition: Box::new(AndCondition {
            condition1: Box::new(PropertyEqualsCondition {
                value: Property::BodyTemperature(PropBodyTemperature::High),
            }),
            condition2: Box::new(AndCondition {
                condition1: Box::new(PropertyEqualsCondition {
                    value: Property::PainLocation(PropPainLocation::Abdomen),
                }),
                condition2: Box::new(PropertyEqualsCondition {
                    value: Property::AssociatedSymptoms(PropAssociatedSymptopms::Nausea),
                }),
            }),
        }),
        outcome: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::High),
    };
    rules.push(temp_pain_nausea_rule);
    
    // Rule 7: If Age Group == Adult AND (Medical History == Diabetes OR Medical History == Hypertension),
    // Set Likelihood of Viral Infection to Moderate (Consolidated from complex_rule_2)
    // This rule overwrites complex_rule_2 and avoids conflict with Rule 1
    let age_medical_history_rule = ProdRule {
        condition: Box::new(AndCondition {
            condition1: Box::new(PropertyEqualsCondition {
                value: Property::AgeGroup(PropAgeGroup::Adult),
            }),
            condition2: Box::new(OrCondition {
                condition1: Box::new(PropertyEqualsCondition {
                    value: Property::MedicalHistory(PropMedicalHistory::Diabetes),
                }),
                condition2: Box::new(PropertyEqualsCondition {
                    value: Property::MedicalHistory(PropMedicalHistory::Hypertension),
                }),
            }),
        }),
        outcome: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::Moderate),
    };
    rules.push(age_medical_history_rule);
    
    // Rule 8: If Likelihood of Viral Infection is High OR Likelihood of Allergic Reaction is High,
    // Set Treatment Recommendation to Doctor Visit
    let infection_allergic_treatment_condition = OrCondition {
        condition1: Box::new(PropertyEqualsCondition {
            value: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::High),
        }),
        condition2: Box::new(PropertyEqualsCondition {
            value: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::Moderate),
        }),
    };
    let treatment_rule = ProdRule {
        condition: Box::new(infection_allergic_treatment_condition),
        outcome: Property::TreatmentRecommendation(PropTreatmentRecommendation::DoctorVisit),
    };
    rules.push(treatment_rule);
    
    // Rule 9: If Likelihood of Viral Infection is Moderate AND Age Group is Adult,
    // Set Treatment Recommendation to Home Rest
    let moderate_infection_home_rest_condition = AndCondition {
        condition1: Box::new(PropertyEqualsCondition {
            value: Property::LikelihoodOfViralInfection(PropInfectionLikelihood::Moderate),
        }),
        condition2: Box::new(PropertyEqualsCondition {
            value: Property::AgeGroup(PropAgeGroup::Adult),
        }),
    };
    let treatment_rule_2 = ProdRule {
        condition: Box::new(moderate_infection_home_rest_condition),
        outcome: Property::TreatmentRecommendation(PropTreatmentRecommendation::HomeRest),
    };
    rules.push(treatment_rule_2);

    rules
}
