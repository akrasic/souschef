use serde_json::{Map, Value};

const INDENT: &str = "  ";

/// traverse_json - Traverses JSON and prints out a formatted display
pub fn traverse_json(value: &Value, prefix: &str) {
    match value {
        Value::Object(map) => traverse_object(map, prefix),
        Value::Array(arr) => traverse_array(arr, prefix),
        Value::String(s) => println!("{}{}", prefix, s), // String without quotes
        Value::Number(n) => println!("{}{}", prefix, n),
        Value::Bool(b) => println!("{}{}", prefix, b),
        Value::Null => println!("{}null", prefix),
    }
}

/// traverse_object - Goes over the object values that are either, Objects, Arrays, or Strings, and
/// prints out their place in the JSON object.
fn traverse_object(map: &Map<String, Value>, prefix: &str) {
    for (key, value) in map {
        match value {
            Value::Object(_) => {
                println!("{}{}:", prefix, key);
                traverse_json(value, &format!("{}{}", prefix, INDENT));
            }
            Value::Array(_) => {
                println!("{}{}:", prefix, key);
                traverse_json(value, &format!("{}{}", prefix, INDENT));
            }
            Value::String(s) => println!("{}{}: {}", prefix, key, s), // String without quotes
            _ => println!("{}{}: {}", prefix, key, value),
        }
    }
}

fn traverse_array(arr: &[Value], prefix: &str) {
    for value in arr {
        match value {
            Value::Object(map) => {
                print!("{}- ", prefix);
                let new_prefix = format!("{}{}", prefix, INDENT);
                let mut first = true;
                for (key, val) in map {
                    if first {
                        match val {
                            Value::Array(_) => {
                                println!("{}:", key);
                                traverse_json(val, &format!("{}{}", prefix, INDENT));
                            }
                            Value::String(s) => println!("{}: {}", key, s), // String without quotes
                            _ => println!("{}: {}", key, val),
                        }
                        first = false;
                    } else {
                        match val {
                            Value::Array(_) => {
                                println!("{}{}:", new_prefix, key);
                                traverse_json(val, &format!("{}{}", new_prefix, INDENT));
                            }
                            Value::String(s) => println!("{}{}: {}", new_prefix, key, s), // String without quotes
                            _ => println!("{}{}: {}", new_prefix, key, val),
                        }
                    }
                }
            }
            Value::Array(_) => {
                println!("{}-", prefix);
                traverse_json(value, &format!("{}{}", prefix, INDENT));
            }
            Value::String(s) => println!("{}- {}", prefix, s), // String without quotes
            _ => println!("{}- {}", prefix, value),
        }
    }
}
