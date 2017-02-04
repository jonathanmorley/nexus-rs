#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DataContainer<T> {
    pub data: T,
}
