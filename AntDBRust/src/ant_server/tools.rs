use crate::{ant_db::db_hashmap_child::ValPairs, ant_resp::value::Value};

pub fn get_list_fields(values: &Vec<Value>) -> Vec<String> {
    let mut keys = Vec::with_capacity(values.len());

    for val_variant in values {
        if let Value::Bulk(key) = val_variant {
            keys.push(key.clone());
        }
    }

    keys
}

pub fn get_list_valpair(mut values: Vec<Value>) -> Vec<ValPairs> {
    let mut vpair: Vec<ValPairs> = Vec::new();
    while values.len() >= 2 {
        let field_variant = values.remove(0);
        let val_variant = values.remove(0);

        if let (Value::Bulk(field), Value::Bulk(value)) = (field_variant, val_variant) {
            vpair.push(ValPairs {
                key: field,
                value: value,
            });
        }
    }

    vpair
}
