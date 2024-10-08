use std::str::FromStr;

/// Struct representing an RGB color
pub(crate) struct Rgb(pub(crate) u32, pub(crate) u32, pub(crate) u32);

pub struct ParseError;

impl FromStr for Rgb {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rgb = s
            .split(',')
            .map(|s| s.parse::<u32>().unwrap_or(255))
            .try_fold(vec![], |mut acc, x| {
                if acc.len() < 3 {
                    acc.push(x);
                    Ok(acc)
                } else {
                    Err(ParseError)
                }
            })?;
        Ok(Rgb(rgb[0], rgb[1], rgb[2]))
    }
}
