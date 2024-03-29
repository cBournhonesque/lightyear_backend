/*
 * Codema
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: info@edgegap.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiModelDeploymentfilter {
    /// Auto Generated Field for field
    #[serde(rename = "field")]
    pub field: Field,
    /// Auto Generated Field for values
    #[serde(rename = "values")]
    pub values: Vec<String>,
    /// Auto Generated Field for filter_type
    #[serde(rename = "filter_type")]
    pub filter_type: FilterType,
}

impl ApiModelDeploymentfilter {
    pub fn new(field: Field, values: Vec<String>, filter_type: FilterType) -> ApiModelDeploymentfilter {
        ApiModelDeploymentfilter {
            field,
            values,
            filter_type,
        }
    }
}

/// Auto Generated Field for field
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Field {
    #[serde(rename = "city")]
    City,
    #[serde(rename = "country")]
    Country,
    #[serde(rename = "continent")]
    Continent,
    #[serde(rename = "region")]
    Region,
    #[serde(rename = "administrative_division")]
    AdministrativeDivision,
    #[serde(rename = "location_tags")]
    LocationTags,
}
/// Auto Generated Field for filter_type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FilterType {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "not")]
    Not,
}

