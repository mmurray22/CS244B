#!/bin/bash

# I need some way to say yes to all prompts

sudo yum install python37
curl -O https://bootstrap.pypa.io/get-pip.py
python3 get-pip.py --user

sudo yum install git -y
sudo yum update -y

# clone the kademlia repo and change it to make it work
git clone https://github.com/bmuller/kademlia
cd kademlia
pip install -r dev-requirements.txt
pip install rpcudp
pytest
