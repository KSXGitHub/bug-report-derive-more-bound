use derive_more::Display;
use std::ops::Deref;

#[derive(Debug, Display)]
#[display(bound = "Value: Deref, Value::Target: Display")]
#[display(fmt = "{}", "&value")]
pub struct MyStruct<Value> {
    pub value: Value,
}
