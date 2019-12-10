use serde_json::Value;

pub fn csv_processor(
    input: String,
    iteration: &usize,
    _current_sequence: &(u32, u32),
    _sequence_lenght: &usize,
) -> String {
    let mut line = input.lines();
    if *iteration > 0 {
        // removing header
        line.next();
    }

    line.filter(|elem| !elem.is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn json_processor(
    input: String,
    iteration: &usize,
    _current_sequence: &(u32, u32),
    sequence_lenght: &usize,
) -> String {
    let v: Value = serde_json::from_str(&input).unwrap();
    let mut result = String::new();

    if *iteration == 0 {
        result.push_str("[\n");
    }

    match &v["response"]["docs"] {
        Value::Array(vv) => {
            result.push_str(
                &vv.iter()
                    .map(|e| format!("\t{}", e.to_string()))
                    .collect::<Vec<String>>()
                    .join(",\n"),
            );
        }
        _ => (),
    }

    if *iteration == *sequence_lenght {
        result.push_str("\n]");
    } else {
        result.push_str(",")
    }

    result
}

pub fn jsonl_processor(
    input: String,
    _iteration: &usize,
    _current_sequence: &(u32, u32),
    _sequence_lenght: &usize,
) -> String {
    let v: Value = serde_json::from_str(&input).unwrap();
    let mut result = String::new();

    match &v["response"]["docs"] {
        Value::Array(vv) => {
            result.push_str(
                &vv.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }
        _ => (),
    }

    result
}

mod tests {
    use super::*;

    #[test]
    fn test_jsonl_processor() {
        let input = r#"{
            "response" : {
                "docs": [
                    {"doc 1": "1"},
                    {"doc 2": "2"}
                ]
            }
        }"#
        .to_string();

        let expected_output = "{\"doc 1\":\"1\"}\n{\"doc 2\":\"2\"}";
        assert_eq!(jsonl_processor(input, &0, &(0, 0), &0), expected_output);
    }

    #[test]
    fn test_json_processor() {
        let input = r#"{
            "response" : {
                "docs": [
                    {"doc 1": "1"},
                    {"doc 2": "2"}
                ]
            }
        }"#
        .to_string();

        let expected_output = "[\n\t{\"doc 1\":\"1\"},\n\t{\"doc 2\":\"2\"}\n]";
        assert_eq!(json_processor(input, &0, &(0, 0), &0), expected_output);

        let input = r#"{
            "response" : {
                "docs": [
                    {"doc 1": "1"},
                    {"doc 2": "2"}
                ]
            }
        }"#
        .to_string();

        let expected_output = "[\n\t{\"doc 1\":\"1\"},\n\t{\"doc 2\":\"2\"},";
        assert_eq!(json_processor(input, &0, &(0, 0), &1), expected_output);

        let input = r#"{
            "response" : {
                "docs": [
                    {"doc 1": "1"},
                    {"doc 2": "2"}
                ]
            }
        }"#
        .to_string();

        let expected_output = "\t{\"doc 1\":\"1\"},\n\t{\"doc 2\":\"2\"},";
        assert_eq!(json_processor(input, &1, &(0, 0), &3), expected_output);
    }

    #[test]
    fn csv_json_processor() {
        let input = "HEADER_1;HEADER_2\n1;2\na;b".to_string();
        assert_eq!(
            csv_processor(input, &0, &(0, 0), &0),
            "HEADER_1;HEADER_2\n1;2\na;b"
        );

        let input = "HEADER_1;HEADER_2\n1;2\na;b".to_string();
        assert_eq!(csv_processor(input, &1, &(0, 0), &1), "1;2\na;b");
    }
}
