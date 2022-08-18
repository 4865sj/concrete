use polars::prelude::*;
use polars::prelude::{Result as PolarResult};
use polars::frame::DataFrame;
use std::fs::File;
use std::path::{Path};
use polars::prelude::SerReader;

pub fn read_csv<P: AsRef<Path>>(path: P) -> PolarResult<DataFrame> {
    /* Example function to create a dataframe from an input csv file*/
    let file = File::open(path).expect("Cannot open file.");

    CsvReader::new(file)
    .has_header(true)
    .finish()
}

pub fn column_to_vec(df: DataFrame, column: &str) -> Vec<f64> {
    let target = df.select(column);
    let target_array = target.unwrap().to_ndarray::<Float64Type>().unwrap();
    let mut target_vec: Vec<f64> = Vec::new();
    for val in target_array.iter() {
        target_vec.push(*val);
    }
    target_vec
}

pub fn reduce_vector_size(data: Vec<f64>, size: i64) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    if data.len() < size.try_into().unwrap() {
        println!("Enter integer less than vector's size");
    } else {
        for i in 0..size {
            let j = i as usize;
            let tempt = data[j].clone();
            result.push(tempt);
        }
    }
    result
}

pub fn normalize_vector(data: Vec<f64>) -> (Vec<f64>, f64) {
    let mut vec_nor: Vec<f64> = Vec::new();
    let max = data.iter().cloned().fold(0./0., f64::max);
    let min = data.iter().cloned().fold(0./0., f64::min);
    let mut memory: f64 = 0.0;
    let min_abs = min.clone().abs();
    if max >= min_abs {
        for val in data {
            let tempt = val/max;
            vec_nor.push(tempt);
            memory = max;
        }
      } else {
        for val in data {
            let tempt = val/min_abs;
            vec_nor.push(tempt);
            memory = min_abs;
        }
    }
    (vec_nor, memory)
}


