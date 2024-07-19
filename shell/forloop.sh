
MY_FIRST_LIST=(foo bar foobar bar foo barfoo)

for item in ${MY_FIRST_LIST[@]}; do echo -n $item | wc -c; 
done 
