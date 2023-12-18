# WRAP-IAM
**currently WIP**

This tool is meant to be run as a wrapper around a command that uses AWS apis. It parses the client side metrics produced by the AWS API and generates an IAM policy that can be used to replicate the command run.

The goal is to make it easier to replicate the permissions needed to run a command. This is particularly useful for IAC tools like Terraform, where it can be difficult to determine the permissions needed to run a command for automation purposes.

## Usage
Currently this just listens for the AWS client side metrics and prints the policy to stdout on exit.
