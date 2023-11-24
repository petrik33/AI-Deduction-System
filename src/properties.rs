use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Property {
    TreatmentRecommendation(PropTreatmentRecommendation),
    LikelihoodOfViralInfection(PropInfectionLikelihood),
    PossibleCausesAbdominalPain(PropPossibleCauses),
    BodyTemperature(PropBodyTemperature),
    PainLocation(PropPainLocation),
    SymptomDuration(PropSymptomDuration),
    AssociatedSymptoms(PropAssociatedSymptopms),
    Allergies(AllergiesType),
    MedicationHistory(PropMedicationHistory),
    MedicalHistory(PropMedicalHistory),
    AgeGroupDangerous(PropAgeGroupDangerous),
    AgeGroup(PropAgeGroup),
}

pub type PropMap = HashSet<Property>;


#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropTreatmentRecommendation {
    HomeRest,
    OverTheCounterMedication,
    DoctorVisit,
    EmergencyCall,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropInfectionLikelihood {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropPossibleCauses {
    Indigestion,
    Gastritis,
    Appendicitis,
    None,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropBodyTemperature {
    High,
    Normal,
    Low,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropPainLocation {
    Head,
    Chest,
    Abdomen,
    Limbs,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropSymptomDuration {
    ShortTerm,
    LongTerm,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropAssociatedSymptopms {
    Fever,
    Nausea,
    Fatigue,
    None,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum AllergiesType {
    Yes,
    No,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropMedicationHistory {
    OnMedication,
    NotOnMedication,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropMedicalHistory {
    Diabetes,
    Hypertension,
    None,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropAgeGroupDangerous {
    Yes,
    No
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum PropAgeGroup {
    Child,
    Adult,
    Senior,
}