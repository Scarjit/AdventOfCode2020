use stackvec::TryCollect;

#[derive(Debug, Clone)]
pub enum Height {
    Centimeter(u32),
    Inch(u32),
    Unknown,
}

impl Height {
    pub fn parse_str(inp: &str) -> Height {
        if inp.len() < 3 {
            return Self::Unknown;
        }
        let split = inp.split_at(inp.len() - 2);
        let h = match split.0.parse::<u32>() {
            Ok(v) => v,
            Err(_) => return Self::Unknown,
        };

        match split.1 {
            "cm" => Self::Centimeter(h),
            "in" => Self::Inch(h),
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
    Unknown,
}

impl EyeColor {
    pub fn parse_str(inp: &str) -> Self {
        match inp {
            "amb" => Self::Amber,
            "blu" => Self::Blue,
            "brn" => Self::Brown,
            "gry" => Self::Grey,
            "grn" => Self::Green,
            "hzl" => Self::Hazel,
            "oth" => Self::Other,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<EyeColor>,
    pid: Option<String>,
    //cid: Option<String>
}

impl Passport {
    pub fn parse_splits(inp: &str) -> Option<Self> {
        let mut passport: Passport = Passport::default();
        let invec = inp
            .split(|c| c == '\n' || c == ' ')
            .map(|s| s.split(':').try_collect::<[&str;2]>().unwrap())
            .collect::<Vec<[&str;2]>>();

        for sx in invec {
            match sx[0] {
                "byr" => {
                    passport.byr = Some(sx[1].parse().unwrap());
                }
                "iyr" => {
                    passport.iyr = Some(sx[1].parse().unwrap());
                }
                "eyr" => {
                    passport.eyr = Some(sx[1].parse().unwrap());
                }
                "hgt" => {
                    passport.hgt = Some(Height::parse_str(sx[1]));
                }
                "hcl" => {
                    passport.hcl = Some(sx[1].to_string());
                }
                "ecl" => {
                    passport.ecl = Some(EyeColor::parse_str(sx[1]));
                }
                "pid" => {
                    passport.pid = Some(sx[1].to_string());
                }
                "cid" => {
                    continue;
                    //passport.cid = Some(sx[1].to_string())
                }
                _ => {
                    return None;
                }
            }
        }

        Some(passport)
    }

    #[inline]
    fn has_req_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn validate_byr(&self) -> bool {
        match self.byr {
            None => false,
            Some(v) => v >= 1920 && v <= 2002,
        }
    }

    fn validate_iyr(&self) -> bool {
        match self.iyr {
            None => false,
            Some(v) => v >= 2010 && v <= 2020,
        }
    }

    fn validate_eyr(&self) -> bool {
        match self.eyr {
            None => false,
            Some(v) => v >= 2020 && v <= 2030,
        }
    }

    fn validate_hgt(&self) -> bool {
        match &self.hgt {
            None => false,
            Some(v) => match v {
                Height::Centimeter(c) => c >= &150 && c <= &193,
                Height::Inch(i) => i >= &59 && i <= &76,
                Height::Unknown => false,
            },
        }
    }

    fn validate_hcl(&self) -> bool {
        match &self.hcl {
            None => false,
            Some(hexstr) => {
                hexstr.starts_with('#')
                    && hexstr.len() == 7
                    && hexstr[6..].chars().all(|c| c.is_ascii_hexdigit())
            }
        }
    }

    fn validate_ecl(&self) -> bool {
        match &self.ecl {
            None => false,
            Some(v) => match v {
                EyeColor::Unknown => false,
                _ => true,
            },
        }
    }

    fn validate_pid(&self) -> bool {
        match &self.pid {
            None => false,
            Some(v) => {
                if v.len() != 9 {
                    return false;
                }
                v.parse::<u32>().is_ok()
            }
        }
    }

    fn is_valid(&self) -> bool {
        self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_pid()
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Option<Passport>> {
    let parts = input
        .split("\n\n")
        .map(|l| Passport::parse_splits(l))
        .collect::<Vec<Option<Passport>>>();
    parts
}

#[aoc(day4, part1)]
pub fn solve_part_1(input: &Vec<Option<Passport>>) -> usize {
    input
        .iter()
        .filter(|f| match f {
            None => false,
            Some(v) => v.has_req_fields(),
        })
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part_2(input: &Vec<Option<Passport>>) -> usize {
    input
        .iter()
        .filter(|f| match f {
            None => false,
            Some(v) => v.is_valid(),
        })
        .count()
}
