use crate::{properties_interface::Prop, properties::*};


impl Prop for PropTreatmentRecommendation {
    fn get_name() -> &'static str {
        "Treatment"
    }

    fn get_description(&self) -> &str {
        match self {
            PropTreatmentRecommendation::DoctorVisit => "Doctor Visit",
            PropTreatmentRecommendation::EmergencyCall => "Emergency Call",
            PropTreatmentRecommendation::HomeRest => "Home Rest",
            PropTreatmentRecommendation::OverTheCounterMedication => "Over the Counter Medication"
        }
    }
}


impl Prop for PropInfectionLikelihood {
    fn get_name() -> &'static str {
        "Infection Likelihood"
    }

    fn get_description(&self) -> &str {
        match self {
            PropInfectionLikelihood::Low => "Low",
            PropInfectionLikelihood::Moderate => "Moderate",
            PropInfectionLikelihood::High => "High",
        }
    }
}

impl Prop for PropPossibleCauses {
    fn get_name() -> &'static str {
        "Possible Causes"
    }

    fn get_description(&self) -> &str {
        match self {
            PropPossibleCauses::Indigestion => "Indigestion",
            PropPossibleCauses::Gastritis => "Gastritis",
            PropPossibleCauses::Appendicitis => "Appendicitis",
            PropPossibleCauses::None => "None",
        }
    }
}

impl Prop for PropBodyTemperature {
    fn get_name() -> &'static str {
        "Temperature"
    }

    fn get_description(&self) -> &str {
        match self {
            PropBodyTemperature::High => "High",
            PropBodyTemperature::Normal => "Normal",
            PropBodyTemperature::Low => "Low",
        }
    }
}

impl Prop for PropPainLocation {
    fn get_name() -> &'static str {
        "Pain Zone"
    }

    fn get_description(&self) -> &str {
        match self {
            PropPainLocation::Head => "Head",
            PropPainLocation::Chest => "Chest",
            PropPainLocation::Abdomen => "Abdomen",
            PropPainLocation::Limbs => "Limbs",
        }
    }
}

impl Prop for PropSymptomDuration {
    fn get_name() -> &'static str {
        "Symptom Duration"
    }

    fn get_description(&self) -> &str {
        match self {
            // Implement according to your logic
            PropSymptomDuration::ShortTerm => "Short-Term",
            PropSymptomDuration::LongTerm => "Long-Term",
        }
    }
}

impl Prop for PropAssociatedSymptopms {
    fn get_name() -> &'static str {
        "Associated Symptoms"
    }

    fn get_description(&self) -> &str {
        match self {
            PropAssociatedSymptopms::Fever => "Fever",
            PropAssociatedSymptopms::Nausea => "Nausea",
            PropAssociatedSymptopms::Fatigue => "Fatigue",
            PropAssociatedSymptopms::None => "None",
        }
    }
}

impl Prop for AllergiesType {
    fn get_name() -> &'static str {
        "Allergies"
    }

    fn get_description(&self) -> &str {
        match self {
            AllergiesType::Yes => "Yes",
            AllergiesType::No => "No",
        }
    }
}

impl Prop for PropMedicationHistory {
    fn get_name() -> &'static str {
        "Medication History"
    }

    fn get_description(&self) -> &str {
        match self {
            PropMedicationHistory::OnMedication => "On Medication",
            PropMedicationHistory::NotOnMedication => "Not on Medication",
        }
    }
}

impl Prop for PropMedicalHistory {
    fn get_name() -> &'static str {
        "Medical History"
    }

    fn get_description(&self) -> &str {
        match self {
            PropMedicalHistory::Diabetes => "Diabetes",
            PropMedicalHistory::Hypertension => "Hypertension",
            PropMedicalHistory::None => "None",
        }
    }
}

impl Prop for PropAgeGroup {
    fn get_name() -> &'static str {
        "Age Group"
    }

    fn get_description(&self) -> &str {
        match self {
            PropAgeGroup::Child => "Child",
            PropAgeGroup::Adult => "Adult",
            PropAgeGroup::Senior => "Senior",
        }
    }
}


impl Prop for PropAgeGroupDangerous {
    fn get_name() -> &'static str {
        "Risk Group"
    }

    fn get_description(&self) -> &str {
        match self {
            Self::Yes => "Dangerous",
            Self::No => "Safe"
        }
    }
}

