#!/bin/bash
set -e
WORKSPACE=$(i3-msg -t get_workspaces | jq -r ".[] | select(.focused == true) | .name")
WORKSPACE_WD=$(echo $WORKSPACE | cut -s -d':' -f2)
WORKSPACE_WD="${WORKSPACE_WD/#\~/$HOME}"
if [ -n "$WORKSPACE_WD" ]; then
  echo $WORKSPACE_WD
else
  echo $HOME
fi
