use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str};


#[derive(Debug, Serialize, Deserialize)]
struct CppCheckError{
    #[serde(rename = "@severity")]
    severity: String,
    
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "errors")]
struct CppErrors{
    #[serde(rename = "error", default)]
    errors: Vec<CppCheckError>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "results")]
struct CppResults {
    #[serde(rename = "errors")]
    pub errors_container: CppErrors,

}


pub fn extract_report(path: &str) -> Result<String, String>
{
    let output = Command::new("cppcheck").args(["--enable=all","--xml",path]).output().expect("failed to execute process");
    
    if output.status.success() 
    {
        Ok(String::from_utf8_lossy(&output.stderr).to_string())
    }
    else{
        Err(format!("Cppcheck failed with exit code: {:?}",output.status.code()))
    }
    
}

pub fn report_debt(xml_report: &str)
{
   
   let result: CppResults = from_str(xml_report).unwrap();
   
   let total_minutes = result.errors_container.errors.iter().map(|issue| {
    get_cost_in_minutes(&issue.severity)
   }).sum::<u32>();

   let total_hours = total_minutes as f64 / 60.0;

   println!("Total Estimated Man-Hours: {:.2}",total_hours);


}

// In minutes per occurrence
fn get_cost_in_minutes(severity: &str) -> u32 {
    match severity {
        "error" => 45,      // Average of 30-60 min
        "warning" => 20,    // Average of 10-30 min
        "style" | "portability" => 10, // Average of 5-15 min
        _ => 5,             // Default for unknown/info
    }
}