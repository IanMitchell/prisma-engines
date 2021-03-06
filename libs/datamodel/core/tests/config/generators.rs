use crate::common::ErrorAsserts;
use datamodel::error::DatamodelError;

#[test]
fn serialize_generators_to_cmf() {
    let schema: &str = r#"
generator js1 {
    provider = "javascript"
    output = "../../js"
}

generator go {
    provider = "go"
    binaryTargets = ["a", "b"]
}"#;

    let expected = r#"[
  {
    "name": "js1",
    "provider": "javascript",
    "output": "../../js",
    "binaryTargets": [],
    "previewFeatures": [],
    "config": {}
  },
  {
    "name": "go",
    "provider": "go",
    "output": null,
    "binaryTargets": ["a","b"],
    "previewFeatures": [],
    "config": {}
  }
]"#;

    assert_mcf(&schema, &expected);
}

#[test]
fn preview_features_setting_must_work() {
    // make sure both single value and array syntax work
    let schema = r#"
        generator js {
            provider = "javascript"
            previewFeatures = "foo"
        }
        
        generator go {
            provider = "go"
            previewFeatures = ["foo", "bar"]
        } 
    "#;

    let expected = r#"[
  {
    "name": "js",
    "provider": "javascript",
    "output":null,
    "binaryTargets": [],
    "previewFeatures": ["foo"],
    "config": {}
  },
  {
    "name": "go",
    "provider": "go",
    "output":null,
    "binaryTargets": [],
    "previewFeatures": ["foo", "bar"],
    "config": {}
  }
]"#;

    assert_mcf(&schema, &expected);
}

#[test]
fn back_slashes_in_providers_must_work() {
    let schema = r#"
        generator mygen {
          provider = "../folder\ with\ space/my\ generator.js"
        }
    "#;

    let expected = r#"[
        {
          "name": "mygen",
          "provider": "../folder\\ with\\ space/my\\ generator.js",
          "output": null,
          "binaryTargets": [],
          "previewFeatures": [],
          "config": {}
        }
    ]"#;

    assert_mcf(&schema, &expected);
}

#[test]
fn new_lines_in_generator_must_work() {
    let schema = r#"
        generator go {
          provider = "go"
          binaryTargets = ["b", "c"]
        
        }
    "#;

    let expected = r#"[
        {
          "name": "go",
          "provider": "go",
          "output": null,
          "binaryTargets": ["b","c"],
          "previewFeatures": [],
          "config": {}
        }
    ]"#;

    assert_mcf(&schema, &expected);
}

#[test]
fn fail_to_load_generator_with_options_missing() {
    let schema = r#"
generator js1 {
    no_provider = "javascript"
    output = "../../js"
}
    "#;
    let res = datamodel::parse_configuration(schema);

    if let Err(error) = res {
        error.assert_is(DatamodelError::GeneratorArgumentNotFound {
            argument_name: String::from("provider"),
            generator_name: String::from("js1"),
            span: datamodel::ast::Span::new(1, 73),
        });
    } else {
        panic!("Expected error.")
    }
}

fn assert_mcf(schema: &str, expected_mcf: &str) {
    let config = datamodel::parse_configuration(schema).unwrap();
    let rendered = datamodel::json::mcf::generators_to_json(&config.generators);

    print!("{}", &expected_mcf);

    assert_eq_json(&rendered, expected_mcf);
}

fn assert_eq_json(a: &str, b: &str) {
    let json_a: serde_json::Value = serde_json::from_str(a).expect("The String a was not valid JSON.");
    let json_b: serde_json::Value = serde_json::from_str(b).expect("The String b was not valid JSON.");

    assert_eq!(json_a, json_b);
}
