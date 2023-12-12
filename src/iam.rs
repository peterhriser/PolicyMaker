use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Statement {
    pub effect: String,
    pub action: Vec<String>,
    pub resource: Vec<String>,
}

impl From<&crate::csm::ApiCall> for Statement {
    fn from(api_call: &crate::csm::ApiCall) -> Self {
        let resource = vec![format!("arn::aws::{}**", api_call.service)];
        Statement {
            effect: "Allow".to_string(),
            action: vec![api_call.api.clone()],
            resource,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    pub version: String,
    pub statement: Vec<Statement>,
}

pub struct PolicyBuilder {
    policy: Policy,
}

impl PolicyBuilder {
    pub fn new() -> Self {
        PolicyBuilder {
            policy: Policy {
                version: "2012-10-17".to_string(),
                statement: Vec::new(),
            },
        }
    }

    pub fn add_statement(mut self, statement: Statement) -> Self {
        self.policy.statement.push(statement);
        self
    }

    pub fn build(self) -> Policy {
        self.policy
    }
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
