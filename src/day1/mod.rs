use std::collections::HashMap;

#[derive(Debug)]
pub struct Lists {
    first: Vec<u32>,
    second: Vec<u32>,
}

impl Lists {
    pub fn sort(&mut self) {
        self.first.sort();
        self.second.sort();
    }
}

pub fn input_generator(input: &str) -> Lists {
    let mut lists = Lists {
        first: Vec::new(),
        second: Vec::new(),
    };

    input.lines().for_each(|line| {
        // INSERT PARSING CODE HERE
        let mut entries = line.split_whitespace().map(|x| x.parse().unwrap());

        lists.first.push(entries.next().unwrap());
        lists.second.push(entries.next().unwrap());
    });

    lists
}

pub fn part1(mut input: Lists) -> usize {
    input.sort();

    input
        .first
        .iter()
        .zip(input.second.iter())
        .map(|(first, second)| first.abs_diff(*second) as usize)
        .sum()
}

pub fn part2(mut input: Lists) -> usize {
    let mut second_map = HashMap::new();

    // Build the second map so we don't iterate over the second list multiple times
    input.second.iter().for_each(|&x| {
        second_map.entry(x).and_modify(|x| *x += 1).or_insert(1);
    });

    input
        .first
        .iter()
        .map(|&x| (x * *second_map.entry(x).or_default()) as usize)
        .sum()
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
    }

    test!(part1, 11);
    test!(part2, 31);
}
