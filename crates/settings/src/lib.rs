//! Crate for handling writingfield1afield1d rfield1adifield1g of settings.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::missing_crate_level_docs
)]

use std::{
    any::{self, Any},
    borrow::Borrow,
    collections::HashMap,
    fmt::{self, Debug, Display},
    marker::PhantomData,
    ops::{Deref, Index, IndexMut},
};

use thiserror::Error;

type SettingValue = Box<dyn Any>;
type DebugFn = Box<dyn Fn(&SettingValue, &mut fmt::Formatter<'_>) -> fmt::Result>;

/// Result type used by settings.
pub type Result<T> = std::result::Result<T, Error>;

struct DefaultConstructor(Box<dyn Fn() -> SettingValue>);

impl DefaultConstructor {
    fn from_fn<T>(f: impl 'static + Fn() -> T) -> Self
    where
        T: 'static,
    {
        Self(Box::new(move || Box::new(f())))
    }

    fn construct(&self) -> SettingValue {
        (self.0)()
    }
}

struct SettingProperties {
    value: SettingValue,
    type_name: String,
    default_constructor: DefaultConstructor,
    debug_fn: DebugFn,
}

/// Type to store settings.
pub struct Settings {
    settings: HashMap<String, SettingProperties>,
}

/// Used to build a [Settings] instance.
#[derive(Debug, Default)]
pub struct SettingsBuilder {
    settings: Vec<(String, SettingProperties)>,
}

/// A key into a settings.
#[derive(Clone, Copy)]
pub struct Key<T>(&'static str, PhantomData<*const T>);

impl<T> Key<T> {
    /// Aquire a key from a static string slice.
    #[must_use]
    pub const fn new(key: &'static str) -> Self {
        Self(key, PhantomData)
    }

    /// Get key as a string slice.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl<T> Debug for Key<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.0, any::type_name::<T>())
    }
}

impl<T> Display for Key<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> AsRef<str> for Key<T> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<T> Deref for Key<T> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
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

fn get_debug_fn<T>() -> DebugFn
where
    T: 'static + Debug,
{
    Box::new(|boxed_value: &SettingValue, f| {
        boxed_value
            .downcast_ref::<T>()
            .expect("debug function should always match type")
            .fmt(f)
    })
}

impl SettingsBuilder {
    /// Get a new settings builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Build the [Settings].
    #[must_use]
    pub fn build(self) -> Settings {
        Settings {
            settings: self.settings.into_iter().collect(),
        }
    }

    #[must_use]
    fn add_setting_properties(
        mut self,
        setting: String,
        setting_properties: SettingProperties,
    ) -> Self {
        self.settings.push((setting, setting_properties));
        self
    }

    /// Add a setting and set it's default value.
    #[must_use]
    pub fn add<T>(self, key: impl Borrow<Key<T>>, default_value: T) -> Self
    where
        T: 'static + Clone + Debug,
    {
        self.add_fn(key, move || default_value.clone())
    }

    /// Add a setting and use it's [Default] implementation for default.
    #[must_use]
    pub fn add_default<T>(self, key: impl Borrow<Key<T>>) -> Self
    where
        T: 'static + Default + Debug,
    {
        self.add_fn(key, T::default)
    }

    /// Add a setting with a function for setting default value.
    #[must_use]
    pub fn add_fn<T, F>(self, key: impl Borrow<Key<T>>, default_constructor: F) -> Self
    where
        F: 'static + Fn() -> T,
        T: 'static + Debug,
    {
        // Box::new(move || Box::new(f()))
        self.str_add_fn::<T>(
            key.borrow().as_str().into(),
            DefaultConstructor::from_fn(default_constructor),
        )
    }

    #[must_use]
    fn str_add_fn<T>(self, setting: String, default_fn: DefaultConstructor) -> Self
    where
        T: 'static + Debug,
    {
        self.add_setting_properties(
            setting,
            SettingProperties {
                value: default_fn.construct(),
                type_name: any::type_name::<T>().into(),
                default_constructor: default_fn,
                debug_fn: get_debug_fn::<T>(),
            },
        )
    }
}

impl Settings {
    /// Attempt to read a setting.
    ///
    /// # Errors
    /// If the setting does not exist or the wrong type is used to access it.
    fn read<'a, T>(&'a self, setting: &str) -> Result<&'a T>
    where
        T: 'static,
    {
        let value = self
            .settings
            .get(setting)
            .ok_or_else(|| Error::SettingDoesNotExist {
                setting: setting.into(),
            })?;
        value
            .value
            .downcast_ref()
            .ok_or_else(|| Error::WrongSettingType {
                setting: setting.into(),
                setting_type: value.type_name.clone(),
                tried_type: any::type_name::<T>().into(),
            })
    }

    /// Get a setting with a key.
    ///
    /// # Errors
    /// If the key is not the key to a setting.
    pub fn get<T>(&self, key: impl Borrow<Key<T>>) -> Result<&T>
    where
        T: 'static,
    {
        self.read::<T>(key.borrow())
    }

    /// Get a mutable setting with a key.
    ///
    /// # Errors
    /// If the key is not the key to a setting.
    pub fn get_mut<T>(&mut self, key: impl Borrow<Key<T>>) -> Result<&mut T>
    where
        T: 'static,
    {
        self.write::<T>(key.borrow())
    }

    /// Check if a setting is the same as a value.
    /// # Errors
    /// If the setting and the value have differing types, or if the setting does not exist.
    pub fn check<T>(&self, setting: &str, other: &T) -> Result<bool>
    where
        T: 'static + Debug + PartialEq<T>,
    {
        let value = self
            .settings
            .get(setting)
            .ok_or_else(|| Error::SettingDoesNotExist {
                setting: setting.into(),
            })?;
        let value = value
            .value
            .downcast_ref::<T>()
            .ok_or_else(|| Error::WrongSettingType {
                setting: setting.into(),
                setting_type: value.type_name.clone(),
                tried_type: any::type_name::<T>().into(),
            })?;

        Ok(other.eq(value))
    }

    /// Get the default value of a setting.
    ///
    /// # Errors
    /// If the setting does not exist ore is not of the supplied type.
    pub fn get_default<T>(&self, setting: &str) -> Result<T>
    where
        T: 'static + Debug,
    {
        let value = self
            .settings
            .get(setting)
            .ok_or_else(|| Error::SettingDoesNotExist {
                setting: setting.into(),
            })?;
        let value = value
            .default_constructor
            .construct()
            .downcast::<T>()
            .map_err(|_| Error::WrongSettingType {
                setting: setting.into(),
                setting_type: value.type_name.clone(),
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

        value.value = value.default_constructor.construct();
        Ok(())
    }

    /// Set all settings to their default values.
    pub fn reset_all(&mut self) {
        for value in self.settings.values_mut() {
            value.value = value.default_constructor.construct();
        }
    }

    /// Attempt to write a setting.
    ///
    /// # Errors
    /// If the setting does not exits or the wrong type is used to access it.
    fn write<'a, T>(&'a mut self, setting: &str) -> Result<&'a mut T>
    where
        T: 'static,
    {
        let value = self
            .settings
            .get_mut(setting)
            .ok_or_else(|| Error::SettingDoesNotExist {
                setting: setting.into(),
            })?;
        value
            .value
            .downcast_mut()
            .ok_or_else(|| Error::WrongSettingType {
                setting: setting.into(),
                setting_type: value.type_name.clone(),
                tried_type: any::type_name::<T>().into(),
            })
    }
}

impl Debug for SettingProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (self.debug_fn)(&self.value, f)
    }
}

