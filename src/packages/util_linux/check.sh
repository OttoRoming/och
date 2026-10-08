#!/bin/bash

touch /etc/fstab
chown -R tester .
su tester -c "make -k check"
