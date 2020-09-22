#[derive(Debug, Copy, Clone)]
pub struct Config {
    gamma: f64, // 
    beta: f64,
    steps: i32,
    initial_s: f64,
    initial_i: f64,
    initial_r: f64
}

impl Config {
    pub fn new() -> Config {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0); // remove program name from argument list
        let is_valid = validate_number_of_arguments(&args);
        if !is_valid {
            panic!("Invalid command line arguments passed");
        }
        let gamma = args[0].parse().unwrap();
        let beta = args[1].parse().unwrap();
        let steps: i32 = args[2].parse().unwrap();
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
    use super::{validate_number_of_arguments};
    #[test]
    fn test_failed_validation() {
        let t : Vec<String> = vec![];
        assert_ne!(validate_number_of_arguments(&t), true);
    }

    #[test]
    fn test_validation_with_correct_number_of_args() {
        let t : Vec<String> = vec![
            String::from("0.05"),
            String::from("0.0001"),
            String::from("100",),
            String::from("10000"),
            String::from("1000",),
            String::from("0"),
        ];

        assert!(validate_number_of_arguments(&t));
    }
}