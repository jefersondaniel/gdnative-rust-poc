use crate::core::error::DataError;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn parse(values: &[String]) -> Result<Self, DataError> {
        let error = DataError::new(format!("Invalid color: {:?}", values));

        if values.len() != 3 {
            return Err(error);
        }

        let r = values[0].parse::<u8>().map_err(|_| error.clone())?;
        let g = values[1].parse::<u8>().map_err(|_| error.clone())?;
        let b = values[2].parse::<u8>().map_err(|_| error.clone())?;

        Ok(Color { r, g, b })
    }
}
