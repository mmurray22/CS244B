import boto3
import sys
from botocore.exceptions import ClientError
import logging
from time import sleep
import math
import numpy
import datetime
from dateutil.tz import tzlocal


ec2_client = boto3.client('ec2')

def get_running_instances():
    """ Returns a list of instance objects for instances that are already running.
        Does not spawn any instances.
    """

    ec2_client = boto3.client('ec2')
    response = ec2_client.describe_instances()

    ec2 = boto3.resource('ec2')
    instances = []

    for reservation in response['Reservations']:
        for instance in reservation["Instances"]:
            if instance['State']['Name'] != 'terminated': #we don't want to include the terminated instances
                instances.append(ec2.Instance(instance["InstanceId"])) #creates the ec2 instance object

    for instance in instances:
        instance.wait_until_running #this assumes all instances are running, TODO maybe check and then start if not?
        print(instance)
        print(instance.public_ip_address)

    return instances


def cancel_command(command_id):
    remaining_uncanceled = True
    while(remaining_uncanceled):
        remaining_uncanceled = False

        response = ssm_client.list_commands(CommandId = command_id)

        for command in response['Commands']:
            if command['Status'] == 'InProgress' or command['Status'] == 'Pending':
                remaining_uncanceled = True

                #this does not guarantee a command will be cancelled so we must double check or send multiple requests
                ssm_client.cancel_command(CommandId=command['CommandId'])

def cancel_all_commands():
    """
        This function cancels any pending (still running) SSM commands.
        It enables us to start fresh.
    """

    remaining_uncanceled = True
    while(remaining_uncanceled):
        remaining_uncanceled = False

        response = ssm_client.list_commands()

        for command in response['Commands']:
            if command['Status'] == 'InProgress' or command['Status'] == 'Pending':
                remaining_uncanceled = True

                print(command)
                #this does not guarantee a command will be cancelled so we must double check or send multiple requests
                ssm_client.cancel_command(CommandId=command['CommandId'])


