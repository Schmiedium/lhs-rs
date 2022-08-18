use core::num;
use std::{fs, error::Error, result, process::Output};
use serde_json::{Value};
use itertools::Itertools;

pub fn run(input: &str) -> std::result::Result<(), Box<dyn Error>> {
    // println!("{}", input);
    let contents: String = fs::read_to_string(input)?;

    let space = SampleSpace::new(&contents)?;



    Ok(())
}



struct DataRange {
    name: String,
    lower_bound: f64,
    upper_bound: f64
}

impl DataRange {
    fn new(range: Value) -> std::result::Result<DataRange, &'static str>{

        let mut name: String = String::new();
        let lower_bound: f64;
        let upper_bound: f64;

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

        Ok(DataRange{ name: name, lower_bound: lower_bound, upper_bound: upper_bound})
    }
}

pub struct SampleSpace {
    space: Vec<DataRange>,
}

impl SampleSpace {
    pub fn new(input: &str) -> std::result::Result<SampleSpace, &'static str> {
        let json_input: serde_json::Value = serde_json::from_str(&input).expect("unable to read file");
        // println!("{}", json_input);
        let array = json_input.as_array();
        let mut data: Vec<Value> = Vec::new();
        let mut results: Vec<DataRange> = Vec::new();

        

        match array {
            Some(x) => data = x.to_vec(),
            None => return Err("input file not structured properly"),
        }

        for v in data {
            let range = DataRange::new(v)?;
            results.push(range);
        }

        Ok(SampleSpace{ space: results })
    }
}

fn generate_levels(num_runs: i32) -> Vec<i32> {
    let mut strata = Vec::new();

    (-num_runs/2..num_runs/2 + 1).for_each(|i: i32| {
        strata.push(i);
    });

    strata
}

fn generate_level_perms(levels: Vec<i32>) -> Vec<Vec<i32>> {
    let len = levels.len();
    levels.into_iter().permutations(len).unique().collect_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levels_output() {
        assert_eq!(generate_levels(5), vec![-2, -1, 0, 1, 2]);
    }

    #[test]
    fn test_level_columns() {
        assert_eq!(generate_level_perms(vec![-1, 0, 1]), vec![vec![-1, 0, 1], vec![-1, 1, 0], vec![0, -1, 1], vec![0, 1, -1], vec![1, -1, 0], vec![1, 0, -1],])
    }
    
    
}