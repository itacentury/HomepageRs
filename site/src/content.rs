use serde_json::json;

pub fn get_projects_vec() -> Vec<serde_json::Value> {
    let projects_vec = vec![
        json!({
            "name": "HomepageRs",
            "link": "https://github.com/itacentury/HomepageRs",
            "image": "/images/homepagers-preview.jpg",
        }),
        json!({
            "name": "CenturyPackageBO1",
            "link": "https://github.com/itacentury/CenturyPackageBO1",
            "image": "/images/centurypackagebo1-preview.jpg",
        }),
        json!({
            "name": "T5GSCLoader",
            "link": "https://github.com/itacentury/T5GSCLoader",
            "image": "/images/t5gscloader-preview.jpg",
        }),
        json!({
            "name": "SudokuPy",
            "link": "https://github.com/itacentury/SudokuPy",
            "image": "/images/sudokupy-preview.jpg",
        }),
    ];

    projects_vec
}

pub fn get_education_vec() -> Vec<serde_json::Value> {
    let education_vec = vec![
        json!({
            "type": "Bachelor of Science",
            "year": "2021-2025",
            "name": "Computer Science",
            "place": "University of Augsburg",
        }),
        json!({
            "type": "Vocational Training",
            "year": "2018-2021",
            "name": "IT Specialist",
            "place": "ESG GmbH",
        }),
    ];

    education_vec
}

pub fn get_experience_vec() -> Vec<serde_json::Value> {
    let experience_vec = vec![
        json!({
            "type": "Fulltime Employment",
            "year": "2025-Now",
            "name": "Embedded Software Developer",
            "place": "WashTec AG",
        }),
        json!({
            "type": "Working Student",
            "year": "2025",
            "name": "Software Developer",
            "place": "Hensoldt AG",
        }),
        json!({
            "type": "Working Student",
            "year": "2021–2024",
            "name": "Software Tester and Developer",
            "place": "ESG GmbH",
        }),
        json!({
            "type": "Fulltime Employment",
            "year": "2021",
            "name": "Software Tester",
            "place": "ESG GmbH",
        }),
    ];

    experience_vec
}

pub fn get_links_vec() -> Vec<serde_json::Value> {
    let links_vec = vec![
        json!({
            "name": "GitHub",
            "link": "https://github.com/itacentury/",
            "linkname": "github.com/itacentury",
        }),
        json!({
            "name": "Linkedin",
            "link": "https://www.linkedin.com/in/hofl/",
            "linkname": "linkedin.com/in/hofl",
        }),
        json!({
            "name": "YouTube",
            "link": "https://www.youtube.com/@Zuckerschlecken",
            "linkname": "youtube.com/@Zuckerschlecken",
        }),
    ];

    links_vec
}
