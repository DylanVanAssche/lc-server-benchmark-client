#!/bin/bash

cd target/release
for i in {1..1500}
do
   echo "Starting PUSHING client $i"
   ./lc-server-benchmark-client --mode pushing &
   sleep 0.1
done
