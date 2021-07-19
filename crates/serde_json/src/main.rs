use serde_json::{Result, Value};

fn main() {
    println!("Hello, world!");
}

#[test]
fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    println!("{}", v.to_string());
    Ok(())
}

#[test]
fn test_json() {
    let x = serde_json::json!(null);
    println!("{} {} {}", x, x.is_null(), x.to_string());

    let x = serde_json::json!(true);
    println!("{} {} {}", x, x.is_boolean(), x.to_string());

    let x = serde_json::json!(1.11);
    println!("{} {} {}", x, x.is_number(), x.to_string());

    let x = serde_json::json!([1, "hello"]);
    println!("{} {} {} {}", x, x[0], x.is_array(), x.to_string());

    let x = serde_json::json!({"a":"girl"});
    println!("{} {} {} {}", x, x["a"], x.is_object(), x.to_string());
}
