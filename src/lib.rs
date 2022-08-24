use data::data::DataRange;
use itertools::{Itertools, zip};
use rand::{seq::SliceRandom, distributions::{Distribution, Uniform}};
use std::{error::Error, fs::{self, File}, io::Write};
mod data;
use crate::data::data::SampleSpace;

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

    let lhs = generate_lhs(&space)?;

    write_to_csv(&lhs, &space);

    // println!("{:?}", lhs);

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

    (-(samples - 1) / 2..(samples / 2) + 1).for_each(|i: i64| {
        strata.push(i);
    });

    strata
}


/// Returns random permutaions of a vector with an associated data range.
/// The DataRange object is there to fit the hypercube samples to the desired range for parameter samples.
/// 
/// # Arguments
/// * Mutable vector of i64, a vector of DataRange objects.
/// 
/// # Returns
/// * Result type containing error or a vector of tuples. The tuples each contain a vector of i64 and a DataRange.
/// * The length of the output vector should be equal to the number of factors input.
/// * The vector of i64 will be a permutation of the levels input vector.
/// * The DataRange object will be one of the same objects in the in the ranges input.
fn generate_level_perms(
    mut levels: Vec<i64>,
    ranges: &Vec<DataRange>,
) -> Result<Vec<(Vec<i64>, &DataRange)>, &'static str> {
    let mut rand = rand::thread_rng();

    let factors = ranges.len();
    Ok(zip(0..factors, ranges)
        .into_iter()
        .map(|x| -> (Vec<i64>, &DataRange) {
            levels.shuffle(&mut rand);
            (levels.iter().map(|x| -> i64 { *x }).collect_vec(), &x.1)
        })
        .collect_vec())
}

/// Generates the latin hypercube(oid) sample of the input space
/// # Arguments
/// * A [`SampleSpace`] object to sample over.
/// 
/// # Returns
/// * A 2D vector of floating point numbers corresponding to the coordinates for each sample point.
/// 
fn generate_lhs(space: &SampleSpace) -> Result<Vec<Vec<f64>>, &'static str> {
    let samples = space.samples;

    let levels = generate_levels(samples);
    let level_matrix = generate_level_perms(levels, &space.space)?;
    // println!("{:?}", &level_matrix);

    Ok(transpose(generate_sample_matrix(level_matrix, samples)))


}


/// Generates the full LHS and scales it to size of the input data ranges
/// # Arguments
/// * Takes in the level permutations matrix generated by [`generate_level_perms`].
/// 
/// # Returns
/// * The semi-final sample matrix containing all the points to be sampled.
/// * Final step is to be transposed in [`transpose`].
fn generate_sample_matrix(level_matrix: Vec<(Vec<i64>, &DataRange)>, samples: i64) -> Vec<Vec<f64>> {
    let mut rand = rand::thread_rng();
    level_matrix
        .into_iter()
        .map(|column: (Vec<i64>, &DataRange)| -> Vec<f64> {
            column.0
                .into_iter()
                .map(|entry| -> f64 {
                    let random = Uniform::from(0.0..1.0).sample(&mut rand);
                    // println!("{:?}", &random);
                    let entry = (entry + ((samples - 1) / 2)) as f64;
                    // println!("{:?}", &entry);
                    let entry = (entry + random) / (samples as f64);
                    // println!("{:?}", &entry);
                    let dilation_factor = column.1.upper_bound - column.1.lower_bound;
                    entry * dilation_factor + column.1.lower_bound
                })
                .collect_vec()
        })
        .collect_vec()
}


/// Utility function found on Stack Overflow
/// transposes a vector of vectors
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}


/// Writes the sample matrix out to a csv file
/// 
/// # Arguments
/// * Takes in a sample matrix &[`Vec<Vec<f64>>`] like from [`generate_lhs`] and &[`SampleSpace`] object
/// 
/// # Returns
/// Nothing
fn write_to_csv(sample_matrix: &Vec<Vec<f64>>, space: &SampleSpace) -> std::io::Result<()> {
    let filename = "output.csv";
    let mut contents: String = String::new();
    let mut header = String::from("CaseName");

    space.space.iter().for_each(|x| -> () { header.push_str(&format!("{}{}{}{}{}", String::from(","), x.name, String::from("_"), String::from("Template,"), x.name)); });
    header.push_str("\n");

    sample_matrix.iter().for_each(|point| -> () {
        let mut line = String::new();
        zip(point, &space.space).for_each(|x| -> () {
            line.push_str(",{{{");
            line.push_str(&x.1.name);
            line.push_str("}}},");
            line.push_str(&x.0.to_string());
        });
        line.push_str(",\n");
        contents.push_str(&line);
    });

    header.push_str(&contents);
    contents = header;

    let mut file = File::create(filename)?;
    file.write_all(contents.as_bytes())?;
    // println!("{}", contents);

    Ok(())
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
