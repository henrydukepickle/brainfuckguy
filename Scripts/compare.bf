returns 1 if the first passed input is leq the second and 0 otherwise
,>,< take inputs
[
    [>>+<<-]
    >>>>+<<<<
]
>>[<<+>>-]>>
-[++.-]<<<<
[ enter loop on 0
    ->-< subtract from both
    [
        [>>+<<-] copy 0 to 2
        >>>>+<<<< set flag on 4
    ]
    >>[<<+>>-]
    >>-
    [
        ++.
        -
        <<<[-]++>>> set 1 to 2 to ensure no double return
    ]
    <<<
    [
        [>>+<<-] copy 1 to 3
        >>>>+<<<< set flag on 5
    ]
    >>[<<+>>-]
    >>-
    [
        +.
    ]
    <<<<<
]