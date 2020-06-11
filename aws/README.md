## Getting Started

We will spin up nodes on AWS to act as Kademlia nodes in the distributed hash table. In order to create, destroy, and manage these nodes, we need an AWS account. (Note this will also cost money, as the tests require many nodes and will likely exceed the free tier on AWS.)

The first step is to sign up for an AWS account. This README will assume that the account is a root user account and not an account managed by a larger organization.

Before creating an account install the AWS command line interferace. See this [link](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-welcome.html) and this [link](https://github.com/aws/aws-cli) for more info.

Next create an AWS account and generate a access id and key pair in "My Security Credentials" menu. Use the AWS CLI to configure your terminal to be able to run commands remotely. See this [link](https://github.com/aws/aws-cli) for more info. Note that you can also configure credentials manually without the AWS CLI.

`aws configure`

Next install boto3, the AWS SDK for Python. See this [link](https://github.com/boto/boto3) for more info.

`pip install boto3`

Next,  use the commands in setup.sh to download the required packages and code on an AWS machine image. Launch all AWS images with this package.

The code to actually run performance tests is in the `testing_scripts\` folder. `create_instances.py` actually creates instances on AWS. The number of instances to create can be specified as a command line argument. Please replace the global variables with the equivalent for your AWS account. You will need to generate an AWS AMI (amazon machine image), as previously mentioned, and create a security group that allows arbitrary protocols to connect to the instance over port 8468. This is the port the Kademlia scripts use. It could be changed it you so desire in the actual code in `testing.py` file. For simplicity I recommend creating a security group that allows any protocol to connect on any port. This is the solution I used.

As an alternative to using the scripts in `testing_scripts\`, the Jupyter notebook `Workbook.ipynb` was were I ran the overwhelming majority of the tests. It contains the same code as in the python scripts in `testing_scripts\` in a logical order.

Honestly, making AWS work for a system like this is very hard to reproduce as someone without access to my specific AWS account because many of the scripts make assumptions about the specific settings of my account. These assumptions are made clear with comments in the code everywhere possible. This is a drawback of running many instances on a live network at scale instead of running a network simulator.

If you would just like to generate the resulting figures, pleas run the `generate_graphs.py` python script!

## Reproduction

We attempted to reproduce ["Performance evaluation of a Kademlia-based communication-oriented P2P system under churn"](https://www.sciencedirect.com/science/article/abs/pii/S1389128609002990?via%3Dihub) in spirit but it's important to highlight the differences between the paper and our reproduction. The goal of our reproduction was to performance test Kademlia, and our project is reproducing the original Kademlia paper. The original Kademlia paper itself does not contain performance related graphs or figures that were concrete enough for us to tangibly reproduce. For this reason, we used ["Performance evaluation of a Kademlia-based communication-oriented P2P system under churn"](https://www.sciencedirect.com/science/article/abs/pii/S1389128609002990?via%3Dihub) as a guide for performance testing. That paper used the Nethawk East network simulator. From what we could gather, it's a closed source simulator and we were unable to access it. As an alternative, we explored using the Mininet network simulator or the Shadow network simulator, but ultimately thought it would be more interesting to test Kademlia in a live environment. This led us to the conclusion that we should be running real Kademlia nodes on AWS and testing over the live internet. This led to some differences from the performance testing paper. Their paper used 400 Kademlia nodes, with the expectation that about 200 would be online at any given time. Nodes were live and faulty for an equal amount of time, that time being dictated by an exponentially distributed random variable for each node. Even after haggling with AWS, we were unable to increase the number of reserved AWS instances we were allowed to provision. The default number of allowed instances is 20, and it would have required a prohibitive expense to be allowed to increase our node limit. For this reason, we are testing at a significantly smaller scale.

I mostly worked on the reproduction in a Jupyter notebook. This was due to the fact that my location internet connection was quite poor and each datapoint took a long time to generate (15-45 minutes). Unfortunately, I would occasionally error out of the script or lose connectivity. For this reason, I wrote each intermediate result to a file and then read those results to generate figures. I also was never able to complete a full run of the python script, since it would take many hours. It should work, and did, in the Jupyter notebook, but I had lots of trouble maintaining constant internet connectivity during that time. Alternatively, I realized quite late in the project that I could run the controlling scripts on an AWS machine, but already had many results. I also tried and had trouble getting AWSCLI to run with root privileges on an EC2 instance. Perhaps this isn't possible for security reasons.
