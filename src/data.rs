pub mod data {
    use serde_json;
    use std;
    use serde_json::Value;

    #[derive(Debug)]
pub(crate) struct DataRange {
        pub(crate) name: String,
        pub(crate) lower_bound: f64,
        pub(crate) upper_bound: f64
    }

    impl DataRange {
        pub(crate) fn new(range: Value) -> std::result::Result<DataRange, &'static str>{

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

    #[derive(Debug)]
pub struct SampleSpace {
        pub(crate) space: Vec<DataRange>,
        pub(crate) samples: i64,
    }

    impl SampleSpace {
        pub fn new(input: &str) -> std::result::Result<SampleSpace, &'static str> {
            let json_input: serde_json::Value = serde_json::from_str(&input).expect("unable to read file");

            let samples: i64;
            match json_input["samples"].as_i64() {
                Some(x) => (samples = x),
                None => return Err("json file must contain a \"samples\" field"),
            };
            let array = json_input["parameters"].as_array();
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

            Ok(SampleSpace{ space: results, samples: samples })
        }
    }
}