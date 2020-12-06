use regex::Regex;
use std::collections::HashMap;

pub fn part1(lines: Vec<String>) -> usize {
    count_valid_passports(lines, valid_field)
}

pub fn part2(lines: Vec<String>) -> usize {
    count_valid_passports(lines, validate_field_and_values)
}

fn count_valid_passports<F>(lines: Vec<String>, validator: F) -> usize
where
    F: Fn(&str) -> i8,
{
    let mut valid_fields = 0;
    let mut valid_passports = 0;
    for line in lines.iter() {
        if !line.is_empty() {
            let passport_data = line.split_whitespace();
            for data in passport_data {
                valid_fields = valid_fields + validator(data);
            }

            if valid_fields >= 7 {
                valid_passports = valid_passports + 1;
                valid_fields = 0;
            }
        } else {
            if valid_fields >= 7 {
                valid_passports = valid_passports + 1;
            }
            valid_fields = 0;
        }
    }

    valid_passports
}

fn fields() -> HashMap<&'static str, Box<dyn Validator>> {
    let mut urv = HashMap::new();
    urv.insert(String::from("cm"), RangeValidator { min: 150, max: 193 });
    urv.insert(String::from("in"), RangeValidator { min: 59, max: 76 });
    let mut fields: HashMap<&str, Box<dyn Validator>> = HashMap::new();
    fields.insert(
        "byr",
        Box::new(RangeValidator {
            min: 1920,
            max: 2002,
        }),
    );
    fields.insert(
        "iyr",
        Box::new(RangeValidator {
            min: 2010,
            max: 2020,
        }),
    );
    fields.insert(
        "eyr",
        Box::new(RangeValidator {
            min: 2020,
            max: 2030,
        }),
    );
    fields.insert("hgt", Box::new(UnitRangeValidator { rules: urv }));
    fields.insert(
        "hcl",
        Box::new(RegexValidator {
            regex: Regex::new(r"^#([0-9a-f]{6})$").unwrap(),
        }),
    );
    fields.insert(
        "ecl",
        Box::new(RegexValidator {
            regex: Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap(),
        }),
    );
    fields.insert(
        "pid",
        Box::new(RegexValidator {
            regex: Regex::new(r"^([0-9]{9})$").unwrap(),
        }),
    );

    fields
}

fn valid_field(field: &str) -> i8 {
    let mut data = field.split(":");
    fields().get(data.next().unwrap()).map_or(0, |_| 1)
}

fn validate_field_and_values(field: &str) -> i8 {
    if valid_field(field) == 1 {
        let mut data = field.split(":");
        fields()
            .get(data.next().unwrap())
            .unwrap()
            .validate(data.next().unwrap().to_string()) as i8
    } else {
        0
    }
}

impl Validator for RangeValidator {
    fn validate(&self, value: String) -> bool {
        *&self.min <= value.parse::<usize>().unwrap()
            && &value.parse::<usize>().unwrap() <= &self.max
    }
}

impl Validator for UnitRangeValidator {
    fn validate(&self, value: String) -> bool {
        let v = value as String;
        let mut unit = "cm";
        let mut num = v.strip_suffix(unit);
        if num.is_none() {
            unit = "in";
            num = v.strip_suffix(unit);
        }
        if num.is_none() {
            false
        } else {
            *&self
                .rules
                .get(unit)
                .unwrap()
                .validate(num.unwrap().parse().unwrap())
        }
    }
}

impl Validator for RegexValidator {
    fn validate(&self, value: String) -> bool {
        *&self.regex.is_match(value.as_str())
    }
}

struct RangeValidator {
    min: usize,
    max: usize,
}

struct UnitRangeValidator {
    rules: HashMap<String, RangeValidator>,
}
struct RegexValidator {
    regex: Regex,
}

trait Validator {
    fn validate(&self, value: String) -> bool;
}
