use super::*;


/// Trait that enables type to be converted into a dynamodb Item
pub trait Item {
    /// convert the given type into a key value pair
    fn item(&self) -> HashMap<String, AttributeValue>;
}


impl Item for HashMap<String, AttributeValue> {
    fn item(&self) -> HashMap<String, AttributeValue> {
        self.clone()
    }
}


impl Item for BTreeMap<String, AttributeValue> {
    fn item(&self) -> HashMap<String, AttributeValue> {
        self.into_iter().map(|(key, value)|{(key.to_owned(), value.to_owned())}).collect()
    }
}