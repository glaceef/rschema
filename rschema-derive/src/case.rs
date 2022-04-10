use darling::FromMeta;

#[derive(Clone, Copy, Debug, PartialEq, FromMeta)]
pub enum Case {
    #[darling(rename = "lowercase")]
    Lower,

    #[darling(rename = "UPPERCASE")]
    Upper,

    #[darling(rename = "camelCase")]
    Camel,

    #[darling(rename = "PascalCase")]
    Pascal,

    #[darling(rename = "kebab-case")]
    Kebab,

    #[darling(rename = "Train-Case")]
    Train,

    #[darling(rename = "COBOL-CASE")]
    Cobol,

    #[darling(rename = "snake_case")]
    Snake,

    #[darling(rename = "UPPER_SNAKE_CASE")]
    UpperSnake,

    #[darling(rename = "flatcase")]
    Flat,

    #[darling(rename = "UPPERFLATCASE")]
    UpperFlat,
}

impl Into<convert_case::Case> for Case {
    fn into(self) -> convert_case::Case {
        match self {
            Self::Lower      => convert_case::Case::Lower,
            Self::Upper      => convert_case::Case::Upper,
            Self::Camel      => convert_case::Case::Camel,
            Self::Pascal     => convert_case::Case::Pascal,
            Self::Kebab      => convert_case::Case::Kebab,
            Self::Train      => convert_case::Case::Train,
            Self::Cobol      => convert_case::Case::Cobol,
            Self::Snake      => convert_case::Case::Snake,
            Self::UpperSnake => convert_case::Case::UpperSnake,
            Self::Flat       => convert_case::Case::Flat,
            Self::UpperFlat  => convert_case::Case::UpperFlat,
        }
    }
}
