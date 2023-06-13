use std::borrow::Cow;

use crate::StyleDeclaration;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub enum PercentInterval {
    #[default]
    Zero,
    Five,
    Ten,
    Fifteen,
    Twenty,
    TwentyFive,
    Thirty,
    ThirtyFive,
    Forty,
    FortyFive,
    Fifty,
    FiftyFive,
    Sixty,
    SixtyFive,
    Seventy,
    SeventyFive,
    Eighty,
    EightyFive,
    Ninety,
    NinetyFive,
    Max,
}

impl StyleDeclaration for PercentInterval {
    fn to_css(&self) -> Cow<str> {
        let value = match self {
            Self::Zero => "gap: 0vw;",
            Self::Five => "gap: 5vw;",
            Self::Ten => "gap: 10vw;",
            Self::Fifteen => "gap: 15vw;",
            Self::Twenty => "gap: 20vw;",
            Self::TwentyFive => "gap: 25vw;",
            Self::Thirty => "gap: 30vw;",
            Self::ThirtyFive => "gap: 35vw;",
            Self::Forty => "gap: 40vw;",
            Self::FortyFive => "gap: 45vw;",
            Self::Fifty => "gap: 50vw;",
            Self::FiftyFive => "gap: 55vw;",
            Self::Sixty => "gap: 60vw;",
            Self::SixtyFive => "gap: 65vw;",
            Self::Seventy => "gap: 70vw;",
            Self::SeventyFive => "gap: 75vw;",
            Self::Eighty => "gap: 80vw;",
            Self::EightyFive => "gap: 85vw;",
            Self::Ninety => "gap: 90vw;",
            Self::NinetyFive => "gap: 95vw;",
            Self::Max => "gap: 100vw;",
        };

        Cow::Borrowed(value)
    }
}
