#[macro_export]
macro_rules! string_enum {
    (pub enum $name:ident {
        $($variant:ident),* $(,)?
    }) => {
				use std::str::FromStr;
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ormlite::types::ManualType,
            serde::Serialize,
            serde::Deserialize,
        )]
        pub enum $name {
            $($variant),*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, "{}", stringify!($variant).to_lowercase())),*
                }
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($variant) => Ok(Self::$variant)),*,
                    _ => Err(format!("Invalid {} value: {}", stringify!($name), s))
                }
            }
        }

        impl std::convert::TryFrom<&str> for $name {
            type Error = String;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::from_str(value)
            }
        }

				impl sqlx::Encode<'_, sqlx::Postgres> for $name {
						fn encode_by_ref(
								&self,
								buf: &mut sqlx::postgres::PgArgumentBuffer
						) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
								let s = self.to_string();
								<String as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
						}
				}

        impl sqlx::Decode<'_, sqlx::Postgres> for $name {
            fn decode(
                value: sqlx::postgres::PgValueRef<'_>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                let s = value.as_str()?;
                Self::from_str(s).map_err(|e| sqlx::error::BoxDynError::from(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
            }
        }

        impl sqlx::Type<sqlx::Postgres> for $name {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                <String as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }
    };
}
