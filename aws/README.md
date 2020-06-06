##Getting Started

We will spin up nodes on AWS to act as Kademlia nodes in the distributed hash table. In order to create, destroy, and manage these nodes, we need an AWS account. (Note this will also cost money, as the tests require many nodes and will likely exceed the free tier on AWS.)

The first step is to sign up for an AWS account. This README will assume that the account is a root user account and not an account managed by a larger organization.

Before creating an account install the AWS command line interferace. See this [link](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-welcome.html) and this [link](https://github.com/aws/aws-cli) for more info.

Next create an AWS account and generate a access id and key pair in "My Security Credentials" menu. Use the AWS CLI to configure your terminal to be able to run commands remotely. See this [link](https://github.com/aws/aws-cli) for more info. Note that you can also configure credentials manually without the AWS CLI.

`aws configure`

Next install boto3, the AWS SDK for Python. See this [link](https://github.com/boto/boto3) for more info.

`pip install boto3`

Next,  



## Reproduction

We attempted to reproduce [insert paper here] in spirit but it's important to highlight the differences between the paper and our reproduction. The goal of our reproduction was to performance test Kademlia, and our project is reproducing the original Kademlia paper. The original Kademlia paper itself does not contain performance related graphs or figures that were concrete enough for us to tangibly reproduce. For this reason, we used [insert paper here] as a guide for performance testing. That paper used the Nethawk East network simulator. From what we could gather, it's a closed source simulator and we were unable to access it. As an alternative, we explored using the Mininet network simulator or the Shadow network simulator, but ultimately thought it would be more interesting to test Kademlia in a live environment. This led us to the conclusion that we should be running real Kademlia nodes on AWS and testing over the live internet. This led to some differences from the [insert paper here] paper. Their paper used 400 Kademlia nodes, with the expectation that about 200 would be online at any given time. Nodes were live and faulty for an equal amount of time, that time being dictated by an exponentially distributed random variable for each node. Even after haggling with AWS, we were unable to increase the number of reserved AWS instances we were allowed to provision. The default number of allowed instances is 20, and it would have required a prohibitive expense to be allowed to increase our node limit. For this reason, we are testing at a significantly smaller scale. 
