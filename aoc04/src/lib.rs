#[macro_use] extern crate lazy_static;
use regex::Regex;

pub mod input;

#[derive(Default)]
pub struct Passport {
    byr: Option<String>, // (Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<String>, // (Country ID)
}

pub fn parse_input(input: &str) -> Vec<Passport> {
    input
        .lines()
        .fold(vec![Passport::default()], |mut passports, line| {
            if line.len() == 0 {
                passports.push(Passport::default());
            } else {
                let passport = passports.last_mut().unwrap();
                line.split(' ').for_each(|entry| {
                    let split: Vec<&str> = entry.split(':').collect();
                    passport.set_value(split[0], split[1].to_string());
                });
            }

            passports
        })
}

impl Passport {
    fn set_value(&mut self, key: &str, value: String) {
        match key {
            "byr" => self.byr = Some(value),
            "iyr" => self.iyr = Some(value),
            "eyr" => self.eyr = Some(value),
            "hgt" => self.hgt = Some(value),
            "hcl" => self.hcl = Some(value),
            "ecl" => self.ecl = Some(value),
            "pid" => self.pid = Some(value),
            "cid" => self.cid = Some(value),
            _ => panic!("Parse error: key not found"),
        }
    }

    pub fn is_valid_a(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid_b(&self) -> bool {
        lazy_static! {
            static ref RE_HGT: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
            static ref RE_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref RE_ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        // When part a is valid, we can safely unwrap
        if !self.is_valid_a() { return false; }

        let byr: u16 = self.byr.as_ref().unwrap().parse().unwrap_or_default();
        let iyr: u16 = self.iyr.as_ref().unwrap().parse().unwrap_or_default();
        let eyr: u16 = self.eyr.as_ref().unwrap().parse().unwrap_or_default();
        let hgt = self.hgt.as_ref().unwrap();
        let hcl = self.hcl.as_ref().unwrap();
        let ecl = self.ecl.as_ref().unwrap();
        let pid = self.pid.as_ref().unwrap();

        byr >= 1920 && byr <= 2002 &&
        iyr >= 2010 && iyr <= 2020 &&
        eyr >= 2020 && eyr <= 2030 &&
        RE_HGT.captures(hgt)
            .map(|caps|
                caps[1]
                    .parse::<u16>()
                    .map(|i| {
                            (&caps[2] == "cm" && i >= 150 && i <= 193) ||
                            (&caps[2] == "in" && i >= 59 && i <= 76)
                    })
                    .unwrap()
            )
            .unwrap_or(false) &&
        RE_HCL.is_match(hcl) &&
        RE_ECL.is_match(ecl) &&
        RE_PID.is_match(pid)
    }
}
