use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HEX: Regex = Regex::new(r"^[0-9a-f]+$").unwrap();
    static ref HGT_FORMAT: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    static ref ALL_NUM: Regex = Regex::new(r"^[0-9]+$").unwrap();
}

pub fn validate_byr(byr: &str) -> bool {
    match byr.parse::<i32>() {
        Ok(x) => x >= 1920 && x <= 2002,
        Err(_) => false,
    }
}

#[cfg(test)]
mod validate_byr {
    use super::validate_byr;

    #[test]
    fn it_works() {
        assert_eq!(validate_byr("abc"), false);
        assert_eq!(validate_byr("1919"), false);
        assert_eq!(validate_byr("1920"), true);
        assert_eq!(validate_byr("2002"), true);
        assert_eq!(validate_byr("2003"), false);
    }
}

pub fn validate_iyr(iyr: &str) -> bool {
    match iyr.parse::<i32>() {
        Ok(x) => x >= 2010 && x <= 2020,
        Err(_) => false,
    }
}

#[cfg(test)]
mod validate_iyr {
    use super::validate_iyr;

    #[test]
    fn it_works() {
        assert_eq!(validate_iyr("abc"), false);
        assert_eq!(validate_iyr("2009"), false);
        assert_eq!(validate_iyr("2010"), true);
        assert_eq!(validate_iyr("2020"), true);
        assert_eq!(validate_iyr("2021"), false);
    }
}

pub fn validate_eyr(eyr: &str) -> bool {
    match eyr.parse::<i32>() {
        Ok(value) => value >= 2020 && value <= 2030,
        Err(_) => false,
    }
}

#[cfg(test)]
mod validate_eyr {
    use super::validate_eyr;

    #[test]
    fn it_works() {
        assert_eq!(validate_eyr("abc"), false);
        assert_eq!(validate_eyr("2019"), false);
        assert_eq!(validate_eyr("2020"), true);
        assert_eq!(validate_eyr("2030"), true);
        assert_eq!(validate_eyr("2031"), false);
    }
}

pub fn validate_hgt(hgt: &str) -> bool {
    let captures = match HGT_FORMAT.captures(hgt) {
        Some(c) => c,
        None => return false,
    };
    let value = match captures.get(1) {
        Some(v) => v.as_str(),
        None => return false,
    };
    let units = match captures.get(2) {
        Some(u) => u.as_str(),
        None => return false,
    };
    match (units, value.parse::<i32>()) {
        ("in", Ok(x)) => x >= 59 && x <= 76,
        ("cm", Ok(x)) => x >= 150 && x <= 193,
        (_, _) => false,
    }
}

#[cfg(test)]
mod validate_hgt {
    use super::validate_hgt;

    #[test]
    fn it_works() {
        assert_eq!(validate_hgt("abc"), false);

        assert_eq!(validate_hgt("58in"), false);
        assert_eq!(validate_hgt("59in"), true);
        assert_eq!(validate_hgt("76in"), true);
        assert_eq!(validate_hgt("77in"), false);

        assert_eq!(validate_hgt("149cm"), false);
        assert_eq!(validate_hgt("150cm"), true);
        assert_eq!(validate_hgt("193cm"), true);
        assert_eq!(validate_hgt("194cm"), false);
    }
}

pub fn validate_hcl(hcl: &str) -> bool {
    let _first = match hcl.get(0..1) {
        Some(letter) => letter,
        None => return false,
    };

    let value = &hcl[1..].to_string();
    if value.len() != 6 {
        return false;
    }
    return HEX.is_match(value);
}

#[cfg(test)]
mod validate_hcl {
    use super::validate_hcl;

    #[test]
    fn it_works() {
        assert_eq!(validate_hcl("abc"), false);
        assert_eq!(validate_hcl("#zxy123"), false);
        assert_eq!(validate_hcl("#abc123"), true);
        assert_eq!(validate_hcl("#987def"), true);
        assert_eq!(validate_hcl(""), false);
    }
}

pub fn validate_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

#[cfg(test)]
mod validate_ecl {
    use super::validate_ecl;

    #[test]
    fn it_works() {
        assert_eq!(validate_ecl("amb"), true);
        assert_eq!(validate_ecl("blu"), true);
        assert_eq!(validate_ecl("brn"), true);
        assert_eq!(validate_ecl("gry"), true);
        assert_eq!(validate_ecl("grn"), true);
        assert_eq!(validate_ecl("hzl"), true);
        assert_eq!(validate_ecl("oth"), true);
        assert_eq!(validate_ecl(""), false);
    }
}

pub fn validate_pid(pid: &str) -> bool {
    pid.len() == 9 && ALL_NUM.is_match(pid)
}

#[cfg(test)]
mod validate_pid {
    use super::validate_pid;

    #[test]
    fn it_works() {
        assert_eq!(validate_pid(""), false);
        assert_eq!(validate_pid("2"), false);
        assert_eq!(validate_pid("42"), false);
        assert_eq!(validate_pid("242"), false);
        assert_eq!(validate_pid("2402"), false);
        assert_eq!(validate_pid("24952"), false);
        assert_eq!(validate_pid("249052"), false);
        assert_eq!(validate_pid("2492052"), false);
        assert_eq!(validate_pid("27492052"), false);
        assert_eq!(validate_pid("827492052"), true);
    }
}