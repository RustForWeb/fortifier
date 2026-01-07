#[cfg(all(feature = "serde", feature = "utoipa"))]
/// Implement an error code.
#[macro_export]
macro_rules! error_code {
    ($name:ident, $code_name:ident, $code:literal) => {
        $crate::error_code_base!($name, $code_name, $code);
        $crate::error_code_serde!($name, $code_name, $code);
        $crate::error_code_utoipa!($name, $code_name, $code);
    };
}

#[cfg(all(feature = "serde", not(feature = "utoipa")))]
/// Implement an error code.
#[macro_export]
macro_rules! error_code {
    ($name:ident, $code_name:ident, $code:literal) => {
        $crate::error_code_base!($name, $code_name, $code);
        $crate::error_code_serde!($name, $code_name, $code);
    };
}

#[cfg(all(not(feature = "serde"), feature = "utoipa"))]
/// Implement an error code.
#[macro_export]
macro_rules! error_code {
    ($name:ident, $code_name:ident, $code:literal) => {
        $crate::error_code_base!($name, $code_name, $code);
        $crate::error_code_utoipa!($name, $code_name, $code);
    };
}

#[cfg(all(not(feature = "serde"), not(feature = "utoipa")))]
/// Implement an error code.
#[macro_export]
macro_rules! error_code {
    ($name:ident, $code_name:ident, $code:literal) => {
        $crate::error_code_base!($name, $code_name, $code);
    };
}

/// Implement an error code.
#[macro_export]
macro_rules! error_code_base {
    ($name:ident, $code_name:ident, $code:literal) => {
        const $code_name: &str = $code;

        /// Email address error code.
        #[derive(Eq, PartialEq)]
        pub struct $name;

        impl Default for $name {
            fn default() -> Self {
                Self
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                $code_name
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&**self, f)
            }
        }
    };
}

/// Implement [`serde`] traits for an error code.
#[cfg(feature = "serde")]
#[macro_export]
macro_rules! error_code_serde {
    ($name:ident, $code_name:ident, $code:literal) => {
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                deserializer
                    .deserialize_any($crate::serde::MustBeStrVisitor($code_name))
                    .map(|()| Self)
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str($code_name)
            }
        }
    };
}

/// Implement [`utoipa`] traits for an error code.
#[cfg(feature = "utoipa")]
#[macro_export]
macro_rules! error_code_utoipa {
    ($name:ident, $code_name:ident, $code:literal) => {
        impl ::utoipa::PartialSchema for $name {
            fn schema() -> ::utoipa::openapi::RefOr<::utoipa::openapi::schema::Schema> {
                ::utoipa::openapi::schema::ObjectBuilder::new()
                    .schema_type(::utoipa::openapi::schema::Type::String)
                    .enum_values(Some([$code_name]))
                    .build()
                    .into()
            }
        }

        impl ::utoipa::ToSchema for $name {}
    };
}
