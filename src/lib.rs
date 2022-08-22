use data::Data::DataRange;
use itertools::{Itertools, zip};
use rand::seq::SliceRandom;
use std::{error::Error, fs};
mod data;
use crate::data::Data::SampleSpace;

/// Main logic function. main handles arguments and calls run
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

    let lhs = generate_lhs(space)?;
    println!("{:?}", lhs);

    Ok(())
}


/// Takes in the desired number of samples, and returns a vector with the all the sample levels
/// # Example:
/// * generate_levels(11) -> [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]
/// 
/// # Arguments
/// * accepts a single i64 with the number of samples
/// 
/// # Returns
/// * vector of i64
fn generate_levels(samples: i64) -> Vec<i64> {
    let mut strata = Vec::new();

    (-samples / 2..(samples / 2) + 1).for_each(|i: i64| {
        strata.push(i);
    });

    strata
}


/// Returns random permutaions of a vector with an associated data range
/// The DataRange object is there to fit the hypercube samples to the desired range for parameter samples
/// 
/// # Arguments
/// * mutable vector of i64, a vector of DataRange objects
/// 
/// # Returns
/// * Result type containing error or a vector of tuples. The tuples each contain a vector of i64 and a DataRange
/// * the length of the output vector should be equal to the number of factors input
/// * the vector of i64 will be a permutation of the levels input vector
/// * the DataRange object will be one of the same objects in the in the ranges input
fn generate_level_perms(
    mut levels: Vec<i64>,
    ranges: Vec<DataRange>,
) -> Result<Vec<(Vec<i64>, DataRange)>, &'static str> {
    let mut rand = rand::thread_rng();

    let factors = ranges.len();
    Ok(zip(0..factors, ranges)
        .into_iter()
        .map(|x| -> (Vec<i64>, DataRange) {
            levels.shuffle(&mut rand);
            (levels.iter().map(|x| -> i64 { *x }).collect_vec(), x.1)
        })
        .collect_vec())
}

/// Generates the latin hypercube(oid) sample of the input space
/// # Arguments
/// * A sampleSpace object to sample over
/// 
/// # Returns
/// * a 2D vector of floating point numbers corresponding to the coordinates for each sample point
/// 
/// # TODO
/// * this function should return a vector of tuples, which makes more sense.
/// * each tuple would be the point being sampled
fn generate_lhs(space: SampleSpace) -> Result<Vec<Vec<f64>>, &'static str> {
    let samples = space.samples;

    let levels = generate_levels(samples);
    let level_matrix = generate_level_perms(levels, space.space)?;

    Ok(generate_sample_matrix(level_matrix, samples))


}

fn generate_sample_matrix(level_matrix: Vec<(Vec<i64>, DataRange)>, samples: i64) -> Vec<Vec<f64>> {
    level_matrix
        .into_iter()
        .map(|column: (Vec<i64>, DataRange)| -> Vec<f64> {
            column.0
                .into_iter()
                .map(|entry| -> f64 {
                    let random = rand::random::<f64>();
                    let entry = (entry + (samples - 1) / 2) as f64;
                    let entry = (entry + random) / (samples as f64);
                    let dilation_factor = column.1.upper_bound - column.1.lower_bound;
                    entry * dilation_factor + column.1.lower_bound
                })
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet, hash::Hash};

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
