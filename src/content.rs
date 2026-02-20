use serde::Serialize;

/// A portfolio project with a name, link, and preview image.
#[derive(Serialize)]
pub struct Project {
    pub name: &'static str,
    pub link: &'static str,
    pub image: &'static str,
}

/// An entry in the education timeline.
#[derive(Serialize)]
pub struct Education {
    pub r#type: &'static str,
    pub year: &'static str,
    pub name: &'static str,
    pub place: &'static str,
}

/// An entry in the work experience timeline.
#[derive(Serialize)]
pub struct Experience {
    pub r#type: &'static str,
    pub year: &'static str,
    pub name: &'static str,
    pub place: &'static str,
}

/// A social or contact link.
#[derive(Serialize)]
pub struct Link {
    pub name: &'static str,
    pub link: &'static str,
    pub linkname: &'static str,
}

/// Return all portfolio projects.
pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            name: "HomepageRs",
            link: "https://github.com/itacentury/HomepageRs",
            image: "/images/homepagers-preview.jpg",
        },
        Project {
            name: "CenturyPackageBO1",
            link: "https://github.com/itacentury/CenturyPackageBO1",
            image: "/images/centurypackagebo1-preview.jpg",
        },
        Project {
            name: "T5GSCLoader",
            link: "https://github.com/itacentury/T5GSCLoader",
            image: "/images/t5gscloader-preview.jpg",
        },
        Project {
            name: "SudokuPy",
            link: "https://github.com/itacentury/SudokuPy",
            image: "/images/sudokupy-preview.jpg",
        },
    ]
}

/// Return all education entries.
pub fn get_education() -> Vec<Education> {
    vec![
        Education {
            r#type: "Bachelor of Science",
            year: "2021-2025",
            name: "Computer Science",
            place: "University of Augsburg",
        },
        Education {
            r#type: "Vocational Training",
            year: "2018-2021",
            name: "IT Specialist",
            place: "ESG GmbH",
        },
    ]
}

/// Return all work experience entries.
pub fn get_experience() -> Vec<Experience> {
    vec![
        Experience {
            r#type: "Fulltime Employment",
            year: "2025-Now",
            name: "Embedded Software Developer",
            place: "WashTec AG",
        },
        Experience {
            r#type: "Working Student",
            year: "2025",
            name: "Software Developer",
            place: "Hensoldt AG",
        },
        Experience {
            r#type: "Working Student",
            year: "2021-2024",
            name: "Software Tester and Developer",
            place: "ESG GmbH",
        },
        Experience {
            r#type: "Fulltime Employment",
            year: "2021",
            name: "Software Tester",
            place: "ESG GmbH",
        },
    ]
}

/// Return all social/contact links.
pub fn get_links() -> Vec<Link> {
    vec![
        Link {
            name: "GitHub",
            link: "https://github.com/itacentury/",
            linkname: "github.com/itacentury",
        },
        Link {
            name: "Linkedin",
            link: "https://www.linkedin.com/in/hofl/",
            linkname: "linkedin.com/in/hofl",
        },
        Link {
            name: "YouTube",
            link: "https://www.youtube.com/@Zuckerschlecken",
            linkname: "youtube.com/@Zuckerschlecken",
        },
    ]
}
