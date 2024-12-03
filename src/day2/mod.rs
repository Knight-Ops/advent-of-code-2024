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
    UnsafeZero,
    UnsafeSwap,
    UnsafeStepSize,
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
                            0 => Some(ReactorState::UnsafeZero),
                            1 | 2 | 3 => {
                                report_direction = ReportDirection::Increasing;
                                Some(ReactorState::Safe)
                            }
                            -1 | -2 | -3 => {
                                report_direction = ReportDirection::Decreasing;
                                Some(ReactorState::Safe)
                            }
                            _ => Some(ReactorState::UnsafeStepSize),
                        };
                    } else {
                        match diff {
                            0 => return Some(ReactorState::UnsafeZero),
                            1 | 2 | 3 => {
                                if report_direction == ReportDirection::Decreasing {
                                    report_direction = ReportDirection::Increasing;
                                    return Some(ReactorState::UnsafeSwap);
                                }
                            }
                            -1 | -2 | -3 => {
                                if report_direction == ReportDirection::Increasing {
                                    report_direction = ReportDirection::Decreasing;
                                    return Some(ReactorState::UnsafeSwap);
                                }
                            }
                            _ => return Some(ReactorState::UnsafeStepSize),
                        }
                    }

                    Some(ReactorState::Safe)
                })
                .find(|x| x != &Some(ReactorState::Safe))
        })
        .filter(|x| x.is_none())
        .count()
}

pub fn part2(input: Vec<ReactorReport>) -> usize {
    input
        .iter()
        .map(|report| {
            let mut dampener_active = true;
            let mut double_swap = false;
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
                        let first_validation = match diff {
                            0 => Some(ReactorState::UnsafeZero),
                            1 | 2 | 3 => {
                                report_direction = ReportDirection::Increasing;
                                Some(ReactorState::Safe)
                            }
                            -1 | -2 | -3 => {
                                report_direction = ReportDirection::Decreasing;
                                Some(ReactorState::Safe)
                            }
                            _ => Some(ReactorState::UnsafeStepSize),
                        };
                        return first_validation;
                    } else {
                        let validation = match diff {
                            0 => Some(ReactorState::UnsafeZero),
                            1 | 2 | 3 => {
                                if report_direction == ReportDirection::Decreasing {
                                    report_direction = ReportDirection::Increasing;
                                    Some(ReactorState::UnsafeSwap)
                                } else {
                                    Some(ReactorState::Safe)
                                }
                            }
                            -1 | -2 | -3 => {
                                if report_direction == ReportDirection::Increasing {
                                    report_direction = ReportDirection::Decreasing;
                                    Some(ReactorState::UnsafeSwap)
                                } else {
                                    Some(ReactorState::Safe)
                                }
                            }
                            _ => Some(ReactorState::UnsafeStepSize),
                        };
                        return validation;
                    }
                })
                .find(|x| match (dampener_active, x) {
                    (true, Some(ReactorState::UnsafeZero)) => {
                        dampener_active = false;
                        return false;
                    }
                    (true, Some(ReactorState::UnsafeSwap)) => {
                        dampener_active = false;
                        return false;
                    }
                    // Special case for the double swap, which appears in the example of 1, 3, 2, 4, 5 - technicall you could remove the 3 or 2 and it is valid.
                    (false, Some(ReactorState::UnsafeSwap)) => {
                        if double_swap {
                            return true;
                        } else {
                            double_swap = true;
                            return false;
                        }
                    }
                    _ => x != &Some(ReactorState::Safe),
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

    test!(part1, 2);
    test!(part2, 4);
}
