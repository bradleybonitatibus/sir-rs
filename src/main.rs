mod config;
mod sir;

fn main() {
    let sir = sir::SIRModel::new();
    let sir = sir.run();
    sir.write_data();
}
