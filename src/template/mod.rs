pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            // INSERT PARSING CODE HERE
            unimplemented!()
        })
        .collect()
}

pub fn part1(input: Vec<u32>) -> usize {
    unimplemented!()
}

pub fn part2(input: Vec<u32>) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;
    macro_rules! test {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2024/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(input), $val);
            }
        };
        ($func:ident, $val:expr, $f:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2024/{}_{}.txt",
                    name[name.len() - 2].trim(),
                    $f
                ));

                let input = super::input_generator(&i);
                assert_eq!(super::$func(input), $val);
            }
        };
    }

    test!(part1, 0);
    test!(part2, 0);
}
