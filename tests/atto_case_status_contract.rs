// Contract comparison checks source mappings and the published JSON Schema.
// This does NOT substitute for guest compiler execution.
use serde_json::Value;

#[test]
fn atto_case_status_matches_wire_schema() {
    let source = include_str!("../atto/case_status.atto");
    let schema: Value = serde_json::from_str(include_str!("../schemas/case.json")).unwrap();
    let allowed = schema["properties"]["status"]["enum"].as_array().unwrap();
    let expected = [
        ("Intake", "intake"),
        ("CollectingDocuments", "collecting_documents"),
        ("Review", "review"),
        ("Submitted", "submitted"),
        ("Completed", "completed"),
        ("Closed", "closed"),
    ];
    assert_eq!(allowed.len(), expected.len());
    for (variant, wire) in expected {
        assert!(allowed.iter().any(|value| value.as_str() == Some(wire)));
        assert!(source.contains(&format!("if status eq CaseStatus.{variant} then return \"{wire}\"; fi")) ||
                (variant == "Closed" && source.contains("return \"closed\";")),
                "Atto mapping for {variant} must match {wire}");
        assert!(source.contains(&format!("status_wire(CaseStatus.{variant}) eq \"{wire}\"")),
                "missing executable fixture for {variant}");
    }
    for (variant, wire) in [
        (apme_interfaces::CaseStatus::Intake, "intake"),
        (apme_interfaces::CaseStatus::CollectingDocuments, "collecting_documents"),
        (apme_interfaces::CaseStatus::Review, "review"),
        (apme_interfaces::CaseStatus::Submitted, "submitted"),
        (apme_interfaces::CaseStatus::Completed, "completed"),
        (apme_interfaces::CaseStatus::Closed, "closed"),
    ] {
        assert_eq!(serde_json::to_value(variant).unwrap(), wire);
    }
    assert!(source.contains("// expect-exit: 42"));
}
