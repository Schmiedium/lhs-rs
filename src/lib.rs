use itertools::Itertools;
use rand::seq::SliceRandom;
use std::{collections::HashSet, error::Error, fs, hash::Hash};
mod data;
use crate::data::Data::SampleSpace;

///Main logic function. main handles arguments and calls run
/// 
/// # Arguments
/// * Accepts a reference to a string slice containing the file path to the json input file
/// 
/// # Returns
/// * Result type containing either nothing or an errror with what went wrong
pub fn run(input: &str) -> std::result::Result<(), Box<dyn Error>> {
    // println!("{}", input);
    let contents: String = fs::read_to_string(input)?;

    let space = SampleSpace::new(&contents)?;

    let lhs = generate_sample_matrix(space)?;

    Ok(())
}



fn generate_levels(samples: i64) -> Vec<i64> {
    let mut strata = Vec::new();

    (-samples / 2..(samples / 2) + 1).for_each(|i: i64| {
        strata.push(i);
    });

    strata
}

fn generate_level_perms(
    mut levels: Vec<i64>,
    factors: usize,
) -> Result<Vec<Vec<i64>>, &'static str> {
    let mut range = rand::thread_rng();

    Ok((0..factors)
        .into_iter()
        .map(|_| -> Vec<i64> {
            levels.shuffle(&mut range);
            levels.iter().map(|x| -> i64 { *x }).collect_vec()
        })
        .collect_vec())
}

fn generate_sample_matrix(space: SampleSpace) -> Result<Vec<Vec<f64>>, &'static str> {
    let samples = space.samples;
    let factors = space.space.len();

    let levels = generate_levels(samples);
    let level_matrix = generate_level_perms(levels, factors)?;

    Ok(level_matrix
        .into_iter()
        .map(|column: Vec<i64>| -> Vec<f64> {
            column
                .into_iter()
                .map(|entry| -> f64 {
                    let random = rand::random::<f64>();
                    let entry = (entry + (samples - 1) / 2) as f64;
                    (entry + random) / (samples as f64)
                })
                .collect_vec()
        })
        .collect_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    #[test]
    fn test_levels_output() {
        assert_eq!(generate_levels(5), vec![-2, -1, 0, 1, 2]);
    }

    #[test]
    fn test_level_columns() {
        assert_eq!(generate_level_perms(vec![-1, 0, 1], 2).unwrap().len(), 2)
    }

    #[test]
    fn test_level_columns_unique() {
        assert!(has_unique_elements(generate_level_perms(
            generate_levels(100),
            4
        )))
    }
}
