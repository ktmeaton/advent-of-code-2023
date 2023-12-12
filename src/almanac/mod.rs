use color_eyre::eyre::{eyre, ContextCompat, Report, Result};
use color_eyre::Help;
use serde::Serialize;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, Serialize)]
pub enum Component {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for Component {
    type Err = Report;

    fn from_str(component: &str) -> Result<Self, Report> {
        let components: Vec<_> =
            Component::iter().map(|c| format!("{c:?}").to_lowercase()).collect();

        let component = Component::iter()
            .filter_map(|c| {
                let s = format!("{c:?}").to_lowercase();
                (component == s).then_some(c)
            })
            .next()
            .wrap_err(eyre!("Unknown Almanac component: {component:?}"))
            .suggestion(format!("Implemented Almanac components: {components:?}"))?;

        Ok(component)
    }
}
