use crate::settings::load_settings;
use crate::errors::FeoError;

pub fn fetch() -> Result<(), FeoError> {
    let settings = load_settings()?;


    todo!()
}