##Getting Started 

We will spin up nodes on AWS to act as Kademlia nodes in the distributed hash table. In order to create, destroy, and manage these nodes, we need an AWS account. (Note this will also cost money, as the tests require many nodes and will likely exceed the free tier on AWS.) 

The first step is to sign up for an AWS account. This README will assume that the account is a root user account and not an account managed by a larger organization. 

Before creating an account install the AWS command line interferace. See this [link](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-welcome.html) for more info.

`pip install aws`

Next create an AWS account and generate a access id and key pair in "My Security Credentials" menu. Use the AWS CLI to configure your terminal to be able to run commands remotely. 

`aws configure`


