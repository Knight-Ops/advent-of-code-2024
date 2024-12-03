pub mod day1;
pub mod day2;
pub mod day3;

pub fn read_input_file(input: &str) -> String {
    let file = std::fs::read_to_string(input)
        .expect("Error while reading provided file name")
        .to_string();
    file
}
