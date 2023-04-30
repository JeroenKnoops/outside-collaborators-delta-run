#!/usr/bin/env bash
curl -X POST -H 'Content-type: application/json' --data "{'text':'@channel Outside collaborator check failed.\n\n'}" $SLACK_WEBHOOK
