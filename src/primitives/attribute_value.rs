use super::*;

/// <p>Represents the data for an attribute.</p>
/// <p>Each attribute value is described as a name-value pair. The name is the data type, and the value is the data itself.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.NamingRulesDataTypes.html#HowItWorks.DataTypes">Data Types</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum AttributeValue {
    #[default]
    #[serde(with="null", rename = "NULL")]
    /// <p>An attribute of type Null. For example:</p>
    /// <p> <code>"NULL": true</code> </p>
    None,
    /// <p>An attribute of type Boolean. For example:</p>
    /// <p> <code>"BOOL": true</code> </p>
    BOOL(bool),
     /// <p>An attribute of type String. For example:</p>
    /// <p> <code>"S": "Hello"</code> </p>
    S(String),
    //// <p>An attribute of type Number. For example:</p>
    /// <p> <code>"N": "123.45"</code> </p>
    /// <p>Numbers are sent across the network to DynamoDB as strings, to maximize compatibility across languages and libraries. However, DynamoDB treats them as number type attributes for mathematical operations.</p>
    N(String),
    /// <p>An attribute of type Binary. For example:</p>
    /// <p> <code>"B": "QmluYXJ5"</code> </p>
    B(Binary),
    /// <p>An attribute of type String Set. For example:</p>
    /// <p> <code>"SS": ["Giraffe", "Hippo" ,"Zebra"]</code> </p>
    SS(Vec<String>),
    /// <p>An attribute of type Number Set. For example:</p>
    /// <p> <code>"NS": ["42.2", "-19", "7.5", "3.14"]</code> </p>
    /// <p>Numbers are sent across the network to DynamoDB as strings, to maximize compatibility across languages and libraries. However, DynamoDB treats them as number type attributes for mathematical operations.</p>
    NS(Vec<String>),
     /// <p>An attribute of type Binary Set. For example:</p>
    /// <p> <code>"BS": ["U3Vubnk=", "UmFpbnk=", "U25vd3k="]</code> </p>
    BS(Vec<Binary>),
    /// <p>An attribute of type Map. For example:</p>
    /// <p> <code>"M": {"Name": {"S": "Joe"}, "Age": {"N": "35"}}</code> </p>
    M(HashMap<String, AttributeValue>),
    /// <p>An attribute of type List. For example:</p>
    /// <p> <code>"L": [ {"S": "Cookies"} , {"S": "Coffee"}, {"N": "3.14159"}]</code> </p>
    L(Vec<AttributeValue>)
}


impl Attribute for AttributeValue {
    fn attribute(&self) -> AttributeValue {
        self.clone()
    }

    fn value(attribute: AttributeValue) -> Result<Self, AttributeError> {
        Ok(attribute)
    }
}