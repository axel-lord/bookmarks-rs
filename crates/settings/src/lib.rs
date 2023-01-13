//! Crate for handling writing and reading of settings.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::missing_crate_level_docs
)]

use std::{
    any::{self, Any},
    collections::HashMap,
};
use thiserror::Error;

type DefaultConstructor = Box<dyn Fn() -> Box<dyn Any>>;

/// Type to store settings.
pub struct Settings {
    settings: HashMap<String, (Box<dyn Any>, String, DefaultConstructor)>,
}

/// Used to build a [Settings] instance.
pub struct SettingsBuilder {
    settings: Settings,
}

/// Error type used by crate.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Error returned when the wrong type is used getting a setting.
    #[error("Setting \"{setting}\" is of type <{setting_type}>, not <{tried_type}>")]
    WrongSettingType {
        /// The key of the setting.
        setting: String,
        /// The type of the setting.
        setting_type: String,
        /// The type that was wrongly used trying to get setting.
        tried_type: String,
    },
    /// Error returned when trying to get a setting that does not exist.
    #[error("Setting \"{setting}\" does not exist")]
    SettingDoesNotExist {
        /// The setting key that does not exist.
        setting: String,
    },
}

type Result<T> = std::result::Result<T, Error>;

impl Default for SettingsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsBuilder {
    /// Get a new settings builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            settings: Settings {
                settings: HashMap::new(),
            },
        }
    }
    /// Build the [Settings].
    #[must_use]
    pub fn build(self) -> Settings {
        self.settings
    }

    /// Add a setting and set it's default value.
    #[must_use]
    pub fn add<T>(mut self, setting: impl Into<String>, default_value: T) -> Self
    where
        T: 'static + Clone,
    {
        self.settings.settings.insert(
            setting.into(),
            (
                Box::new(default_value.clone()),
                any::type_name::<T>().into(),
                Box::new(move || Box::new(default_value.clone())),
            ),
        );
        self
    }

    /// Add a setting and use it's [Default] implementation for default.
    #[must_use]
    pub fn add_default<T>(mut self, setting: impl Into<String>) -> Self
    where
        T: 'static + Default,
    {
        self.settings.settings.insert(
            setting.into(),
            (
                Box::<T>::default(),
                any::type_name::<T>().into(),
                Box::new(|| Box::<T>::default()),
            ),
        );
        self
    }

    /// Add a setting with a function for setting default value.
    #[must_use]
    pub fn add_fn<T>(
        mut self,
        setting: impl Into<String>,
        default_fn: impl 'static + Fn() -> T,
    ) -> Self
    where
        T: 'static,
    {
        self.settings.settings.insert(
            setting.into(),
            (
                Box::new(default_fn()),
                any::type_name::<T>().into(),
                Box::new(move || Box::new(default_fn())),
            ),
        );
        self
    }
}

impl Settings {
    /// Attempt to read a setting.
    ///
    /// # Errors
    /// If the setting does not exist or the wrong type is used to access it.
    pub fn read<'a, T: 'static>(&'a self, setting: &str) -> Result<&'a T> {
        let value = self
            .settings
            .get(setting)
            .ok_or_else(|| Error::SettingDoesNotExist {
                setting: setting.into(),
            })?;
        value
            .0
            .downcast_ref()
            .ok_or_else(|| Error::WrongSettingType {
                setting: setting.into(),
                setting_type: value.1.clone(),
                tried_type: any::type_name::<T>().into(),
            })
    }

    /// Check if a setting is the same as a value.
    /// # Errors
    /// If the setting and the value have differing types, or if the setting does not exist.
    pub fn check<T: 'static + PartialEq<T>>(&self, setting: &str, other: &T) -> Result<bool> {
        let value = self
            .settings
            .get(setting)
            .ok_or_else(|| Error::SettingDoesNotExist {
                setting: setting.into(),
            })?;
        let value = value
            .0
            .downcast_ref::<T>()
            .ok_or_else(|| Error::WrongSettingType {
                setting: setting.into(),
                setting_type: value.1.clone(),
                tried_type: any::type_name::<T>().into(),
            })?;

        Ok(other.eq(value))
    }

