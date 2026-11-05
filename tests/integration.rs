use sigmaos_archbbs::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_archbbs::Archbbs::new("sigmaos-archbbs").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-archbbs"));
}
