use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, VecDeque},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char},
    combinator::{map, peek, value},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use num::integer::lcm;

advent_of_code::solution!(20);

const PUSH_TIMES: usize = 1000;

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
    Untyped,
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    destinations: BTreeSet<String>,
    state: bool,
    inputs: BTreeMap<String, bool>,
}

fn parse_module_configuration(input: &str) -> IResult<&str, Module> {
    let module_type = alt((
        value(ModuleType::Broadcast, peek(tag("broadcast"))),
        value(ModuleType::FlipFlop, char('%')),
        value(ModuleType::Conjunction, char('&')),
    ));
    let module_name = map(alpha1, |s: &str| s.to_string());
    let destinations = map(separated_list1(tag(", "), alpha1), |destinations| {
        BTreeSet::from_iter(destinations.iter().map(|s: &&str| s.to_string()))
    });
    let module_configuration = tuple((
        module_type,
        module_name,
        preceded(tag(" -> "), destinations),
    ));
    let (i, module) = map(module_configuration, |(module_type, name, destinations)| {
        Module {
            name,
            module_type,
            destinations,
            state: false,
            inputs: BTreeMap::new(),
        }
    })(input)?;

    Ok((i, module))
}

fn parse_input(input: &str) -> BTreeMap<String, RefCell<Module>> {
    let modules: BTreeMap<String, RefCell<Module>> = input
        .lines()
        .map(|line| {
            let (_, m) = parse_module_configuration(line).unwrap();
            (m.name.clone(), RefCell::new(m))
        })
        .collect();

    for (module_name, module) in &modules {
        for dest in &module.borrow().destinations {
            if let Some(dest_module) = modules.get(dest) {
                dest_module
                    .borrow_mut()
                    .inputs
                    .insert(module_name.clone(), false);
            }
        }
        // println!("{:?}", module.borrow());
    }

    modules
}

fn process_signal(
    modules: &BTreeMap<String, RefCell<Module>>,
    source: String,
    destination: String,
    signal: bool,
    queue: &mut VecDeque<(String, String, bool)>,
) {
    let untyped: std::cell::RefCell<Module> = RefCell::new(Module {
        name: "Dummy".to_owned(),
        module_type: ModuleType::Untyped,
        destinations: BTreeSet::new(),
        state: false,
        inputs: BTreeMap::new(),
    });
    // println!("{source} -{signal}-> {destination}");
    let module = modules.get(&destination).unwrap_or(&untyped);
    let mtype = module.borrow().module_type.clone();
    let signal_out = match (mtype, &signal) {
        (ModuleType::Broadcast, _) => {
            module.borrow().destinations.iter().for_each(|dest| {
                queue.push_back((module.borrow().name.clone(), dest.clone(), signal))
            });
            Some(signal)
        }
        (ModuleType::FlipFlop, false) => {
            module.borrow().destinations.iter().for_each(|dest| {
                queue.push_back((
                    module.borrow().name.clone(),
                    dest.clone(),
                    !module.borrow().state,
                ))
            });
            Some(!module.borrow().state)
        }
        (ModuleType::Conjunction, _) => {
            *module.borrow_mut().inputs.get_mut(&source).unwrap() = signal;
            let signal_out = !module.borrow().inputs.values().all(|x| *x);
            module.borrow().destinations.iter().for_each(|dest| {
                queue.push_back((module.borrow().name.clone(), dest.clone(), signal_out))
            });
            Some(signal_out)
        }
        _ => None,
    };

    if let Some(signal_out) = signal_out {
        module.borrow_mut().state = signal_out;
    };
}

pub fn part_one(input: &str) -> Option<u32> {
    let modules = parse_input(input);
    let mut pulses_high = 0;
    let mut pulses_low = 0;
    let queue: &mut VecDeque<(String, String, bool)> = &mut VecDeque::new();

    for _ in 0..PUSH_TIMES {
        // we could cache the state of the modules and the pulses delta and avoid
        // processing the same states over and over
        queue.push_back(("button".to_owned(), "broadcaster".to_owned(), false));

        while let Some((source, destination, signal)) = queue.pop_front() {
            match signal {
                true => pulses_high += 1,
                false => pulses_low += 1,
            }

            process_signal(&modules, source, destination, signal, queue);
        }
    }

    Some(pulses_high * pulses_low)
}

pub fn part_two(input: &str) -> Option<u64> {
    let modules = parse_input(input);
    let mut found = false;
    let queue: &mut VecDeque<(String, String, bool)> = &mut VecDeque::new();
    let mut qb_input_cycles: BTreeMap<String, u32> = BTreeMap::new();
    let mut i = 0;

    while !found {
        queue.push_back(("button".to_owned(), "broadcaster".to_owned(), false));
        i += 1;

        while let Some((source, destination, signal)) = queue.pop_front() {
            // this solution is only valid for the given input
            if destination == "qb" && signal {
                qb_input_cycles.entry(source.clone()).or_insert(i);
                if qb_input_cycles.len() == 4 {
                    // println!("All 4 inputs to qb seen as high: {:?}", qb_input_cycles);
                    found = true;
                }
            }

            process_signal(&modules, source, destination, signal, queue);
        }
    }

    let min_pushes = qb_input_cycles
        .values()
        .fold(1u64, |acc, &v| lcm(acc, v as u64));

    Some(min_pushes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result_0 = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 0,
        ));
        assert_eq!(result_0, Some(32000000));

        let result_1 = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result_1, Some(11687500));
    }
}
