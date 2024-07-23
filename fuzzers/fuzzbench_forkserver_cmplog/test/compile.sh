#!/bin/bash

afl-cc -O0 test-cmplog.c -o test-cmplog.afl
AFL_LLVM_CMPLOG=1 afl-cc -O0 test-cmplog.c -o test-cmplog.cmplog

cp ./test-cmplog.afl ..
cp ./test-cmplog.cmplog ..
