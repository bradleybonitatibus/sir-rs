#[derive(Debug, Copy, Clone)]
pub struct Config {
    gamma: f64,
    beta: f64,
    steps: i32,
    initial_s: f64,
    initial_i: f64,
    initial_r: f64,
}

impl Config {
    pub fn new() -> Config {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0); // remove program name from argument list
        let is_valid = validate_number_of_arguments(&args);
        if !is_valid {
            panic!("Invalid command line arguments passed");
        }
        Config::build(&args)
    }

    fn build(args: &Vec<String>) -> Config {
        let gamma = args[0].parse().expect("Failed to parse gamma");
        let beta = args[1].parse().expect("Failed to parse beta");
        let steps: i32 = args[2].parse().expect("Failed to parse steps");
        Config {
            gamma,
            beta,
            steps,
            initial_s: args[3].parse().expect("Failed to parse initial S"),
            initial_i: args[4].parse().expect("Failed to parse initial I"),
            initial_r: args[5].parse().expect("Failed to parse initial R"),
        }
    }
}

impl Config {
    pub fn get_steps(self) -> i32 {
        self.steps
    }

    pub fn get_gamma(self) -> f64 {
        self.gamma
    }

    pub fn get_beta(self) -> f64 {
        self.beta
    }

    pub fn get_initial_s(self) -> f64 {
        self.initial_s
    }

    pub fn get_initial_i(self) -> f64 {
        self.initial_i
    }

    pub fn get_initial_r(self) -> f64 {
        self.initial_r
    }
}

fn validate_number_of_arguments(args: &Vec<String>) -> bool {
    if args.len() < 6 {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_failed_validation() {
        let t: Vec<String> = vec![];
        assert_ne!(validate_number_of_arguments(&t), true);
    }

    #[test]
    fn test_validation_with_correct_number_of_args() {
        let t: Vec<String> = vec![
            String::from("0.05"),
            String::from("0.0001"),
            String::from("100"),
            String::from("10000"),
            String::from("1000"),
            String::from("0"),
        ];

        assert!(validate_number_of_arguments(&t));
    }

    #[test]
    fn test_builds_config_successfully() {
        let gamma = 0.05;
        let beta = 0.0001;
        let steps = 100;
        let s = 10000 as f64;
        let i = 1000 as f64;
        let r = 0 as f64;
        let t: Vec<String> = vec![
            gamma.to_string(),
            beta.to_string(),
            steps.to_string(),
            s.to_string(),
            i.to_string(),
            r.to_string(),
        ];

        let cfg = Config::build(&t);
        assert_eq!(cfg.get_gamma(), gamma);
        assert_eq!(cfg.get_beta(), beta);
        assert_eq!(cfg.get_steps(), steps);
        assert_eq!(cfg.get_initial_s(), s);
        assert_eq!(cfg.get_initial_i(), i);
        assert_eq!(cfg.get_initial_r(), r);
    }

    #[test]
    #[should_panic]
    fn test_builds_and_panics_with_bad_datatypes() {
        let t: Vec<String> = vec![
            String::from("hello"),
            2.to_string(),
            3.to_string(),
            4.to_string(),
            5.to_string(),
            6.to_string(),
        ];
        Config::build(&t);
    }
}
