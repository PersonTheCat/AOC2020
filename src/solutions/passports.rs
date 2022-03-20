use crate::solution_template::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::Split;

lazy_static! {
    /// The pattern used for checking years.
    static ref YEAR_PATTERN: Regex = Regex::new(r"^\d{4}$").unwrap();

    /// The pattern used for checking height values.
    static ref HEIGHT_PATTERN: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();

    /// The pattern used for checking color codes.
    static ref COLOR_CODE_PATTERN: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();

    /// The pattern used for checking color values.
    static ref COLOR_NAME_PATTERN: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();

    /// The pattern used for verifying passport IDs.
    static ref PASSPORT_ID_PATTERN: Regex = Regex::new(r"^\d{9}$").unwrap();
}

pub struct PassportSolution;

impl Solution for PassportSolution {
    type Data = Vec<HashMap<String, String>>;
    type Output = i32;

    const MESSAGE_A: &'static str = "Valid passports (num)";
    const MESSAGE_B: &'static str = "Valid passports (num+chk)";

    fn from_string(s: &str) -> Vec<HashMap<String, String>> {
        Self::map_clusters(s, to_map)
    }

    /// Counts the total number of "valid" passports in an array, according
    /// to the rules defined in step one.
    fn get_solution_a(data: &Vec<HashMap<String, String>>) -> Option<i32> {
        let mut count = 0;
        for map in data {
            if contains_needed_fields(map) {
                count += 1;
            }
        }
        Some(count)
    }

    /// Counts the number of valid passports, verifying that their fields
    /// contain the expected data.
    fn get_solution_b(data: &Vec<HashMap<String, String>>) -> Option<i32> {
        let mut count = 0;
        for map in data {
            if contains_needed_fields(map) && is_valid(map) {
                count += 1;
            }
        }
        Some(count)
    }
}

/// Converts a collection of key-value pairs, separated by whitespace
/// and assigned by `:`, into a hashmap containing the same data.
fn to_map(data: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for p in data.split_whitespace() {
        let mut kv = p.split(':');
        map.insert(read_kv_next(&mut kv, p), read_kv_next(&mut kv, p));
    }
    map
}

fn read_kv_next(kv: &mut Split<char>, source: &str) -> String {
    kv.next()
        .expect(&format!("invalid kv pair: {}", source))
        .to_string()
}

/// Determines whether a passport contains all of the 8 expected keys,
/// not including `cid`, which may be ignored.
fn contains_needed_fields(map: &HashMap<String, String>) -> bool {
    // Because the map never contains unknown keys, we can simply
    // check the length. This may not hold up in future challenges.
    map.len() == 8 || (map.len() == 7 && !map.contains_key("cid"))
}

/// Determines whether each key-value pair in the map contains the expected data.
fn is_valid(map: &HashMap<String, String>) -> bool {
    for (k, v) in map {
        if !is_pair_valid(k, v) {
            return false;
        }
    }
    true
}

/// Checks a key-value pair to make sure it contains the expected data.
///
/// byr (Birth Year) - four digits; at least 1920 and at most 2002.
/// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
/// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
/// hgt (Height) - a number followed by either cm or in:
///  * If cm, the number must be at least 150 and at most 193.
///  * If in, the number must be at least 59 and at most 76.
/// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
/// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
/// pid (Passport ID) - a nine-digit number, including leading zeroes.
/// cid (Country ID) - ignored, missing or not.
fn is_pair_valid(k: &str, v: &str) -> bool {
    match k {
        "byr" => check_year(v, 1920, 2002),
        "iyr" => check_year(v, 2010, 2020),
        "eyr" => check_year(v, 2020, 2030),
        "hgt" => check_height(v),
        "hcl" => COLOR_CODE_PATTERN.is_match(v),
        "ecl" => COLOR_NAME_PATTERN.is_match(v),
        "pid" => PASSPORT_ID_PATTERN.is_match(v),
        "cid" => true,
        _ => panic!("Unknown key in passport: {}", k),
    }
}

/// Determines whether `v` is a year number and in the expected range.
fn check_year(v: &str, min: i32, max: i32) -> bool {
    if !YEAR_PATTERN.is_match(v) {
        return false;
    }
    let year: i32 = v.parse().unwrap();
    year >= min && year <= max
}

/// Determines whether this is a valid height, in cm or in.
fn check_height(v: &str) -> bool {
    if !HEIGHT_PATTERN.is_match(v) {
        return false;
    }
    let captures = HEIGHT_PATTERN.captures(v).unwrap();
    let num: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
    let unit = captures.get(2).unwrap().as_str();

    match unit {
        "cm" => num >= 150 && num <= 193,
        "in" => num >= 59 && num <= 76,
        _ => unreachable!(),
    }
}

