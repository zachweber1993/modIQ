use super::{
    context::AssessmentContext, evidence::Evidence, finding::Finding,
    recommendation::Recommendation, report::AssessmentReport, subject::AssessmentSubject,
};

pub struct Assessment {
    subject: AssessmentSubject,
    context: AssessmentContext,
    evidence: Vec<Evidence>,
    findings: Vec<Finding>,
    recommendations: Vec<Recommendation>,
    report: AssessmentReport,
}
