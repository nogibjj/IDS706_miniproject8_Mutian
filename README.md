# Week 8 Mini Project 8

## Goal
Rewrite a Python script for data processing in Rust, highlighting the improvements in speed and resource usage.

## Overview
This project demonstrates the efficiency difference between python and rust by caculating some mean values from a stock buy and sell dataset. I compare the time difference, and find that rust is 5500x times faster than python.  


## Run
 * For Python:
    Enter the root directory, and run command `make pyall`
   
* For Rust:
    Enther the `rscopy` directory, and run command `make all`

### Makefile Commands 
`make install` for rust dependancy setup

`make format` for rust formatting 

`make lint` for rust linting

`make test` for rust testing

`make all` to run all rust commands

`make pyinstall` for python dependancy installation

`make pyformat` for python formatting 

`make pylint` for python linting

`make pytest` for python testing

`make pyall` to run all python commands

## Test

<img width="790" alt="image" src="https://github.com/nogibjj/IDS706_miniproject8_Mutian/assets/108935314/0fcfb127-1f76-4ce5-a920-bd4ed155ef80">


## Output

rust version:

<img width="877" alt="image" src="https://github.com/nogibjj/IDS706_miniproject8_Mutian/assets/108935314/37f507f5-1e36-414c-92a2-4d0cfb06a90f">



python version:

 <img width="761" alt="image" src="https://github.com/nogibjj/IDS706_miniproject8_Mutian/assets/108935314/c387a4b6-5c42-4fc2-91c6-e97d70f75b3a">

