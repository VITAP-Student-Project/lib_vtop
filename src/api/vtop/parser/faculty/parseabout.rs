use crate::api::vtop::types::*;
use scraper::{Html, Selector};

pub fn parse_faculty_data(html: String) -> FacultyDetails {
    let document = Html::parse_document(&html);
    let table_selector = Selector::parse("table.table.table-bordered").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();
    
    let mut faculty_details = FacultyDetails {
        name: String::new(),
        designation: String::new(),
        department: String::new(),
        school_centre: String::new(),
        email: String::new(),
        cabin_number: String::new(),
    };
    
    if let Some(table) = document.select(&table_selector).next() {
        for row in table.select(&row_selector) {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            if cells.len() >= 2 {
                let label = cells[0]
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .replace("\t", "")
                    .replace("\n", "")
                    .to_lowercase();
                
                let value = cells[1]
                    .text()
                    .collect::<Vec<_>>()
                    .join("")
                    .trim()
                    .replace("\t", "")
                    .replace("\n", "");
                
                match label.as_str() {
                    l if l.contains("name of the faculty") => {
                        faculty_details.name = value;
                    }
                    l if l.contains("designation") => {
                        faculty_details.designation = value;
                    }
                    l if l.contains("name of department") => {
                        faculty_details.department = value;
                    }
                    l if l.contains("school") || l.contains("centre") => {
                        faculty_details.school_centre = value;
                    }
                    l if l.contains("e-mail") => {
                        faculty_details.email = value;
                    }
                    l if l.contains("cabin number") => {
                        faculty_details.cabin_number = value;
                    }
                    _ => {}
                }
            }
        }
    }
    
    faculty_details
}
                   