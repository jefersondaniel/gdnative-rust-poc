use crate::core::{attribute_value::{AttributeValue, ParseAttributeValue}, enumerations::PrintJustification, error::DataError};

use super::color::Color;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrintData {
    pub index: i16,
    pub colorindex: i16,
    pub justification: PrintJustification,
    pub color: Option<Color>,
}

impl ParseAttributeValue for PrintData {
    fn parse_attribute_value(value: AttributeValue) -> Result<PrintData, DataError> {
        let values = value.split_values();

        if values.len() >= 2 {
            let index = values[0].parse::<i16>()
                .map_err(|_| DataError::new(format!("Invalid index: {}", values[0])))?;
            let colorindex = values[0].parse::<i16>()
                .map_err(|_| DataError::new(format!("Invalid index: {}", values[1])))?;
            let mut justification = PrintJustification::Center;
            let mut color = None;

            if values.len() >= 3 {
                justification = values[2].parse::<i16>()
                    .map_err(|_| DataError::new(format!("Invalid font justification: {}", values[3])))?
                    .into();
            }

            if values.len() >= 6 {
                color = Some(Color::parse(&values[3..6])?);
            }

            return Ok(PrintData {
                index,
                colorindex,
                justification,
                color
            });
        }

        Err(DataError::new(format!("Invalid font format: \"{}\"", value.to_string())))
    }
}
