use serde_json::json;

pub fn get_education_vec() -> Vec<serde_json::Value> {
    let education_vec = vec![
        json!({
            "type": "bachelor of science",
            "year": "2021-2025",
            "name": "computer science",
            "place": "university of augsburg",
        }),
        json!({
            "type": "vocational training",
            "year": "2018-2021",
            "name": "IT specialist",
            "place": "esg gmbh",
        }),
    ];

    education_vec
}

pub fn get_experience_vec() -> Vec<serde_json::Value> {
    let experience_vec = vec![
        json!({ 
            "type": "fulltime employment", 
            "year": "2025-current", 
            "name": "embedded software developer",
            "place": "washtec gmbh",
        }),
        json!({ 
            "type": "working student", 
            "year": "2025", 
            "name": "software developer",
            "place": "hensoldt ag",
        }),
        json!({ 
            "type": "working student", 
            "year": "2021–2024", 
            "name": "software developer",
            "place": "esg gmbh",
        }),
        json!({ 
            "type": "fulltime employment", 
            "year": "2021", 
            "name": "software tester",
            "place": "esg gmbh",
        }),
    ];

    experience_vec
}

pub fn get_links_vec() -> Vec<serde_json::Value> {
    let links_vec = vec![
        json!({
            "name": "github",
            "link": "https://github.com/itacentury/",
            "linkname": "github.com/itacentury",
        }),
        json!({
            "name": "linkedin",
            "link": "https://www.linkedin.com/in/hofl/",
            "linkname": "linkedin.com/in/hofl",
        }),
        json!({
            "name": "youtube",
            "link": "https://www.youtube.com/@Zuckerschlecken",
            "linkname": "youtube.com/@Zuckerschlecken",
        }),
    ];

    links_vec
}
