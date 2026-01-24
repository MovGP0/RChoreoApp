use choreo_master_mobile_json::{export, import};

#[test]
fn import_sample_choreo() {
    let json = include_str!("data/Test.choreo");
    let choreography = import(json).expect("import should succeed");

    assert_eq!(choreography.name, "ChoreoName");
    assert!(!choreography.roles.is_empty());
    assert!(!choreography.dancers.is_empty());
    assert!(!choreography.scenes.is_empty());
}

#[test]
fn export_round_trip() {
    let json = include_str!("data/Test.choreo");
    let choreography = import(json).expect("import should succeed");

    let exported = export(&choreography).expect("export should succeed");
    let round_trip = import(&exported).expect("re-import should succeed");

    assert_eq!(round_trip.name, choreography.name);
    assert_eq!(round_trip.roles.len(), choreography.roles.len());
    assert_eq!(round_trip.dancers.len(), choreography.dancers.len());
    assert_eq!(round_trip.scenes.len(), choreography.scenes.len());
}
