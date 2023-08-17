enum HeightMode {
    Cm,
    In,
}
impl HeightMode {
    fn get_mode(hgt: &str) -> Result<Self, ()> {
        if hgt.contains("cm") {
            return Ok(Self::Cm);
        } else if hgt.contains("in") {
            return Ok(Self::In);
        }
        return Err(());
    }
}

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            byr: String::new(),
            iyr: String::new(),
            eyr: String::new(),
            hgt: String::new(),
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: None,
        }
    }

    fn parse_passport1(input: &String) -> Result<(), &'static str> {
        let mut passport = Passport::new();

        input.trim_start().split(' ').for_each(|i| {
            let identifier = i.get(0..3);
            let piece = i.trim_start().split(':').nth(1);
            match (identifier, piece) {
                (Some("byr"), Some(x)) => passport.byr = x.to_owned(),
                (Some("iyr"), Some(x)) => passport.iyr = x.to_owned(),
                (Some("eyr"), Some(x)) => passport.eyr = x.to_owned(),
                (Some("hgt"), Some(x)) => passport.hgt = x.to_owned(),
                (Some("hcl"), Some(x)) => passport.hcl = x.to_owned(),
                (Some("ecl"), Some(x)) => passport.ecl = x.to_owned(),
                (Some("pid"), Some(x)) => passport.pid = x.to_owned(),
                (Some("cid"), Some(x)) => passport.cid = Some(x.to_owned()),
                (_, _) => {}
            }
        });
        if passport.byr == ""
            || passport.iyr == ""
            || passport.eyr == ""
            || passport.hgt == ""
            || passport.hcl == ""
            || passport.ecl == ""
            || passport.pid == ""
        {
            return Err("shit don't work");
        }
        Ok(())
    }

    fn parse_passport2(input: &String) -> Result<(), ()> {
        let mut passport = Passport::new();

        input.trim_start().split(' ').for_each(|i| {
            let identifier = i.get(0..3);
            let piece = i.trim_start().split(':').nth(1);
            match (identifier, piece) {
                (Some("byr"), Some(x)) => passport.byr = x.to_owned(),
                (Some("iyr"), Some(x)) => passport.iyr = x.to_owned(),
                (Some("eyr"), Some(x)) => passport.eyr = x.to_owned(),
                (Some("hgt"), Some(x)) => passport.hgt = x.to_owned(),
                (Some("hcl"), Some(x)) => passport.hcl = x.to_owned(),
                (Some("ecl"), Some(x)) => passport.ecl = x.to_owned(),
                (Some("pid"), Some(x)) => passport.pid = x.to_owned(),
                (Some("cid"), Some(x)) => passport.cid = Some(x.to_owned()),
                (_, _) => {}
            }
        });
        let byr_parsed = passport.byr.parse::<usize>().unwrap_or(0);
        let byr_v = byr_parsed >= 1920 && byr_parsed <= 2002;
        let iyr_parsed = passport.iyr.parse::<usize>().unwrap_or(0);
        let iyr_v = iyr_parsed >= 2010 && iyr_parsed <= 2020;
        let eyr_parsed = passport.eyr.parse::<usize>().unwrap_or(0);
        let eyr_v = eyr_parsed >= 2020 && eyr_parsed <= 2030;
        let hgt_parsed = passport
            .hgt
            .chars()
            .take_while(|n| n.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(0);
        let hgt_mode = HeightMode::get_mode(&passport.hgt)?;
        let hgt_v = match hgt_mode {
            HeightMode::Cm => hgt_parsed >= 150 && hgt_parsed <= 193,
            HeightMode::In => hgt_parsed >= 59 && hgt_parsed <= 76,
        };
        let hcl_v = if let Some('#') = passport.hcl.chars().nth(0) {
            passport
                .hcl
                .chars()
                .skip(1)
                .take_while(|c| c.is_ascii_digit() || c.is_ascii_lowercase())
                .count()
                == 6
        } else {
            false
        };
        let ecl_v = passport.ecl == "amb"
            || passport.ecl == "blu"
            || passport.ecl == "brn"
            || passport.ecl == "gry"
            || passport.ecl == "grn"
            || passport.ecl == "hzl"
            || passport.ecl == "oth";
        let pid_v = passport.pid.chars().take_while(|pid| pid.is_ascii_digit()).count() == 9;

        if byr_v && iyr_v && eyr_v && hgt_v && hcl_v && ecl_v && pid_v {
            return Ok(());
        }
        Err(())
    }
}

fn main() {
    let contents = include_str!("../../inputs/day4.txt");

    let mut count1 = 0;
    let mut count2 = 0;
    let mut input = String::new();
    for line in contents.lines() {
        if line != "" {
            input.push(' ');
            input.push_str(line);
            continue;
        }
        if Passport::parse_passport1(&input).is_ok() {
            count1 += 1;
        }
        if Passport::parse_passport2(&input).is_ok() {
            count2 += 1;
        }
        input = String::new();
    }
    if Passport::parse_passport1(&input).is_ok() {
        count1 += 1;
    }
    if Passport::parse_passport2(&input).is_ok() {
        count2 += 1;
    }

    println!("Answer 1: {}", count1);
    println!("Answer 2: {}", count2);
}
