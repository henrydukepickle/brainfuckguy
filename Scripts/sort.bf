
>>>>,[>>>>,] take inputs on the four cells
<<<<[<<<<] go to 0
>+ return to flag on 1
[
    - delete flag
    >>>>>>> move to 8
    [ loop on n plus 4
        [>+<-]>[>+<<+>-]< clone to plus 6 move to plus 4
        <<<< move to n
        [>+<-]>[>+<<+>-]> clone to plus 2
        {compare}(|>>>>|)(<<<<<|-) compare the values and write and write flipped flag to plus 1 move to plus 1
        >>+<< set tracker flag on plus 3
        [+ delete flag
            < go to n
            [<<<<] go to 0
            >[-]+>> set flag to true on 1 go to 3
            >>>> go to first tracker flag
            [>>>>]<<<< go to last tracker flag on plus 3
            <<<[>>+<<-]>>>>[<<<<+>>>>-]<<[>>+<<-] swap values
            < end on plus 1
        ]
        >>>>>>> move to plus 8
    ] when loop exited we found first zero value
    <<<<< go to last tracker flag
    [-<<<<] go to 3 and delete tracker flags
    << go to flag on 1
]
>>>[.>>>>]


