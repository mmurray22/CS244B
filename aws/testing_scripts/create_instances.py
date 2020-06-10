import boto3
import sys
from botocore.exceptions import ClientError
import logging
from time import sleep
import math
import numpy
import datetime
from dateutil.tz import tzlocal
import sys

"""
This script creates aws instances. Usage python creates_instances <number of instances to create>
If a number of instances to create is not specified, one instance will be created. 
"""

# I used https://github.com/awsdocs/aws-doc-sdk-examples/blob/master/python/example_code/ec2/create_instance.py for help here
def create_ec2_instance(image_id, instance_type, keypair_name, security_group, security_group_id, iam_role_name, min=1, max=1):
    """Provision and launch an EC2 instance
    The method returns without waiting for the instance to reach
    a running state.
    :param image_id: ID of AMI to launch, such as 'ami-XXXX'
    :param instance_type: string, such as 't2.micro'
    :param keypair_name: string, name of the key pair
    :return Dictionary containing information about the instance. If error,
    returns None.
    """

    # Provision and launch the EC2 instance
    ec2_client = boto3.client('ec2')
    try:
        response = ec2_client.run_instances(ImageId=image_id,
                                            InstanceType=instance_type,
                                            KeyName=keypair_name,
                                            SecurityGroupIds = [security_group_id],
                                            SecurityGroups= [security_group],
                                            IamInstanceProfile={
                                                #'Arn': 'arn:aws:iam::578971879148:instance-profile/EnablesEC2ToAccessSystemsManagerRole',
                                                'Name': iam_role_name
                                            },
                                            MinCount=min,
                                            MaxCount=max)
    except ClientError as e:
        logging.error(e)
        return None
    return response

if __name__ == '__main__':

    ec2_client = boto3.client('ec2')

    AMI_IMAGE_ID = 'ami-02f53f5f90a9cc773' #This is the Amazon machine image ID
    INSTANCE_TYPE = 't2.micro' #this is the type of vm ec2 instance
    KEYPAIR_NAME = 'key' #this is the name of the key that allows you to ssh into the instance
    SECURITY_GROUP = 'kademlia-all-access' #this is the name of the security group. I custom defined a security group that allows any protocol to connect on any port.
    SECURITY_GROUP_ID = 'sg-06474ad72b0f3fd58' #this is the id associated with the security group on our account
    IAM_ROLE_NAME = 'EnablesEC2ToAccessSystemsManagerRole' #this is the iam role to allow ssm to send commands to each instance.

    # Set up logging
    logging.basicConfig(level=logging.DEBUG,
                        format='%(levelname)s: %(asctime)s: %(message)s')

    max = 1
    if (sys.argc == 2):
        max = sys.argv[1]

    response = create_ec2_instance(AMI_IMAGE_ID, INSTANCE_TYPE, KEYPAIR_NAME, SECURITY_GROUP, SECURITY_GROUP_ID, IAM_ROLE_NAME, min=1, max = max)

    ec2 = boto3.resource('ec2')

    # AWS Instance Type Docs: https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/ec2.html#instance
    instances = [] #this is a python list of type ec2 instance objects

    for instance in response['Instances']:
        logging.info(f'Launched EC2 Instance {instance["InstanceId"]}')
        logging.info(f'    VPC ID: {instance["VpcId"]}')
        logging.info(f'    Private IP Address: {instance["PrivateIpAddress"]}')
        logging.info(f'    Current State: {instance["State"]["Name"]}')
        print(instance)
        instances.append(ec2.Instance(instance["InstanceId"])) #creates the ec2 instance object

    for instance in instances:
        instance.wait_until_running()
        print(instance)
        print(instance.public_ip_address)
