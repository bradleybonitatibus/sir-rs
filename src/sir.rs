use crate::config;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
struct SIR {
    susceptible: f64,
    infectious: f64,
    recovered: f64,
}

impl SIR {
    fn new(s: f64, i: f64, r: f64) -> SIR {
        SIR{
            susceptible: s,
            infectious: i,
            recovered: r,
        }
    }

    fn to_string(self) -> Vec<String> {
        vec![
            self.susceptible.to_string(),
            self.infectious.to_string(),
            self.recovered.to_string(),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct SIRModel {
    config: config::Config,
    data: Vec<SIR>,
}

impl SIRModel {
    pub fn new() -> SIRModel {
        let config = config::Config::new();
        SIRModel {
            config,
            data: Vec::new(),
        }
    }

    pub fn run(mut self) -> SIRModel {
        // mail ODE loop to caluclate each day's
        println!("Running");
        let mut counter: i32 = 0;
        let mut s = self.config.get_initial_s() as f64;
        let mut i = self.config.get_initial_i() as f64;
        let mut r = self.config.get_initial_r() as f64;
        let n = s + i + r;
        while self.config.get_steps() > counter {
            println!("Loop");
            let new_infections = self.config.get_beta() * s * i / n;
            let new_recovered = self.config.get_gamma() * i;
            let s_prime = -new_infections;
            let i_prime = new_infections - new_recovered;
            let r_prime = new_recovered;
            s = s + s_prime;
            i = i + i_prime;
            r = r + r_prime;
            println!("{} {} {}", s, i, r);
            let tmp: SIR = SIR::new(s, i, r);
            self.data.push(tmp);
            counter += 1;
        }
        self
    }

    pub fn write_data(self) {
        // let mut writer = csv::Writer::from_writer(vec![]);
        // for row in &self.data {
        //     writer.serialize(row).unwrap();
        // }

        let mut wtr = csv::WriterBuilder::new().from_path("results.csv").unwrap();
        for row in &self.data {
            wtr.write_record(&row.to_string()).unwrap();
        }

    }
}

