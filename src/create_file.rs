use xlsxwriter::*;
use std::io::{Error};

pub fn create_xlsx() -> Result<(), Error> {
    let workbook = Workbook::new("report.xlsx");
    workbook.close().unwrap();
    Ok(())
}


