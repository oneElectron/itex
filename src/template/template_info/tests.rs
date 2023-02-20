#![cfg(test)]

#[test]
fn json_parsing() {
    let data = r#"
        {
            "name": "Test Template",
            "description": "A very good template",
            "id": 43
        }"#;

    let output = super::parse_json(data);

    assert_eq!(output.name, "Test Template".to_string());
    assert_eq!(output.description, "A very good template".to_string());
    assert_eq!(output.id, 43);
}
