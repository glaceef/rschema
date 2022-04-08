use darling::FromMeta;

#[derive(Copy, Clone, Debug, FromMeta)]
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
            Case::Lower      => convert_case::Case::Lower,
            Case::Upper      => convert_case::Case::Upper,
            Case::Camel      => convert_case::Case::Camel,
            Case::Pascal     => convert_case::Case::Pascal,
            Case::Kebab      => convert_case::Case::Kebab,
            Case::Train      => convert_case::Case::Train,
            Case::Cobol      => convert_case::Case::Cobol,
            Case::Snake      => convert_case::Case::Snake,
            Case::UpperSnake => convert_case::Case::UpperSnake,
            Case::Flat       => convert_case::Case::Flat,
            Case::UpperFlat  => convert_case::Case::UpperFlat,
        }
    }
}
