#!/bin/bash

# Server1 details
USER1='username1'
HOST1='Server1'
LOGFILE1='/path/to/logfile.log'

# Server2 details
USER2='username2'
HOST2='Server2'
DESTINATION_PATH='/Users/kaj/Desktop/phd_code/casetest/src'

# Command to execute on Server1
CMD="tail -f $LOGFILE1"

# Stream the log from Server1 to a file on Server2
ssh $USER1@$HOST1 "$CMD" | ssh sgxy "cat > $DESTINATION_PATH/log.log"sudo log show --last 1m