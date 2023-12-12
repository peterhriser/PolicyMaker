use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Statement {
    #[serde(rename = "Effect")]
    pub effect: String,
    #[serde(rename = "Action")]
    pub action: Vec<String>,
    #[serde(rename = "Resource")]
    pub resource: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Policy {
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Statement")]
    pub statement: Vec<Statement>,
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::iam;

    const EXAMPLE_BUCKET: &str = "example-bucket";

    fn create_example_iam_json() -> String {
        json!({
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Effect": "Allow",
                    "Action": [
                        "s3:ListBucket"
                    ],
                    "Resource": [
                        format!("arn:aws:s3:::{}", EXAMPLE_BUCKET)
                    ]
                }
            ]
        })
        .to_string()
    }

    #[test]
    fn test_iam_from_json() {
        let json = &create_example_iam_json();
        let policy = serde_json::from_str::<iam::Policy>(&json).unwrap();
        let json_val: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(policy.version, json_val["Version"]);
        assert_eq!(
            policy.statement[0].effect,
            json_val["Statement"][0]["Effect"]
        );
        assert_eq!(
            policy.statement[0].action[0],
            json_val["Statement"][0]["Action"][0]
        );
        assert_eq!(
            policy.statement[0].resource[0],
            json_val["Statement"][0]["Resource"][0]
        );
    }
}
