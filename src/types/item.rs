use super::*;

pub trait Item {
    fn item(&self) -> HashMap<String, AttributeValue>;
}