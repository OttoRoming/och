#!/usr/bin/env brush

LC_ALL=C.UTF-8 expect << "EOF"
set timeout -1
spawn make tests
expect eof
lassign [wait] _ _ _ value
exit $value
EOF
