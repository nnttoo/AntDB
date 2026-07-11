use crate::ant_resp::value::Value;

 
pub fn get_list_fields(values: &Vec<Value>) -> Vec<String> {
    let mut keys = Vec::with_capacity(values.len());

    for val_variant in values {
        if let Value::Bulk(key) = val_variant {
            keys.push(key.clone());
        }
    }

    keys
}
