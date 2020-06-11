#!/bin/bash

# I need some way to say yes to all prompts
sudo yum install python37
curl -O https://bootstrap.pypa.io/get-pip.py
python3 get-pip.py

sudo yum install git -y
sudo yum update -y

git clone https://github.com/mmurray22/CS244B
