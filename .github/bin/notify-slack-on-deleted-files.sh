#!/usr/bin/env bash
curl -X POST -H 'Content-type: application/json' --data "{'text':'@channel Outside collaborator PR - files ( $1 ) are deleted. Please verify the PR manually. \n\n'}" $SLACK_WEBHOOK
