use std::{fs, error::Error, result, process::Output};
use serde_json::{Value, json};

pub fn run(input: &str) -> std::result::Result<(), &'static str> {
    let contents: String = fs::read_to_string(input).unwrap();



    Ok(())
}



struct DataRange {
    name: String,
    lower_bound: f64,
    upper_bound: f64,
    step_size: f64,
}

impl DataRange {
    fn new(range: Value) -> std::result::Result<DataRange, &'static str>{

        let mut name: String = String::new();
        let lower_bound: f64;
        let upper_bound: f64;
        let step_size: f64;

        match range["name"].as_str() {
            Some(x) => name = x.to_string(),
            None => return Err("all variables must specify a name"),
        };
        match range["lower_bound"].as_f64() {
            Some(x) => lower_bound = x,
            None => return Err("all variables must specify a lower bound"),
        };
        match range["upper_bound"].as_f64() {
            Some(x) => upper_bound = x,
            None => return Err("all variables must specify an upper bound"),
        };
        match range["step_size"].as_f64() {
            Some(x) => step_size = x,
            None => return Err("all variables must specify a step size")
        };

        Ok(DataRange{ name: name, lower_bound: lower_bound, upper_bound: upper_bound, step_size: step_size})
    }
}

struct SampleSpace {
    space: Vec<DataRange>,
}

impl SampleSpace {
    fn new(input: &str) -> std::result::Result<SampleSpace, &'static str> {
        let data = json!(input).as_array()?;
        let mut Output;

        for v in data {
            
        }
        


        Ok(SampleSpace{ space: Output })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    
    
}