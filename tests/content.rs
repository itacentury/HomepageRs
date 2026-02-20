use site::content;

#[test]
fn test_projects() {
    let projects = content::get_projects();
    assert_eq!(projects.len(), 4);
    assert_eq!(projects[0].name, "HomepageRs");
}

#[test]
fn test_education() {
    let education = content::get_education();
    assert_eq!(education.len(), 2);
}

#[test]
fn test_experience() {
    let experience = content::get_experience();
    assert_eq!(experience.len(), 4);
}

#[test]
fn test_links() {
    let links = content::get_links();
    assert_eq!(links.len(), 3);
}
