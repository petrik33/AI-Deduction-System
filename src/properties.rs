#[derive(Debug, Eq, Hash, PartialEq)]
pub enum HealthProperty {
    TreatmentRecommendation(TreatmentRecommendationType),
    LikelihoodOfViralInfection(LikelihoodOfViralInfectionType),
    PossibleCausesAbdominalPain(PossibleCausesAbdominalPainType),
    BodyTemperature(BodyTemperatureType),
    PainLocation(PainLocationType),
    SymptomDuration(SymptomDurationType),
    AssociatedSymptoms(AssociatedSymptomsType),
    Allergies(AllergiesType),
    MedicationHistory(MedicationHistoryType),
    MedicalHistory(MedicalHistoryType),
    AgeGroup(AgeGroupType),
}


#[derive(Debug, Eq, Hash, PartialEq)]
pub enum TreatmentRecommendationType {
    HomeRest,
    OverTheCounterMedication,
    DoctorVisit,
    EmergencyCall,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum LikelihoodOfViralInfectionType {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum PossibleCausesAbdominalPainType {
    Indigestion,
    Gastritis,
    Appendicitis,
    None,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum BodyTemperatureType {
    High,
    Normal,
    Low,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum PainLocationType {
    Head,
    Chest,
    Abdomen,
    Limbs,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum SymptomDurationType {
    ShortTerm,
    LongTerm,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum AssociatedSymptomsType {
    Fever,
    Nausea,
    Fatigue,
    None,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum AllergiesType {
    Yes,
    No,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum MedicationHistoryType {
    OnMedication,
    NotOnMedication,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum MedicalHistoryType {
    Diabetes,
    Hypertension,
    None,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum AgeGroupType {
    Child,
    Adult,
    Senior,
}