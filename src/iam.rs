use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::csm::ApiCall;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Statement {
    pub effect: String,
    pub action: HashSet<String>,
    pub resource: Vec<String>,
}

impl From<&crate::csm::ApiCall> for Statement {
    fn from(api_call: &crate::csm::ApiCall) -> Self {
        let resource = vec!["*".to_string()];
        let actions = AWS_MAP_STRUCT
            .get_iam_action(api_call)
            .unwrap_or_else(|| vec![]);
        Statement {
            effect: "Allow".to_string(),
            action: HashSet::from_iter(
                actions
                    .iter()
                    .map(|action| action.to_string())
                    .collect::<Vec<String>>(),
            ),
            resource,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    pub version: String,
    pub sid: Option<String>,
    pub statement: Vec<Statement>,
}

#[derive(Clone)]
pub struct PolicyBuilder {
    statements: Vec<(ApiCall, Statement)>,
}

impl PolicyBuilder {
    pub fn new() -> Self {
        PolicyBuilder {
            statements: Vec::new(),
        }
    }

    pub fn add_api_call(&mut self, api_call: &crate::csm::ApiCall) {
        let statement = Statement::from(api_call);
        let api_call = api_call.clone();
        self.statements.push((api_call, statement));
    }

    fn combine_statements_together(&self) -> Vec<Statement> {
        let mut combined_statements: HashMap<String, Statement> = HashMap::new();
        let mut new_statements = Vec::new();
        for (api_call, statement) in &self.statements {
            let key = format!("{}", api_call.service);
            let combined_statement = combined_statements.entry(key).or_insert(statement.clone());
            combined_statement.action.extend(statement.action.clone());
        }
        for (_, statement) in combined_statements {
            new_statements.push(statement);
        }
        new_statements
    }

    pub fn build(&self) -> Policy {
        let statements = self.combine_statements_together();
        Policy {
            version: "2012-10-17".to_string(),
            sid: None,
            statement: statements,
        }
    }
}

#[cfg(test)]
mod tests {}

#[derive(Debug, Deserialize, Serialize)]
struct AwsMappings {
    sdk_method_iam_mappings: HashMap<String, Vec<AwsMap>>,
}

impl AwsMappings {
    pub fn get_iam_action(&self, api_call: &crate::csm::ApiCall) -> Option<Vec<String>> {
        let key = format!(
            "{}.{}",
            api_call.service.to_string().to_uppercase(),
            api_call.api
        );
        let aws_map = self.sdk_method_iam_mappings.get(&key)?;
        let mut actions = Vec::new();
        for map in aws_map {
            let action = map.action.clone();
            actions.push(action);
        }
        Some(actions)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AwsMap {
    action: String,
    resource_mappings: Option<HashMap<String, ResourceMapTemplate>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResourceMapTemplate {
    template: String,
}

const AWS_MAP_FILE: &str = include_str!("../aws/aws/map.json");
lazy_static! {
    static ref AWS_MAP_STRUCT: AwsMappings = serde_json::from_str(AWS_MAP_FILE).unwrap();
}

#[cfg(test)]
mod test {
    use crate::iam;
    use serde_json::{json, Value};
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
            policy.statement[0]
                .action
                .iter()
                .next()
                .unwrap()
                .to_string(),
            json_val["Statement"][0]["Action"][0]
        );
        assert_eq!(
            policy.statement[0].resource[0],
            json_val["Statement"][0]["Resource"][0]
        );
    }
    #[test]
    fn test_parse_single_section() {
        let test_map: &str = r#"{
    "info": "This file is sourced from https://github.com/iann0036/iam-dataset",
    "sdk_permissionless_actions": [
        "DynamoDB.DescribeEndpoints",
        "STS.GetCallerIdentity",
        "STS.GetSessionToken"
    ],
    "sdk_method_iam_mappings": {
        "Budgets.CreateBudget": [
            {
                "action": "budgets:ModifyBudget",
                "resource_mappings": {
                    "BudgetName": {
                        "template": "${Budget.BudgetName}"
                    }
                }
            }
        ]}}"#;
        let aws_map_ = serde_json::from_str::<super::AwsMappings>(test_map).unwrap();
    }

    #[test]
    fn test_parse_real_map() {
        let aws_map_ = serde_json::from_str::<Value>(super::AWS_MAP_FILE).unwrap();
        for (key, value) in aws_map_["sdk_method_iam_mappings"].as_object().unwrap() {
            let AwsMap = match serde_json::from_value::<Vec<super::AwsMap>>(value.clone()) {
                Ok(map) => map,
                Err(err) => {
                    eprintln!("Error parsing {}: {}\n {}", key, value, err);
                    panic!("Error parsing map")
                }
            };
        }
    }
}
