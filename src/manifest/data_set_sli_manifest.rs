use super::csv_params::CsvParams;
use super::part::Part;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSetSLIManifest {
    #[serde(rename = "dataSet")]
    pub dataset: Option<String>,

    pub file: Option<String>,

    #[serde(rename = "csvParams")]
    pub csv_params: Option<CsvParams>,

    pub parts: Option<Vec<Part>>
}
