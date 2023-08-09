use super::*;


/// Trait that enables type to be converted into a dynamodb Item
pub trait Item {
    /// convert the given type into a key value pair
    fn item(&self) -> HashMap<String, AttributeValue>;
}