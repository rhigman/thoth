use anyhow::Result;
use stdweb::{js, unstable::TryInto};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CookieError {
    #[error("no cookie found for the given name")]
    NotFound,
}

pub struct CookieService;

impl CookieService {
    pub fn new() -> Self {
        CookieService
    }

    // default expiry 1 day
    pub fn set(&self, name: &str, value: &str) {
        self.set_expiring(name, value, 1)
    }

    pub fn get(&self, name: &str) -> Result<String> {
        let cookie_strings = js! { return document.cookie.split(';') };
        let cookies: Vec<String> = cookie_strings.try_into()?;
        cookies
            .iter()
            .filter_map(|x| {
                let name_value: Vec<_> = x.splitn(2, '=').collect();
                match name_value.get(0) {
                    None => None,
                    Some(c) => {
                        if c.trim_start() == name {
                            name_value.get(1).map(|x| (*x).to_owned())
                        } else {
                            None
                        }
                    }
                }
            })
            .collect::<Vec<String>>()
            .pop()
            .ok_or_else(|| CookieError::NotFound.into())
    }

    pub fn remove(&self, name: &str) {
        self.set_expiring(name, "", -1);
    }

    fn set_expiring(&self, name: &str, value: &str, days: i32) {
        js! {
            document.cookie = @{name} + "=" + (@{value} || "") +
                ";max-age=" + (@{days} * 24 * 60 * 60) + ";path=/";
        }
    }
}
