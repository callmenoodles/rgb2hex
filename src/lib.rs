/// Convert RGB to Hex
/// Example: `rgb2hex`
pub fn rgb2hex(r: u8, g: u8, b: u8) -> Result<u32, String> {
    let colors = [r, g, b];
    let mut result = String::new();

    for color in colors.iter() {
        result.push_str(format!("{:02x?}", color).as_str());
    }

    Ok(match u32::from_str_radix(result.as_str(), 16) {
        Ok(v) => v,
        Err(e) => Err(e.to_string())?,
    })
}

/// Convert RGB (str) to Hex
/// Example: `rgb2hex_from_str("251 ,169,12")`
pub fn rgb2hex_from_str(value: &str) -> Result<u32, String> {
    let color = value
        .split(",")
        .map(|x| x.replace(" ", ""))
        .collect::<Vec<_>>();

    if color.len() != 3 {
        Err(format!("Invalid color format: {}", value))
    } else {
        let to_color = |v: &str| -> Result<u8, String> {
            match v.parse::<u8>() {
                Ok(v) => Ok(v),
                Err(e) => Err(e.to_string())?,
            }
        };

        return rgb2hex(
            to_color(color[0].as_str())?,
            to_color(color[1].as_str())?,
            to_color(color[2].as_str())?,
        );
    }
}

#[cfg(test)]
mod test {
    use crate::{rgb2hex, rgb2hex_from_str};

    #[test]
    fn test() {
        assert_eq!(rgb2hex(251, 169, 12), Ok(0xfba90c));
        assert_eq!(rgb2hex_from_str("251, 169, 12"), Ok(0xfba90c));
        assert_eq!(rgb2hex_from_str("251 ,169,12 "), Ok(0xfba90c));
    }
}
