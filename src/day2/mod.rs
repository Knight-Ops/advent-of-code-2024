#[derive(Debug, Clone)]
pub struct ReactorReport {
    pub data: Vec<u32>,
}

#[derive(Debug, PartialEq)]
pub enum ReportDirection {
    Increasing,
    Decreasing,
}

#[derive(Debug, PartialEq)]
pub enum ReactorState {
    Safe,
    Unsafe,
}

pub fn input_generator(input: &str) -> Vec<ReactorReport> {
    input
        .lines()
        .map(|line| {
            let data = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            ReactorReport { data }
        })
        .collect()
}

pub fn part1(input: Vec<ReactorReport>) -> usize {
    input
        .iter()
        .map(|report| {
            let mut first_value = true;
            let mut report_direction = ReportDirection::Increasing;

            report
                .data
                .windows(2)
                .map(|window| {
                    // I don't like these accesses, but I'm not sure how to do it better
                    let diff = (window[1]) as i32 - (window[0]) as i32;
                    if first_value {
                        first_value = false;
                        return match diff {
                            0 => Some(ReactorState::Unsafe),
                            1 | 2 | 3 => {
                                report_direction = ReportDirection::Increasing;
                                Some(ReactorState::Safe)
                            }
                            -1 | -2 | -3 => {
                                report_direction = ReportDirection::Decreasing;
                                Some(ReactorState::Safe)
                            }
                            _ => Some(ReactorState::Unsafe),
                        };
                    } else {
                        match diff {
                            0 => return Some(ReactorState::Unsafe),
                            1 | 2 | 3 => {
                                if report_direction == ReportDirection::Decreasing {
                                    report_direction = ReportDirection::Increasing;
                                    return Some(ReactorState::Unsafe);
                                }
                            }
                            -1 | -2 | -3 => {
                                if report_direction == ReportDirection::Increasing {
                                    report_direction = ReportDirection::Decreasing;
                                    return Some(ReactorState::Unsafe);
                                }
                            }
                            _ => return Some(ReactorState::Unsafe),
                        }
                    }

                    Some(ReactorState::Safe)
                })
                .find(|x| x == &Some(ReactorState::Unsafe))
        })
        .filter(|x| x.is_none())
        .count()
}

pub fn part2(input: Vec<ReactorReport>) -> usize {
    let mut dampener_active = true;

    input
        .iter()
        .map(|report| {
            let mut first_value = true;
            let mut report_direction = ReportDirection::Increasing;

            report
                .data
                .windows(2)
                .map(|window| {
                    // I don't like these accesses, but I'm not sure how to do it better
                    let diff = (window[1]) as i32 - (window[0]) as i32;
                    if first_value {
                        first_value = false;
                        return match diff {
                            0 => Some(ReactorState::Unsafe),
                            1 | 2 | 3 => {
                                report_direction = ReportDirection::Increasing;
                                Some(ReactorState::Safe)
                            }
                            -1 | -2 | -3 => {
                                report_direction = ReportDirection::Decreasing;
                                Some(ReactorState::Safe)
                            }
                            _ => Some(ReactorState::Unsafe),
                        };
                    } else {
                        match diff {
                            0 => return Some(ReactorState::Unsafe),
                            1 | 2 | 3 => {
                                if report_direction == ReportDirection::Decreasing {
                                    report_direction = ReportDirection::Increasing;
                                    return Some(ReactorState::Unsafe);
                                }
                            }
                            -1 | -2 | -3 => {
                                if report_direction == ReportDirection::Increasing {
                                    report_direction = ReportDirection::Decreasing;
                                    return Some(ReactorState::Unsafe);
                                }
                            }
                            _ => return Some(ReactorState::Unsafe),
                        }
                    }

                    Some(ReactorState::Safe)
                })
                .find(|x| {
                    if dampener_active && x == &Some(ReactorState::Unsafe) {
                        dampener_active = false;
                        return false;
                    } else {
                        x == &Some(ReactorState::Unsafe)
                    }
                })
        })
        .filter(|x| x.is_none())
        .count()
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

    // macro_rules! test {
    //     ($func:ident, $val:expr) => {
    //         #[test]
    //         fn $func() {
    //             let name = module_path!().split("::").collect::<Vec<&str>>();
    //             let i = read_input_file(&format!(
    //                 "input/2024/{}_test.txt",
    //                 name[name.len() - 2].trim()
    //             ));

    //             let input = super::input_generator(&i);
    //             assert_eq!(super::$func(&input), $val);
    //         }
    //     };
    // }

    macro_rules! test_mut {
        ($func:ident, $val:expr) => {
            #[test]
            fn $func() {
                let name = module_path!().split("::").collect::<Vec<&str>>();
                let i = read_input_file(&format!(
                    "input/2024/{}_test.txt",
                    name[name.len() - 2].trim()
                ));

                let mut input = super::input_generator(&i);
                assert_eq!(super::$func(&mut input), $val);
            }
        };
    }

    test!(part1, 2);
    test!(part2, 4);
}
