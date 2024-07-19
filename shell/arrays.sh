#!/opt/homebrew/bin/bash

MY_FIRST_LIST=(foo bar foobar bar foo barfoo)
echo $MY_FIRST_LIST 

echo ${MY_FIRST_LIST[@]}

echo ${MY_FIRST_LIST[2]}
echo ${MY_FIRST_LIST[3]}
echo ${MY_FIRST_LIST[4]}