def run_nodes(ksize, alpha, instances, mean_time=100):
    ''' This function will run the actual testing for success rate once
        we have all the instances spawned and set up. It assumes instances[0]
        is the first (bootstrapping) node that never churns. This is desgined to
        be defined within another function or in a larger script.

        @mean_time is how long each server instance should be alive on
        average. The same value is used for how long a server is down when
        it fails. The mean uptime and downtime and therefore the same.
    '''

    def execute_commands_on_linux_instances(client, commands, instance_ids):
        """Runs commands on remote linux instances
        :param client: a boto/boto3 ssm client
        :param commands: a list of strings, each one a command to execute on the instances
        :param instance_ids: a list of instance_id strings, of the instances on which to execute the command
        :return: the response from the send_command function (check the boto3 docs for ssm client.send_command() )
        """

        resp = client.send_command(
            DocumentName="AWS-RunShellScript", # One of AWS' preconfigured documents
            Parameters={'commands': commands},
            InstanceIds=instance_ids,
        )
        return resp

    first_node_ip = instances[0].public_ip_address
    if(first_node_ip == None):
        println("\n\n\n\n\n ERROR: Failed to get public IPv4 address for the first (bootstrapping) node")

    first_node_command = 'python3 /home/ec2-user/CS244B/aws/kademlia_scripts/first_node.py {} {}'.format(ksize, alpha)
    bootstrap_server_command = 'python3 /home/ec2-user/CS244B/aws/kademlia_scripts/run_node.py {} 8468 {} {}'.format(first_node_ip, ksize, alpha)

    def get_command(key):
        _get_command = 'python3 /home/ec2-user/CS244B/aws/kademlia_scripts/get.py {} 8468 {}'.format(first_node_ip, key)
        return _get_command
    def set_command(key, value):
        _set_command = 'python3 /home/ec2-user/CS244B/aws/kademlia_scripts/set.py {} 8468 {} {}'.format(first_node_ip, key, value)
        return _set_command
    def sleep_command(time):
        _sleep_command = 'sleep {}'.format(time)
        return _sleep_command

    set_instance_index = len(instances)-2
    get_instance_index = len(instances)-1

    # this will be used to set values and determine whether we got the right value back or not
    truth_table = {
        'a':1,
        'b':2,
        'c':3,
        'd':4,
        'e':5,
        'f':6,
        'g':7,
        'h':8,
        'i':9,
        'j':10,
        'k':11,
        'l':12,
        'm':13,
        'n':14,
        'o':15,
        'p':16,
        'q':17,
        'r':18,
        's':19,
        't':20,
        'u':21,
        'v':22,
        'w':23,
        'x':24,
        'y':25,
        'z':26,
    }

    # note the assigned timeout is not used for the first instance, which  always remains live
    # or for the last two instances which are used to get and set nodes and do not run long lived servers
    timeouts = [numpy.random.exponential(mean_time) for i in range(len(instances))]

    command_responses = ["" for i in range(len(instances))]

    #start first node. The first node is the bootstrapping node that does not churn
    commands = [first_node_command]
    instance_ids = [instances[0].id]
    command_responses[0] = execute_commands_on_linux_instances(ssm_client, commands, instance_ids)
    sleep(3)

    # start the kademlia servers, these nodes will churn
    for index in range(1, len(instances)-2): # we leave the final two instances for set and get requests

        commands = [bootstrap_server_command]
        instance_ids = [instances[index].id]
        command_responses[index] = execute_commands_on_linux_instances(ssm_client, commands, instance_ids)

    sleep(10) #allow the instances a chance to connect and populate routing tables

    def churn():

        def get_elapsed_time(command):
            time_response = ssm_client.list_commands(CommandId = command['CommandId'])
            dt = time_response['Commands'][0]['RequestedDateTime']
            now = datetime.datetime.now(dt.tzinfo)
            return (now - dt).seconds


        #TODO there is a problem where commands are run on nodes that are already running kademlia
        for command_idx in range(1, len(command_responses)-2):
            command = command_responses[command_idx]['Command']

            #get updated info on the status of the command
            command_info = ssm_client.get_command_invocation(CommandId=command['CommandId'],
                                              InstanceId= command['InstanceIds'][0])
            print(command_info)

            if get_elapsed_time(command) > timeouts[command_idx]: #simulate node churn
                cancel_command(command['CommandId']) #this is a blocking call and makes sure the command is cancelled

                commands = [sleep_command(timeouts[command_idx])]
                instance_ids = [command['InstanceIds'][0]]
                command_responses[command_idx] = execute_commands_on_linux_instances(ssm_client, commands, instance_ids)

            elif(command_info['Status'] == 'Success'): # restart kademlia server, simulated downtime has ended
                assert('sleep' in command['Parameters']['commands'][0])

                # recalculate the exponentially distributed timeout here
                timeouts[command_idx] = numpy.random.exponential(mean_time)

                #we must restart the kademlia server
                commands = [bootstrap_server_command]
                instance_ids = [command['InstanceIds'][0]]
                command_responses[command_idx] = execute_commands_on_linux_instances(client, commands, instance_ids)

            elif command_info['Status'] == 'InProgress' or command_info['Status'] == 'Pending':
                pass
                #this is expected
            else:
                print("\n\n\n\n\ UNEXPECTED BEHAVIOR: The command didn't succeed or just keep running")
                print(command_info['Status'])

    def evaluate_get_response(get_response, key, value):
        stdout_result = get_response['StandardOutputContent']
        print(stdout_result)
        result_list = stdout_result.split(' ')
        idx = result_list.index('result:')
        result = result_list[idx+1]
        if "None" in result:
            return False
        if str(value) in result:
            return True
        else:
            print("Inconclusive Result: {} for value {}".format(stdout_result, value))
            return False

    def wait_until_complete(command):
        sleep(1)

        while(True):
            command_info = ssm_client.get_command_invocation(CommandId=command['Command']['CommandId'],
                                              InstanceId= command['Command']['InstanceIds'][0])

            if(command_info['Status'] == "Success"):
                return True
            if(command_info['Status'] == "Failed" or command_info['Status'] == "TimedOut"):
                print("Print command we were waiting for failed: {}".format(command_info))
                return False
            else:
                sleep(1)


    success_list = []

    for key, value in truth_table.items():

        churn()

        #begin to make get and set requests to judge the success rate
        commands = [set_command(key, value)]
        instance_ids = [instances[set_instance_index].id] #this will be the set instance index
        command_responses[set_instance_index] = execute_commands_on_linux_instances(ssm_client, commands, instance_ids)
        if not wait_until_complete(command_responses[set_instance_index]):
            #TODO relaunch
            pass

        churn()

        commands = [get_command(key)]
        instance_ids = [instances[get_instance_index].id] #this will be the get instance index
        command_responses[get_instance_index] = execute_commands_on_linux_instances(ssm_client, commands, instance_ids)
        if not wait_until_complete(command_responses[get_instance_index]):
            #TODO relaunch
            pass

        get_response = ssm_client.get_command_invocation(CommandId=command_responses[get_instance_index]['Command']['CommandId'],
                                          InstanceId= command_responses[get_instance_index]['Command']['InstanceIds'][0])
        print(get_response)

        if evaluate_get_response(get_response, key, value):
            success_list.append(1)
        else:
            success_list.append(0)

    success_rate = sum(success_list)/len(truth_table)
    return success_rate


run_nodes(20, 3, instances)



instances = get_running_instances()

# I used https://stackoverflow.com/questions/42645196/how-to-ssh-and-run-commands-in-ec2-using-boto3 for help
# AWS SSM Docs: https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/ssm.html
# AWS SSM Info: https://docs.aws.amazon.com/systems-manager/latest/userguide/what-is-systems-manager.html
# AWS SSM Getting Started Guide: https://aws.amazon.com/getting-started/hands-on/remotely-run-commands-ec2-instance-systems-manager/
ssm_client = boto3.client('ssm')

cancel_all_commands()
