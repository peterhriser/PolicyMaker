use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ApiCallType {
    ApiCall,
    ApiCallAttempt,
    #[serde(other)]
    Other,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ApiCallService {
    Sts,
    S3,
    Ec2,
    Ecs,
    Rds,
    #[serde(other)]
    Other,
}

impl Display for ApiCallService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiCallService::Sts => write!(f, "sts"),
            ApiCallService::S3 => write!(f, "s3"),
            ApiCallService::Ec2 => write!(f, "ec2"),
            ApiCallService::Ecs => write!(f, "ecs"),
            ApiCallService::Rds => write!(f, "rds"),
            ApiCallService::Other => write!(f, "other"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ApiCallRegion {
    #[serde(rename = "us-west-1")]
    UsWest1,
    #[serde(rename = "us-west-2")]
    UsWest2,
    #[serde(rename = "us-east-1")]
    UsEast1,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ApiCall {
    pub api: String,
    pub region: ApiCallRegion,
    pub service: ApiCallService,
    #[serde(rename = "Type")]
    pub type_: ApiCallType,
}

#[cfg(test)]
mod test {
    use crate::csm::{ApiCall, ApiCallRegion, ApiCallService, ApiCallType};

    #[test]
    fn test_from_json() {
        let json = r#"{
  "Api": "ListObjectsV2",
  "AttemptCount": 1,
  "ClientId": "",
  "FinalHttpStatusCode": 200,
  "Latency": 356,
  "MaxRetriesExceeded": 0,
  "Region": "us-west-2",
  "Service": "S3",
  "Timestamp": 1702364837492,
  "Type": "ApiCall",
  "UserAgent": "aws-cli/2.13.28 Python/3.11.6 Linux/6.2.6-76060206-generic exe/x86_64.pop.22 prompt/off command/s3.ls",
  "Version": 1
}"#;
        let api_call = serde_json::from_str::<ApiCall>(&json).unwrap();
        assert_eq!(api_call.api, "ListObjectsV2");
        assert_eq!(api_call.region, ApiCallRegion::UsWest2);
        assert_eq!(api_call.service, ApiCallService::S3);
        assert_eq!(api_call.type_, ApiCallType::ApiCall);
    }

    #[test]
    fn test_from_json_handle_unknowns() {
        let json = r#"{
  "Api": "ListObjectsV2",
  "AttemptCount": 1,
  "ClientId": "",
  "FinalHttpStatusCode": 200,
  "Latency": 356,
  "MaxRetriesExceeded": 0,
  "Region": "us-west-2",
  "Service": "S4",
  "Timestamp": 1702364837492,
  "Type": "ApiCallWrong",
  "UserAgent": "aws-cli/2.13.28 Python/3.11.6 Linux/6.2.6-76060206-generic exe/x86_64.pop.22 prompt/off command/s3.ls",
  "Version": 1
}"#;
        let api_call = serde_json::from_str::<ApiCall>(&json).unwrap();
        assert_eq!(api_call.api, "ListObjectsV2");
        assert_eq!(api_call.region, ApiCallRegion::UsWest2);
        assert_eq!(api_call.service, ApiCallService::Other);
        assert_eq!(api_call.type_, ApiCallType::Other);
    }
}
