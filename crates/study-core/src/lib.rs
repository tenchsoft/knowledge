macro_rules! study_id_type {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                let value = value.into();
                tench_storage_core::validate_safe_id(&value)
                    .expect("study id must be a safe storage identifier");
                Self(value)
            }

            pub fn parse(value: impl AsRef<str>) -> Result<Self, tench_storage_core::SafeIdError> {
                tench_storage_core::validate_safe_id(value.as_ref())?;
                Ok(Self(value.as_ref().to_string()))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = <String as serde::Deserialize>::deserialize(deserializer)?;
                tench_storage_core::validate_safe_id(&value).map_err(serde::de::Error::custom)?;
                Ok(Self(value))
            }
        }
    };
}

pub(crate) use study_id_type;

pub mod ai;
pub mod authoring;
pub mod builtin;
pub mod curriculum;
pub mod exchange;
pub mod glossary;
pub mod indexing;
pub mod learning;
pub mod locale;
pub mod notes;
pub mod pack;
pub mod programming;
pub mod release;
pub mod review;
pub mod storage;
pub mod subjects;
pub mod visual;

pub use ai::*;
pub use authoring::*;
pub use builtin::*;
pub use curriculum::*;
pub use exchange::*;
pub use glossary::*;
pub use indexing::*;
pub use learning::*;
pub use locale::*;
pub use notes::*;
pub use pack::*;
pub use programming::*;
pub use release::*;
pub use review::*;
pub use storage::*;
pub use subjects::*;
pub use visual::*;
