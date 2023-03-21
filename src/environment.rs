use std::{
    env::{self, VarError},
    error::Error,
    fmt::Display,
};

use reqwest::Url;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};
use url::ParseError;

#[allow(non_camel_case_types)]
#[derive(IntoStaticStr, EnumIter)]
enum EnvironmentVariable {
    KEYCLOAK_BASE_URL,
    KEYCLOAK_REALM,
}

#[derive(Debug)]
pub enum EnvironmentValidationError {
    VarError(VarError),
    ParseError(ParseError),
}

impl Display for EnvironmentValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for EnvironmentValidationError {} // 3

pub fn validate_environment() -> Result<(), EnvironmentValidationError> {
    for environment_variable in EnvironmentVariable::iter() {
        match environment_variable {
            EnvironmentVariable::KEYCLOAK_BASE_URL => {
                Url::parse(
                    env::var(<&'static str>::from(EnvironmentVariable::KEYCLOAK_BASE_URL))
                        .map_err(EnvironmentValidationError::VarError)?
                        .as_str(),
                )
                .map_err(EnvironmentValidationError::ParseError)?;
            }
            x => {
                env::var(<&'static str>::from(x)).map_err(EnvironmentValidationError::VarError)?;
            }
        }
    }
    Ok(())
}
