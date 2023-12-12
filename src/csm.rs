use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum ApiCallType {
    ApiCall,
    ApiCallAttempt,
    Other,
}
#[derive(Debug, Deserialize, Serialize)]
enum ApiCallService {
    Sts,
    S3,
    Ec2,
    Other,
    Ecs,
    Rds,
    DynamoDB,
    Iam,
    Lambda,
    CloudWatch,
    CloudFormation,
    CloudTrail,
    CloudFront,
    Ssm,
    Sqs,
    Sns,
    Kms,
    Ecr,
}
#[derive(Debug, Deserialize, Serialize)]
enum ApiCallRegion {
    UsWest1,
    UsWest2,
    UsEast1,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiCall {
    api: String,
    region: ApiCallRegion,
    service: ApiCallService,
    type_: ApiCallType,
}
