use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

struct Header {
    header_dict: HashMap<String, String>,
    Samples: u32,
}


fn parse_vcf(file_name: String) -> HashMap<String, String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    let reader = BufReader::new(file);
    // file.read_to_string(&mut contents).unwrap();
    let mut x = 0;
    let mut vcf_header: Header = Header { header_dict: HashMap::new(), Samples: 0 };
    for line in reader.lines() {
        x += 1;
        let line = line.unwrap();
        println!("{}: {}", x, line);
        let parts: Vec<&str> = line.split("=").collect();
        if (parts.len() > 1) {
            vcf_header.header_dict.insert(
                parts[0].to_string(),
                parts[1].to_string(),
            );
        }

        if parts[0] == "##fileformat" {
            println!("File format is: {}", parts[1]);
        }
    }
    println!("{}", contents);
    for key in vcf_header.header_dict.keys() {
        println!("{}: {}", key, vcf_header.header_dict[key]);
    }
    vcf_header.header_dict
}

fn parse_vcf_to_header(file_name: String) -> Header {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    let reader = BufReader::new(file);
    // file.read_to_string(&mut contents).unwrap();
    let mut x = 0;
    let mut vcf_header: Header = Header { header_dict: HashMap::new(), Samples: 0 };
    for line in reader.lines() {
        x += 1;
        let line = line.unwrap();
        println!("{}: {}", x, line);
        let parts: Vec<&str> = line.split("=").collect();
        if (parts.len() > 1) {
            vcf_header.header_dict.insert(
                parts[0].to_string(),
                parts[1].to_string(),
            );
        }

        if parts[0] == "##fileformat" {
            println!("File format is: {}", parts[1]);
        }
    }
    println!("{}", contents);
    for key in vcf_header.header_dict.keys() {
        println!("{}: {}", key, vcf_header.header_dict[key]);
    }
    (vcf_header)
}

#[pyfunction]
/// Formats the sum of two numbers as string
fn read_vcf_file(file_name: String) -> PyResult<HashMap<String, String>> {
    Ok(parse_vcf(file_name))
}
#[pyfunction]
fn return_object(file_name: String) -> Header {
    parse_vcf_to_header(file_name)
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn vcftools(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(read_vcf_file))?;
    m.add_wrapped(wrap_pyfunction!(return_object))?;
    Ok(())
}
