extern crate simple_excel_writer as excel;

use excel::*;

pub fn create_excel() {
    let mut wb = Workbook::create("/report.xlsx");
    wb.close().expect("close excel error!");
}