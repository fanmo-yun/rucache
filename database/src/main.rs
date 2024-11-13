use nosql::{nosql::RuCache, value::Value};

mod nosql;

fn main() {
    let mut ru = RuCache::new();

    ru.put("key1".to_string(), Value::Int(12)).unwrap();
    ru.put("key2".to_string(), Value::Double(13.02)).unwrap();
    ru.put("key3".to_string(), Value::Str("yes".to_string())).unwrap();
    ru.put("key4".to_string(), Value::Int(12)).unwrap();
    ru.put("key5".to_string(), Value::Double(14.23)).unwrap();
    ru.put("key6".to_string(), Value::Str("wow".to_string())).unwrap();
    ru.put("key7".to_string(), Value::Int(12)).unwrap();
    ru.put("key8".to_string(), Value::Double(15.12)).unwrap();
    ru.put("key1".to_string(), Value::Int(15)).unwrap();

    println!("{}", ru);
}
