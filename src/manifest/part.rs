use std::collections::BTreeMap;

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
