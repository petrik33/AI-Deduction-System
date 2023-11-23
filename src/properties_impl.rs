use crate::{properties_interface::Property, properties::*};


impl Property for TreatmentRecommendationType {
    fn get_name(&self) -> &str {
        "Treatment"
    }

    fn get_description(&self) -> &str {
        match self {
            TreatmentRecommendationType::DoctorVisit => "Doctor Visit",
            TreatmentRecommendationType::EmergencyCall => "Emergency Call",
            TreatmentRecommendationType::HomeRest => "Home Rest",
            TreatmentRecommendationType::OverTheCounterMedication => "Over the Counter Medication"
        }
    }
}


impl Property for LikelihoodOfViralInfectionType {
    fn get_name(&self) -> &str {
        "Infection Likelihood"
    }

    fn get_description(&self) -> &str {
        match self {
            LikelihoodOfViralInfectionType::Low => "Low",
            LikelihoodOfViralInfectionType::Moderate => "Moderate",
            LikelihoodOfViralInfectionType::High => "High",
        }
    }
}

impl Property for PossibleCausesAbdominalPainType {
    fn get_name(&self) -> &str {
        "Possible Causes"
    }

    fn get_description(&self) -> &str {
        match self {
            PossibleCausesAbdominalPainType::Indigestion => "Indigestion",
            PossibleCausesAbdominalPainType::Gastritis => "Gastritis",
            PossibleCausesAbdominalPainType::Appendicitis => "Appendicitis",
            PossibleCausesAbdominalPainType::None => "None",
        }
    }
}

impl Property for BodyTemperatureType {
    fn get_name(&self) -> &str {
        "Temperature"
    }

    fn get_description(&self) -> &str {
        match self {
            BodyTemperatureType::High => "High",
            BodyTemperatureType::Normal => "Normal",
            BodyTemperatureType::Low => "Low",
        }
    }
}

impl Property for PainLocationType {
    fn get_name(&self) -> &str {
        "Pain Zone"
    }

    fn get_description(&self) -> &str {
        match self {
            PainLocationType::Head => "Head",
            PainLocationType::Chest => "Chest",
            PainLocationType::Abdomen => "Abdomen",
            PainLocationType::Limbs => "Limbs",
        }
    }
}

impl Property for SymptomDurationType {
    fn get_name(&self) -> &str {
        "Symptom Duration"
    }

    fn get_description(&self) -> &str {
        match self {
            // Implement according to your logic
            SymptomDurationType::ShortTerm => "Short-Term",
            SymptomDurationType::LongTerm => "Long-Term",
        }
    }
}

impl Property for AssociatedSymptomsType {
    fn get_name(&self) -> &str {
        "Associated Symptoms"
    }

    fn get_description(&self) -> &str {
        match self {
            AssociatedSymptomsType::Fever => "Fever",
            AssociatedSymptomsType::Nausea => "Nausea",
            AssociatedSymptomsType::Fatigue => "Fatigue",
            AssociatedSymptomsType::None => "None",
        }
    }
}

impl Property for AllergiesType {
    fn get_name(&self) -> &str {
        "Allergies"
    }

    fn get_description(&self) -> &str {
        match self {
            AllergiesType::Yes => "Yes",
            AllergiesType::No => "No",
        }
    }
}

impl Property for MedicationHistoryType {
    fn get_name(&self) -> &str {
        "Medication History"
    }

    fn get_description(&self) -> &str {
        match self {
            MedicationHistoryType::OnMedication => "On Medication",
            MedicationHistoryType::NotOnMedication => "Not on Medication",
        }
    }
}

impl Property for MedicalHistoryType {
    fn get_name(&self) -> &str {
        "Medical History"
    }

    fn get_description(&self) -> &str {
        match self {
            MedicalHistoryType::Diabetes => "Diabetes",
            MedicalHistoryType::Hypertension => "Hypertension",
            MedicalHistoryType::None => "None",
        }
    }
}

impl Property for AgeGroupType {
    fn get_name(&self) -> &str {
        "Age Group"
    }

    fn get_description(&self) -> &str {
        match self {
            AgeGroupType::Child => "Child",
            AgeGroupType::Adult => "Adult",
            AgeGroupType::Senior => "Senior",
        }
    }
}

