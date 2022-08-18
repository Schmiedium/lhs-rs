use core::num;
use std::{fs, error::Error, result, process::Output};
use serde_json::{Value};
use itertools::Itertools;
use rand::seq::IteratorRandom;

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

fn generate_levels(samples: i32) -> Vec<i32> {
        
    let mut strata = Vec::new();

    (-samples/2..(samples/2)+1).for_each(|i: i32| {
        strata.push(i);
    });

    strata
}

fn generate_level_perms(levels: Vec<i32>, factors: usize) -> Result< Vec<Vec<i32>>, &'static str> {
    let len = levels.len();
    if len < factors {
        return Err("invalid space: must have more levels than factors to test");
    }

    let mut range = rand::thread_rng();
    Ok(levels.into_iter().permutations(len).unique().choose_multiple(&mut range, factors))
}

fn generate_sample_matrix(samples: i32, level_matrix: Vec<Vec<i32>>) {

    level_matrix.into_iter()
    .map(|column: Vec<i32>| -> Vec<f64> {
        
        column.into_iter()
        .map(|entry| -> f64 {
            let random = rand::random::<f64>();
            let entry = (entry + (samples-1)/2) as f64;
            (entry + random)/(samples as f64)
        })
        .collect_vec()

    }).collect_vec();

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levels_output() {
        assert_eq!(generate_levels(5), vec![-2, -1, 0, 1, 2]);
    }
    
    
}