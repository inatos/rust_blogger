use serde::{Deserialize, Deserializer, de::{self, DeserializeOwned}};


#[derive(Deserialize)]
#[serde(untagged)]
enum DataOrString<T> {
    Data(T),
    Empty {},
    String(String),
}

/**
 * Default nullable value to either Ok or None. 
 */
pub fn ok_or_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    match DataOrString::deserialize(deserializer)? {
        DataOrString::Data(data) => Ok(data),
        DataOrString::Empty {} => Ok(None),
        DataOrString::String(data_string) => 
            if data_string.is_empty() 
            {
                Ok(None)
            }
            else 
            {
                serde_json::from_str::<Option<T>>(&data_string).map_err(de::Error::custom)  
            }
    }
}