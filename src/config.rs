#[derive(Debug, Copy, Clone)]
pub struct Config {
    gamma: f64, // 
    beta: f64,
    steps: i32,
    initial_s: i64,
    initial_i: i64,
    initial_r: i64
}

impl Config {
    pub fn new() -> Config {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0); // remove program name from argument list
        let is_valid = validate_args(&args);
        if !is_valid {
            panic!("Invalid command line arguments passed");
        }
        let gamma = args[0].parse().unwrap();
        let beta = args[1].parse().unwrap();
        let steps = args[2].parse().unwrap();
        Config {
            gamma,
            beta,
            steps,
            initial_s: args[3].parse().unwrap(),
            initial_i: args[4].parse().unwrap(),
            initial_r: args[5].parse().unwrap(),
        }
    }

    pub fn get_steps(self) -> i32 {
        self.steps
    }

    pub fn get_gamma(self) -> f64 {
        self.gamma
    }

    pub fn get_beta(self) -> f64 {
        self.beta
    }

    pub fn get_initial_s(self) -> i64 {
        self.initial_s
    }

    pub fn get_initial_i(self) -> i64 {
        self.initial_i
    }

    pub fn get_initial_r(self) -> i64 {
        self.initial_r
    }
}

fn validate_args(args: &Vec<String>) -> bool {
    if args.len() < 6 {
        return false;
    }
    return true;
}
