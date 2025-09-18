, input small number on 0 / 0
[>+>+<<-] copy 0 to 1 and 2 / 0
>>>>, input big number on 4 / 4
[ enter loop on big number / 4
    -<<- subtract 1 from big tally on 4 and small tally on 2 / 2
    [ enter conditional / 2
        [>+<-] move 2 to 3 / 2
        >>>+<<< set flag on 5 / 2
    ] / 2
    >[<+>-]< move 3 to 2 / 2
    >>> move to flag on 5 / 5
    - flip flag on 5 / 5
    [ enter conditional on 5 / 5
        <<<< go to copy of small number on 1 / 1
        [<+>-]< copy 1 to 0 / 0
        [>+>+<<-] copy 0 to 1 and 2 / 0
        >>>>> move to 5 / 5
        + delete flag / 5
        >+< increment quotient on 6 / 5
    ] / 5
    < move to big number on 4 / 4
] exit loop / 4
>>. output quotient / 6
<<<< go to remaining tally on 2 / 2
[<->-] subtract tally from remaining small number to get remainder / 2
<. output remainder / 2