use debt_calculator::{extract_report, report_debt};


fn main() {
    let path = "assets/sample_code.c";

    match extract_report(path) {
        Ok(xml_report) => {
            report_debt(&xml_report);
        },
        Err(e) => eprintln!("Cppcheck Execution Error: {}",e),
    }
    
    

}