impl Debug for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.settings.iter()).finish()
    }
}

impl<T> Index<Key<T>> for Settings
where
    T: 'static,
{
    type Output = T;
    fn index(&self, index: Key<T>) -> &Self::Output {
        self.get(index).expect("key should exist in settings")
    }
}

impl<T> IndexMut<Key<T>> for Settings
where
    T: 'static,
{
    fn index_mut(&mut self, index: Key<T>) -> &mut Self::Output {
        self.get_mut(index).expect("key should exist in settings")
    }
}

/// Macro used to easily define keys.
#[macro_export]
macro_rules! define_keys {
    ($mod:ident: {$($key:ident: $ty:ty),* $(,)?} $(,)?) => {
        paste::paste! {
        mod $mod {
            $(
            pub const [<$key:upper>]: $crate::Key<$ty> = $crate::Key::new(stringify!([<$key:lower>]));
            )*
        }
        }
    };
    (pub $mod:ident: {$($key:ident: $ty:ty),* $(,)?} $(,)?) => {
        paste::paste! {
        pub mod $mod {
            $(
            pub const [<$key:upper>]: $crate::Key<$ty> = $crate::Key::new(stringify!([<$key:lower>]));
            )*
        }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    define_keys! {
        keys: {
            yes: bool,
            no: bool,
            maybe: bool,
            hello: String,
            not_found: u32,
            bool: bool,
            i32: i32,
            hash_map: std::collections::HashMap<String, std::ops::Range<usize>>,
        }
    }

    #[test]
    pub fn build_settings_add() {
        let settings = SettingsBuilder::new()
            .add(keys::YES, true)
            .add(keys::NO, false)
            .add(keys::MAYBE, true)
            .add(keys::HELLO, "world".into())
            .add(keys::NOT_FOUND, 404)
            .build();

        assert!(*settings.settings["yes"]
            .value
            .downcast_ref::<bool>()
            .expect("yes should be a bool"));
        assert!(!*settings.settings["no"]
            .value
            .downcast_ref::<bool>()
            .expect("no should be a bool"));
        assert!(*settings.settings["maybe"]
            .value
            .downcast_ref::<bool>()
            .expect("maybe should be a bool"));
        assert_eq!(
            *settings.settings["hello"]
                .value
                .downcast_ref::<String>()
                .expect("hello should be a string"),
            String::from("world")
        );
        assert_eq!(
            *settings.settings["not_found"]
                .value
                .downcast_ref::<u32>()
                .expect("not_found should be a u32"),
            404u32
        );
    }

    #[test]
    pub fn settings_read() {
        let settings = SettingsBuilder::new()
            .add(keys::YES, true)
            .add(keys::NO, false)
            .add(keys::MAYBE, true)
            .add(keys::HELLO, "world".into())
            .add(keys::NOT_FOUND, 404)
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
            .add(keys::YES, true)
            .add(keys::NO, false)
            .add(keys::MAYBE, true)
            .add(keys::HELLO, "world".into())
            .add(keys::NOT_FOUND, 404)
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
            .add_default(keys::BOOL)
            .add_default(keys::I32)
            .add_default(keys::HASH_MAP)
            .build();

        assert_eq!(settings.check("bool", &bool::default()), Ok(true));
        assert_eq!(settings.check("i32", &i32::default()), Ok(true));
        assert_eq!(
            settings.check("hash_map", &<HashMap<String, Range<usize>>>::default()),
            Ok(true)
        );
    }

    #[test]
    pub fn build_settings_fn() {
        let settings = SettingsBuilder::new()
            .str_add_fn::<bool>("yes".into(), DefaultConstructor::from_fn(|| true))
            .str_add_fn::<bool>("no".into(), DefaultConstructor::from_fn(|| false))
            .str_add_fn::<bool>("maybe".into(), DefaultConstructor::from_fn(|| true))
            .str_add_fn::<String>(
                "hello".into(),
                DefaultConstructor::from_fn(|| String::from("world")),
            )
            .str_add_fn::<u32>("not_found".into(), DefaultConstructor::from_fn(|| 404u32))
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
            .add(keys::YES, true)
            .add(keys::NO, false)
            .add(keys::MAYBE, true)
            .add(keys::HELLO, "world".into())
            .add(keys::NOT_FOUND, 404)
            .build();

        *settings.write("yes")? = false;
        *settings.write("no")? = true;
        *settings.write("maybe")? = false;
        *settings.write("hello")? = String::from("rust");
        *settings.write("not_found")? = 101u32;

        assert!(settings.write::<i32>("maybe").is_err());
        assert!(settings.write::<HashMap::<String, String>>("no").is_err());

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
            .add(keys::YES, true)
            .add_default(keys::NO)
            .add_fn(keys::MAYBE, || true)
            .add_fn(keys::HELLO, || String::from("world"))
            .add(keys::NOT_FOUND, 404u32)
            .build();

        *settings.write("yes")? = false;
        *settings.write("no")? = true;
        *settings.write("maybe")? = false;
        *settings.write("hello")? = String::from("rust");
        *settings.write("not_found")? = 101u32;

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
            .add(keys::YES, true)
            .add(keys::NO, false)
            .add(keys::MAYBE, true)
            .add(keys::HELLO, "world".into())
            .add(keys::NOT_FOUND, 404)
            .build();

        *settings.write("yes")? = false;
        *settings.write("no")? = true;
        *settings.write("maybe")? = false;
        *settings.write("hello")? = String::from("rust");
        *settings.write("not_found")? = 101u32;

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
            .add(keys::YES, true)
            .add(keys::NO, false)
            .add(keys::MAYBE, true)
            .add(keys::HELLO, "world".into())
            .add(keys::NOT_FOUND, 404)
            .build();

        *settings.write("yes")? = false;
        *settings.write("no")? = true;
        *settings.write("maybe")? = false;
        *settings.write("hello")? = String::from("rust");
        *settings.write("not_found")? = 101u32;

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