#[test]
fn test_valid_passport() {
    let valid1 = "byr:1 iyr:2 eyr:3 hgt:4 hcl:5 ecl:6 pid:7 cid:8";
    let valid2 = "eyr:3 pid:7 iyr:2 hgt:4 hcl:5 cid:8 byr:1 ecl:6";
    let valid3 = "byr:1 iyr:2 eyr:3 hgt:4 hcl:5 ecl:6 pid:7";
    let invalid1 = "iyr:2 eyr:3 hgt:4 hcl:5 ecl:6 cid:8";
    let invalid2 = "eyr:3 iyr:2 hgt:4 hcl:5 cid:8 ecl:6";
    let invalid3 = "byr:1 iyr:2 hgt:4 hcl:5 ecl:6 pid:7";

    let three_valid = vec![
        to_map(valid1),
        to_map(valid2),
        to_map(valid3),
        to_map(invalid1),
        to_map(invalid2),
    ];
    let two_valid = vec![
        to_map(valid1),
        to_map(valid3),
        to_map(invalid1),
        to_map(invalid2),
        to_map(invalid3),
    ];
    assert_eq!(PassportSolution::get_solution_a(&three_valid).unwrap(), 3);
    assert_eq!(PassportSolution::get_solution_a(&two_valid).unwrap(), 2);
}

/// byr (Birth Year) - four digits; at least 1920 and at most 2002.
/// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
/// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
/// hgt (Height) - a number followed by either cm or in:
///  * If cm, the number must be at least 150 and at most 193.
///  * If in, the number must be at least 59 and at most 76.
/// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
/// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
/// pid (Passport ID) - a nine-digit number, including leading zeroes.
/// cid (Country ID) - ignored, missing or not.
#[test]
fn test_valid_passport_fields() {
    // Each of these contains all valid fields, ignoring `cid`.
    let valid1 = "byr:1980 iyr:2015 eyr:2025 hgt:150cm hcl:#9999FF ecl:amb pid:123456789 cid:na";
    let valid2 = "byr:1920 iyr:2010 eyr:2020 hgt:193cm hcl:#123456 ecl:blu pid:000000009 cid:na";
    let valid3 = "byr:2002 iyr:2020 eyr:2030 hgt:59in hcl:#abcdef ecl:brn pid:100000000 cid:na";
    let valid4 = "byr:1975 iyr:2011 eyr:2021 hgt:76in hcl:#a1c3ef ecl:oth pid:590498502 cid:na";
    // Each of these contains exactly one invalid field.
    let invalid1 = "byr:1919 iyr:2015 eyr:2025 hgt:150cm hcl:#9999FF ecl:amb pid:123456789 cid:na";
    let invalid2 = "byr:1980 iyr:2009 eyr:2025 hgt:150cm hcl:#9999FF ecl:amb pid:123456789 cid:na";
    let invalid3 = "byr:1980 iyr:2015 eyr:2019 hgt:150cm hcl:#9999FF ecl:amb pid:123456789 cid:na";
    let invalid4 = "byr:1980 iyr:2015 eyr:2025 hgt:149cm hcl:#9999FF ecl:amb pid:123456789 cid:na";
    let invalid5 = "byr:1980 iyr:2015 eyr:2025 hgt:150cm hcl:#9999GG ecl:non pid:12345678 cid:na";
    let invalid6 = "byr:1980 iyr:2015 eyr:2025 hgt:999in hcl:#9999FF ecl:amb pid:123456789 cid:na";
    let invalid7 = "byr:1980 iyr:2015 eyr:2025 hgt:150cm hcl:#0000 ecl:amb pid:123456789 cid:na";
    let invalid8 = "byr:1980 iyr:2015 eyr:2025 hgt:194cm hcl:#9999FF ecl:amb pid:123456789 cid:na";
    let invalid9 = "byr:2003 iyr:2015 eyr:2025 hgt:150cm hcl:#9999FF ecl:amb pid:123456789 cid:na";
    let invalid0 = "byr:1980 iyr:2015 eyr:2025 hgt:150cm hcl:#9999FF ecl:amb pid:1234567890 cid:na";

    assert!(is_valid(&to_map(valid1)));
    assert!(is_valid(&to_map(valid2)));
    assert!(is_valid(&to_map(valid3)));
    assert!(is_valid(&to_map(valid4)));
    assert!(!is_valid(&to_map(invalid1)));
    assert!(!is_valid(&to_map(invalid2)));
    assert!(!is_valid(&to_map(invalid3)));
    assert!(!is_valid(&to_map(invalid4)));
    assert!(!is_valid(&to_map(invalid5)));
    assert!(!is_valid(&to_map(invalid6)));
    assert!(!is_valid(&to_map(invalid7)));
    assert!(!is_valid(&to_map(invalid8)));
    assert!(!is_valid(&to_map(invalid9)));
    assert!(!is_valid(&to_map(invalid0)));
}
