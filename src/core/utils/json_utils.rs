use json::JsonValue;

pub fn find_value_from_json_key(
    json: &JsonValue,
    key_order: &[&str],
) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let mut value = json;
    for key in key_order {
        if value.has_key(key) {
            value = &value[*key];
        } else {
            return Err(format!(
                "Could not find key <{:?}> in {}",
                key_order.last(),
                json.pretty(4),
            )
            .into());
        }
    }
    Ok(value.clone())
}

pub fn parse_json_as_str(value: &JsonValue) -> Result<String, Box<dyn std::error::Error>> {
    let parsed = value
        .as_str()
        .ok_or("Cannot parse value as str")?
        .to_string();
    if parsed.is_empty() || parsed.to_lowercase() == "null" || parsed.to_lowercase() == "none" {
        return Err(format!("Cannot parse value as str: value is empty or null or none").into());
    }
    Ok(parsed)
}

pub fn parse_json_as_f64(value: &JsonValue) -> Result<f64, Box<dyn std::error::Error>> {
    value
        .to_string()
        .parse::<f64>()
        .map_err(|e| format!("Failed to parse '{}' as f64: {}", value, e).into())
}

pub fn parse_json_as_u64(value: &JsonValue) -> Result<u64, Box<dyn std::error::Error>> {
    value
        .to_string()
        .parse::<u64>()
        .map_err(|e| format!("Failed to parse '{}' as u64: {}", value, e).into())
}

pub fn parse_json_as_bool(value: &JsonValue) -> Result<bool, Box<dyn std::error::Error>> {
    // Сначала пытаемся как булево значение
    if let Some(bool_val) = value.as_bool() {
        return Ok(bool_val);
    }
    if let Some(int_val) = value.as_u8() {
        match int_val {
            0 => return Ok(false),
            1 => return Ok(true),
            _ => {}
        }
    }
    // Если не получилось, пытаемся как строку
    if let Some(str_val) = value.as_str() {
        match str_val.to_lowercase().as_str() {
            "true" => return Ok(true),
            "1" => return Ok(true),
            "0" => return Ok(false),
            "false" => return Ok(false),
            _ => {}
        }
    }

    Err("Could not parse value as boolean".into())
}