    /// Get the default value of a setting.
    ///
    /// # Errors
    /// If the setting does not exist ore is not of the supplied type.
    pub fn get_default<T: 'static>(&self, setting: &str) -> Result<T> {
        let value = self
            .settings
            .get(setting)
            .ok_or_else(|| Error::SettingDoesNotExist {
                setting: setting.into(),
            })?;
        let value = value.2()
            .downcast::<T>()
            .map_err(|_| Error::WrongSettingType {
                setting: setting.into(),
                setting_type: value.1.clone(),
                tried_type: any::type_name::<T>().into(),
            })?;
        Ok(*value)
    }

    /// Set a setting to it's default value.
    ///
    /// # Errors
    /// If the setting does not exist.
    pub fn reset_setting(&mut self, setting: &str) -> Result<()> {
        let value = self
            .settings
            .get_mut(setting)
            .ok_or_else(|| Error::SettingDoesNotExist {
                setting: setting.into(),
            })?;

        value.0 = value.2();
        Ok(())
    }

    /// Set all settings to their default values.
    pub fn reset_all(&mut self) {
        for value in self.settings.values_mut() {
            value.0 = value.2();
        }
    }

    /// Attempt to write a setting.
    ///
    /// # Errors
    /// If the setting does not exits or the wrong type is used to access it.
    pub fn write<T: 'static>(&mut self, setting: &str, new_value: T) -> Result<()> {
        let mut value =
            self.settings
                .get_mut(setting)
                .ok_or_else(|| Error::SettingDoesNotExist {
                    setting: setting.into(),
                })?;

        if !(value.0.is::<T>()) {
            return Err(Error::WrongSettingType {
                setting: setting.into(),
                setting_type: value.1.clone(),
                tried_type: any::type_name::<T>().into(),
            });
        }

        value.0 = Box::new(new_value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    #[test]
    pub fn build_settings_add() {
        let settings = SettingsBuilder::new()
            .add("yes", true)
            .add("no", false)
            .add("maybe", true)
            .add("hello", String::from("world"))
            .add("not_found", 404u32)
            .build();

        assert!(*settings.settings["yes"]
            .0
            .downcast_ref::<bool>()
            .expect("yes should be a bool"));
        assert!(!*settings.settings["no"]
            .0
            .downcast_ref::<bool>()
            .expect("no should be a bool"));
        assert!(*settings.settings["maybe"]
            .0
            .downcast_ref::<bool>()
            .expect("maybe should be a bool"));
        assert_eq!(
            *settings.settings["hello"]
                .0
                .downcast_ref::<String>()
                .expect("hello should be a string"),
            String::from("world")
        );
        assert_eq!(
            *settings.settings["not_found"]
                .0
                .downcast_ref::<u32>()
                .expect("not_found should be a u32"),
            404u32
        );
    }

    #[test]
    pub fn settings_read() {
        let settings = SettingsBuilder::new()
            .add("yes", true)
            .add("no", false)
            .add("maybe", true)
            .add("hello", String::from("world"))
            .add("not_found", 404u32)
            .build();

        assert!(*settings
            .read::<bool>("yes")
            .expect("yes should exist and be a bool"));
        assert!(!*settings
            .read::<bool>("no")
            .expect("no should exist and be a bool"));
        assert!(*settings
            .read::<bool>("maybe")
            .expect("maybe should exist and be a bool"));
        assert_eq!(
            *settings
                .read::<String>("hello")
                .expect("hello should exist and be a string"),
            String::from("world")
        );
        assert_eq!(
            *settings
                .read::<u32>("not_found")
                .expect("not_found should exist and be a u32"),
            404u32
        );
    }

    #[test]
    pub fn settings_check() {
        let settings = SettingsBuilder::new()
            .add("yes", true)
            .add("no", false)
            .add("maybe", true)
            .add("hello", String::from("world"))
            .add("not_found", 404u32)
            .build();

        assert_eq!(settings.check("yes", &true), Ok(true));
        assert_eq!(settings.check("yes", &false), Ok(false));
        assert_eq!(settings.check("no", &false), Ok(true));
        assert_eq!(settings.check("no", &true), Ok(false));
        assert_eq!(settings.check("maybe", &true), Ok(true));
        assert_eq!(settings.check("maybe", &false), Ok(false));
        assert_eq!(settings.check("hello", &String::from("world")), Ok(true));
        assert_eq!(settings.check("hello", &String::from("rust")), Ok(false));
        assert_eq!(settings.check("not_found", &404u32), Ok(true));

        assert!(settings.check("no", &404).is_err());
        assert!(settings.check("nice", &true).is_err());
    }

    #[test]
    pub fn build_settings_default() {
        let settings = SettingsBuilder::new()
            .add_default::<bool>("bool")
            .add_default::<i32>("i32")
            .add_default::<HashMap<String, Range<usize>>>("HashMap")
            .build();

        assert_eq!(settings.check("bool", &bool::default()), Ok(true));
        assert_eq!(settings.check("i32", &i32::default()), Ok(true));
        assert_eq!(
            settings.check("HashMap", &<HashMap<String, Range<usize>>>::default()),
            Ok(true)
        );
    }

    #[test]
    pub fn build_settings_fn() {
        let settings = SettingsBuilder::new()
            .add_fn("yes", || true)
            .add_fn("no", || false)
            .add_fn("maybe", || true)
            .add_fn("hello", || String::from("world"))
            .add_fn("not_found", || 404u32)
            .build();

        assert_eq!(settings.check("yes", &true), Ok(true));
        assert_eq!(settings.check("no", &false), Ok(true));
        assert_eq!(settings.check("maybe", &true), Ok(true));
        assert_eq!(settings.check("hello", &String::from("world")), Ok(true));
        assert_eq!(settings.check("not_found", &404u32), Ok(true));

        assert!(settings.check("no", &404).is_err());
        assert!(settings.check("nice", &true).is_err());
    }

    #[test]
    pub fn settings_write() -> Result<()> {
        let mut settings = SettingsBuilder::new()
            .add("yes", true)
            .add("no", false)
            .add("maybe", true)
            .add("hello", String::from("world"))
            .add("not_found", 404u32)
            .build();

        settings.write("yes", false)?;
        settings.write("no", true)?;
        settings.write("maybe", false)?;
        settings.write("hello", String::from("rust"))?;
        settings.write("not_found", 101u32)?;

        assert!(settings.write("maybe", 50).is_err());
        assert!(settings
            .write("no", HashMap::<String, String>::new())
            .is_err());

        assert_eq!(settings.check("yes", &false), Ok(true));
        assert_eq!(settings.check("no", &true), Ok(true));
        assert_eq!(settings.check("maybe", &false), Ok(true));
        assert_eq!(settings.check("hello", &String::from("rust")), Ok(true));
        assert_eq!(settings.check("not_found", &101u32), Ok(true));

        Ok(())
    }

    #[test]
    pub fn settings_get_default() -> Result<()> {
        let mut settings = SettingsBuilder::new()
            .add("yes", true)
            .add_default::<bool>("no")
            .add_fn("maybe", || true)
            .add_fn("hello", || String::from("world"))
            .add("not_found", 404u32)
            .build();

        settings.write("yes", false)?;
        settings.write("no", true)?;
        settings.write("maybe", false)?;
        settings.write("hello", String::from("rust"))?;
        settings.write("not_found", 101u32)?;

        assert!(settings.get_default::<bool>("yes")?);
        assert!(!settings.get_default::<bool>("no")?);
        assert!(settings.get_default::<bool>("maybe")?);
        assert_eq!(
            settings.get_default::<String>("hello")?,
            String::from("world")
        );
        assert_eq!(settings.get_default::<u32>("not_found")?, 404u32);
        Ok(())
    }

    #[test]
    pub fn settings_reset_setting() -> Result<()> {
        let mut settings = SettingsBuilder::new()
            .add("yes", true)
            .add("no", false)
            .add("maybe", true)
            .add("hello", String::from("world"))
            .add("not_found", 404u32)
            .build();

        settings.write("yes", false)?;
        settings.write("no", true)?;
        settings.write("maybe", false)?;
        settings.write("hello", String::from("rust"))?;
        settings.write("not_found", 101u32)?;

        // make sure writes worked
        assert_eq!(settings.check("yes", &false), Ok(true));
        assert_eq!(settings.check("no", &true), Ok(true));
        assert_eq!(settings.check("maybe", &false), Ok(true));
        assert_eq!(settings.check("hello", &String::from("rust")), Ok(true));
        assert_eq!(settings.check("not_found", &101u32), Ok(true));

        settings.reset_setting("yes")?;
        settings.reset_setting("no")?;
        settings.reset_setting("maybe")?;

        assert_eq!(settings.check("yes", &true), Ok(true));
        assert_eq!(settings.check("no", &false), Ok(true));
        assert_eq!(settings.check("maybe", &true), Ok(true));
        assert_eq!(settings.check("hello", &String::from("rust")), Ok(true));
        assert_eq!(settings.check("not_found", &101u32), Ok(true));

        settings.reset_setting("hello")?;
        settings.reset_setting("not_found")?;

        assert_eq!(settings.check("hello", &String::from("world")), Ok(true));
        assert_eq!(settings.check("not_found", &404u32), Ok(true));

        Ok(())
    }

    #[test]
    pub fn settings_reset_all() -> Result<()> {
        let mut settings = SettingsBuilder::new()
            .add("yes", true)
            .add("no", false)
            .add("maybe", true)
            .add("hello", String::from("world"))
            .add("not_found", 404u32)
            .build();

        settings.write("yes", false)?;
        settings.write("no", true)?;
        settings.write("maybe", false)?;
        settings.write("hello", String::from("rust"))?;
        settings.write("not_found", 101u32)?;

        // make sure writes worked
        assert!(settings.check("yes", &false)?);
        assert!(settings.check("no", &true)?);
        assert!(settings.check("maybe", &false)?);
        assert!(settings.check("hello", &String::from("rust"))?);
        assert!(settings.check("not_found", &101u32)?);

        settings.reset_all();

        // reset worked
        assert!(settings.check("yes", &true)?);
        assert!(settings.check("no", &false)?);
        assert!(settings.check("maybe", &true)?);
        assert!(settings.check("hello", &String::from("world"))?);
        assert!(settings.check("not_found", &404u32)?);

        Ok(())
    }
}
