use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

use nom::{
    character::complete::{alpha1, anychar, char, line_ending, one_of, u32},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

advent_of_code::solution!(19);

#[derive(Clone)]
struct Rule {
    attribute: char,
    cmp: char,
    value: u32,
    next_workflow: String,
}

impl Rule {
    fn eval(&self, part: &Part) -> Option<String> {
        let v = part[&self.attribute];
        match self.cmp {
            '>' if v > self.value => Some(self.next_workflow.clone()),
            '<' if v < self.value => Some(self.next_workflow.clone()),
            _ => None,
        }
    }
}

struct Workflow {
    rules: Vec<Rule>,
    fallback: String,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let cond = tuple((one_of("xmas"), one_of("<>"), u32));
    let (i, ((attribute, cmp, value), next_wf)) = separated_pair(cond, char(':'), alpha1)(input)?;

    let rule = Rule {
        attribute,
        cmp,
        value,
        next_workflow: next_wf.to_string(),
    };

    Ok((i, rule))
}

fn parse_workflow(input: &str) -> IResult<&str, (String, Workflow)> {
    // let rules = separated_pair(many1(rule), char(','), alpha1);
    let rules = separated_list1(char(','), parse_rule);
    let fallback = preceded(char(','), alpha1);
    let workflow_body = delimited(char('{'), tuple((rules, fallback)), char('}'));
    let mut workflow = pair(alpha1, workflow_body);
    let (i, (name, (rules, fallback))) = workflow(input)?;

    let workflow = Workflow {
        rules,
        fallback: fallback.to_string(),
    };

    Ok((i, (name.to_string(), workflow)))
}

type Part = HashMap<char, u32>;
fn parse_part(input: &str) -> IResult<&str, Part> {
    let attr = separated_pair(anychar, char('='), u32);
    let (i, part_attrs) = delimited(char('{'), separated_list1(char(','), attr), char('}'))(input)?;

    let parts: Part = part_attrs.into_iter().collect();

    Ok((i, parts))
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<String, Workflow>, Vec<Part>)> {
    let workflows = terminated(separated_list1(line_ending, parse_workflow), line_ending);
    let parts = separated_list1(line_ending, parse_part);
    let (i, (workflows, parts)) = separated_pair(workflows, line_ending, parts)(input)?;

    let workflows: HashMap<String, Workflow> = workflows.into_iter().collect();

    Ok((i, (workflows, parts)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (workflows, parts)) = parse_input(input).finish().unwrap();

    Some(
        parts
            .iter()
            .filter_map(|part| {
                let mut next_wf = "in".to_string();
                loop {
                    let wf = &workflows[next_wf.as_str()];

                    next_wf = wf
                        .rules
                        .iter()
                        .find_map(|r| r.eval(part))
                        .unwrap_or(wf.fallback.to_string());

                    match next_wf.as_str() {
                        "A" => return Some(part.values().sum::<u32>()),
                        "R" => return None,
                        _ => (),
                    }
                }
            })
            .sum(),
    )
}

// The input range is mutated so it has the range matching the rule,
// the returned Range contains the non-matching range.
fn split_range_by_rule(ranges: &mut PartRanges, rule: &Rule) -> PartRanges {
    let mut excluded_ranges = ranges.clone();
    if rule.cmp == '<' {
        ranges.insert(rule.attribute, ranges[&rule.attribute].start..rule.value);
        excluded_ranges.insert(
            rule.attribute,
            rule.value..excluded_ranges[&rule.attribute].end,
        );
    } else {
        ranges.insert(rule.attribute, rule.value + 1..ranges[&rule.attribute].end);
        excluded_ranges.insert(
            rule.attribute,
            excluded_ranges[&rule.attribute].start..rule.value + 1,
        );
    }
    excluded_ranges
}

fn get_combinations(ranges: &PartRanges) -> usize {
    ranges
        .values()
        .map(|r| (r.end - r.start) as usize)
        .product()
}

type PartRanges = HashMap<char, Range<u32>>;
pub fn part_two(input: &str) -> Option<usize> {
    let (_, (workflows, _)) = parse_input(input).finish().unwrap();

    let possible_ranges: PartRanges = HashMap::from([
        ('x', 1..4001),
        ('m', 1..4001),
        ('a', 1..4001),
        ('s', 1..4001),
    ]);

    let mut q: VecDeque<(&str, PartRanges)> = VecDeque::new();
    q.push_back(("in", possible_ranges));

    let mut accepted_combinations: usize = 0;
    while let Some((wf_name, ranges)) = q.pop_front() {
        match wf_name {
            "R" => continue,
            "A" => {
                accepted_combinations += get_combinations(&ranges);
                continue;
            }
            _ => {
                let wf = &workflows[wf_name];
                let excluded_ranges = wf.rules.iter().fold(ranges, |mut acc, rule| {
                    let excluded_ranges = split_range_by_rule(&mut acc, rule);
                    q.push_back((&rule.next_workflow, acc));
                    excluded_ranges
                });

                q.push_back((&wf.fallback, excluded_ranges))
            }
        }
    }

    Some(accepted_combinations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
