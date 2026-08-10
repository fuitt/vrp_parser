#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Name(String),
    Comment(String),
    Type(String),
    Dimension(String),
    EdgeWeightType(String),
    EdgeWeightFormat(String),
    NodeCoordType(String),
    Capacity(String),
    EdgeWeightSection,
    NodeCoordSection,
    DemandSection,
    DepotSection,
    Eof,
    Data(Vec<String>),
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TokenError {
    #[error("unrecognized key: {0}")]
    UnknownKey(String),
}
