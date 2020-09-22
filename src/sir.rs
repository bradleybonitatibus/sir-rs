use crate::config;

#[derive(Debug, Copy, Clone)]
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
        let mut counter: i32 = 0;
        let mut s = self.config.get_initial_s();
        let mut i = self.config.get_initial_i();
        let mut r = self.config.get_initial_r();
        let gamma = self.config.get_gamma();
        let beta = self.config.get_beta();
        let steps = self.config.get_steps();
        while counter < steps {
            let new_infections = beta * (s * i);
            let new_recovered = gamma * i;
            let delta_s = -new_infections;
            let delta_i = new_infections - new_recovered;
            let delta_r = new_recovered;
            s += delta_s;
            i += delta_i;
            r += delta_r;
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

