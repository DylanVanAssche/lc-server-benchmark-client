#!/bin/bash

INTERVAL=30000
cd target/release
for i in {1..250}
do
   echo "Starting POLLING client $i with interval $INTERVAL ms"
   ./lc-server-benchmark-client --mode polling --interval $INTERVAL &
   sleep 0.1
done
