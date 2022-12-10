use proc_macro2::TokenStream;

use crate::{
    base::{Kind, SpannedItem},
    models::{Guard, RawGuard},
};

// Sanitizer
//

#[derive(Debug)]
pub enum FloatSanitizer<T> {
    With(TokenStream),
    _Phantom(std::marker::PhantomData<T>),
}

pub type SpannedFloatSanitizer<T> = SpannedItem<FloatSanitizer<T>>;

#[derive(Debug, PartialEq, Eq)]
pub enum FloatSanitizerKind {
    With,
}

impl std::fmt::Display for FloatSanitizerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::With => write!(f, "with"),
        }
    }
}

impl<T> Kind for FloatSanitizer<T> {
    type Kind = FloatSanitizerKind;

    fn kind(&self) -> FloatSanitizerKind {
        match self {
            Self::With(_) => FloatSanitizerKind::With,
            Self::_Phantom(_) => {
                unreachable!("Kind::kind(): FloatSanitizer::_Phantom must not be used")
            }
        }
    }
}

// Validator
//

#[derive(Debug)]
pub enum FloatValidator<T> {
    Min(T),
    Max(T),
    With(TokenStream),
}

pub type SpannedFloatValidator<T> = SpannedItem<FloatValidator<T>>;

#[derive(Debug, PartialEq, Eq)]
pub enum FloatValidatorKind {
    Min,
    Max,
    With,
}

impl std::fmt::Display for FloatValidatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::With => write!(f, "with"),
        }
    }
}

impl<T> Kind for FloatValidator<T> {
    type Kind = FloatValidatorKind;

    fn kind(&self) -> FloatValidatorKind {
        match self {
            Self::Min(_) => FloatValidatorKind::Min,
            Self::Max(_) => FloatValidatorKind::Max,
            Self::With(_) => FloatValidatorKind::With,
        }
    }
}

// Traits
//
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum FloatDeriveTrait {
    // Standard
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    FromStr,
    AsRef,
    Into,
    From,
    TryFrom,
    Borrow,
    Display,
    // // External crates
    //
    // Serialize,
    // Deserialize,
    // Arbitrary,
}

pub type FloatRawGuard<T> = RawGuard<SpannedFloatSanitizer<T>, SpannedFloatValidator<T>>;
pub type FloatGuard<T> = Guard<FloatSanitizer<T>, FloatValidator<T>>;
