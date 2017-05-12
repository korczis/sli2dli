#[derive(Debug, Serialize, Deserialize)]
pub struct CsvParams {
    #[serde(rename = "quoteChar")]
    pub quote_char: Option<String>,

    #[serde(rename = "escapeChar")]
    pub escape_char: Option<String>,

    #[serde(rename = "separatorChar")]
    pub separator_char: Option<String>,

    #[serde(rename = "endOfLine")]
    pub end_of_line: Option<String>,
}
