EXE="./target/release/computer-org-homework-1"
ONE_FLAG="-a"
TWO_FLAG="-b"
THREE_FLAG="-c"
FOUR_FLAG="-d"

RUN_QUESTION_ONE="$EXE $ONE_FLAG"
RUN_QUESTION_TWO="$EXE $TWO_FLAG"
RUN_QUESTION_THREE="$EXE $THREE_FLAG"
RUN_QUESTION_FOUR="$EXE $FOUR_FLAG"

print_and_run () {
    echo "$1"
    $1
}

echo -e "\nQuestion 1:\n"

print_and_run "$RUN_QUESTION_ONE abcdefgHIJKLMNOP"
print_and_run "$RUN_QUESTION_ONE asdf::;;::HJKL POIU,.,.qwer"

echo -e "\nQuestion 2:\n"

print_and_run "$RUN_QUESTION_TWO 11110000000000001111"
print_and_run "$RUN_QUESTION_TWO 0 1 10 11 100 101 110 111 1000 1001 1010 1011 1100 1101 1110 1111 10000 10001 10010 10011 10100 10101 10110 10111"

echo -e "\nQuestion 3:\n"

print_and_run "$RUN_QUESTION_THREE -3 -2 -1 0 1 2 3 4"
print_and_run "$RUN_QUESTION_THREE -2147483648 2147483647"
print_and_run "$RUN_QUESTION_THREE -559038737 -558904578 -1095894322"

echo -e "\nQuestion 4:\n"

print_and_run "$RUN_QUESTION_FOUR -3 -2 -1 0 1 2 3 4"
print_and_run "$RUN_QUESTION_FOUR -2147483648 2147483647"
print_and_run "$RUN_QUESTION_FOUR -559038737 -558904578 -1095894322"
