#!/bin/bash

chown -R tester .
su tester -c "PATH=$PATH make check -k"
