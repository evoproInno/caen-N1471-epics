#!/bin/sh
[ -d "/tmp/.ssh" ] && cp -r /tmp/.ssh ~/
/bin/sh -c "while sleep 1000; do :; done"