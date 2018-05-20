use std::io;
use std::str::FromStr;

// Safe case-insensitive 'readLn'
pub fn readln<T: FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid UTF-8");
    Ok(input.to_titlecase().trim().parse()?)
}

// Extend String
trait TitleCase {
    type Owned;
    fn to_titlecase(&self) -> Self::Owned;
}

impl TitleCase for String {
    type Owned = String;

    fn to_titlecase(&self) -> String {
        let mut c = self.chars();

        match c.next() {
            None => String::new(),
            Some(f) => {
                f.to_uppercase().collect::<String>() + c.collect::<String>().to_lowercase().as_str()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_titlecase() {
        let s = String::from("teSt");
        assert_eq!("Test", s.to_titlecase());
    }
}
