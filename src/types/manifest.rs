use std::collections::BTreeMap;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    #[serde(rename = "columnName")]
    pub column_name: Option<String>,

    pub mode: Option<String>,

    pub populates: Option<Vec<String>>,

    pub constraints: Option<BTreeMap<String, String>>,

    #[serde(rename = "referenceKey")]
    pub reference_key: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSetSLIManifest {
    #[serde(rename = "dataSet")]
    pub dataset: Option<String>,

    pub file: Option<String>,

    #[serde(rename = "csvParams")]
    pub csv_params: Option<CsvParams>,

    pub parts: Option<Vec<Part>>

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "dataSetSLIManifest")]
    pub manifest: Option<DataSetSLIManifest>
}
