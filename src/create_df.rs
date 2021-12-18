use polars::prelude::*;
use polars_core::prelude::*;
use polars::frame::DataFrame;


pub fn create_df(pathss:Vec<String>) -> Result<DataFrame> {
    let s1 = Series::new("Paths", pathss);
    let df: Result<DataFrame> = DataFrame::new(vec![s1]);
    //df.unwrap().to_csv("C:\\Users\\jakub\\Documents\\report.csv");
    df
    
}