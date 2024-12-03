use pest::Parser;
use pest_derive::Parser;
use regex::Regex;

#[derive(Parser)]
#[grammar = "day3/day3.pest"]
pub struct Day3Parser;

pub fn input_generator(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    // This gets and unwraps the "program" rule from the input
    let parse = Day3Parser::parse(Rule::program, input)
        .unwrap()
        .next()
        .unwrap();

    parse
        .into_inner()
        .map(|pair| match pair.as_rule() {
            Rule::operation => {
                let mut operation_pair = pair.into_inner();
                match operation_pair.next().unwrap().as_str() {
                    "mul" => {
                        let first = operation_pair
                            .next()
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap();
                        let second = operation_pair
                            .next()
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap();

                        first * second
                    }
                    _ => unimplemented!(),
                }
            }
            _ => 0,
        })
        .sum()
}

pub fn part1_regex(input: &str) -> usize {
    let re = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [op, val1, val2])| {
            if op == "mul" {
                val1.parse::<usize>().unwrap() * val2.parse::<usize>().unwrap()
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut multiply_active = true;

    // This gets and unwraps the "program" rule from the input
    let parse = Day3Parser::parse(Rule::program, input)
        .unwrap()
        .next()
        .unwrap();

    parse
        .into_inner()
        .map(|pair| match pair.as_rule() {
            Rule::operation => {
                let mut operation_pair = pair.into_inner();
                match operation_pair.next().unwrap().as_str() {
                    "mul" => {
                        if multiply_active {
                            // Extract the first `value` and the second `value` multiply them.
                            operation_pair
                                .next()
                                .unwrap()
                                .as_str()
                                .parse::<usize>()
                                .unwrap()
                                * operation_pair
                                    .next()
                                    .unwrap()
                                    .as_str()
                                    .parse::<usize>()
                                    .unwrap()
                        } else {
                            0
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            Rule::conditional => match pair.as_str() {
                "do()" => {
                    multiply_active = true;
                    0
                }
                "don't()" => {
                    multiply_active = false;
                    0
                }
                _ => unimplemented!(),
            },
            _ => 0,
        })
        .sum()
}

pub fn part2_regex(input: &str) -> usize {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").unwrap();

    let mut multiply_active = true;

    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [op])| match op.split("(").nth(0).unwrap() {
            "mul" => {
                if multiply_active {
                    op.split("(")
                        .nth(1)
                        .unwrap()
                        .split(")")
                        .nth(0)
                        .unwrap()
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .product()
                } else {
                    0
                }
            }
            "do" => {
                multiply_active = true;
                0
            }
            "don't" => {
                multiply_active = false;
                0
            }
            _ => unreachable!(),
        })
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

    test!(part1, 161);
    test!(part1_regex, 161);
    test!(part2, 48, "test2");
    test!(part2_regex, 48, "test2");
}
