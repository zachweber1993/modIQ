use super::subject_error::AssessmentSubjectError;

/// Represents the artifact being evaluated by an Assessment.
///
/// The subject identifies *what* is being assessed.
/// It contains identifying information about the target,
/// but no assessment state or results.
///
/// `label` is submission-time-known content only — whatever the
/// caller already knows when constructing the Assessment (C15,
/// `ARCHITECTURAL_RESOLUTION_C15_ASSESSMENT_REPORT_IDENTITY.md` §5(E))
/// — never content derived from collected Evidence, which does not
/// yet exist at construction time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentSubject {
    label: String,
}

impl AssessmentSubject {
    /// Creates a new AssessmentSubject.
    ///
    /// `label` must not be empty; an empty label carries no
    /// identifying content for a Report to reflect.
    pub fn new(label: impl Into<String>) -> Result<Self, AssessmentSubjectError> {
        let label = label.into();
        if label.trim().is_empty() {
            return Err(AssessmentSubjectError::EmptyLabel);
        }

        Ok(Self { label })
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accepts_a_real_label() {
        let subject = AssessmentSubject::new("FS25_HH_HempProdSystem.zip").unwrap();

        assert_eq!(subject.label(), "FS25_HH_HempProdSystem.zip");
    }

    #[test]
    fn new_rejects_an_empty_label() {
        let result = AssessmentSubject::new("");

        assert_eq!(result, Err(AssessmentSubjectError::EmptyLabel));
    }

    #[test]
    fn new_rejects_a_whitespace_only_label() {
        let result = AssessmentSubject::new("   ");

        assert_eq!(result, Err(AssessmentSubjectError::EmptyLabel));
    }
}
