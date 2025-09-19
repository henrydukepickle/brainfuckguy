# Overview
This esolang expands on brainfuck by adding a syntax for defining and calling functions.

The core structure of the language is the same as that of brainfuck, and the basic commands `+ - < > [ ]` work exactly as they do in brainfuck.
#Function Calling Syntax
To call a function, the syntax:

`{name}(input)(output)` is used.

`input` essentially stores instructions on how to construct the arguments to pass into the function from the data contained in the cells at the time of the function call. To support this, the additional command `|` is added.

When the `input` section of a function call is entered by the interpreter, a new empty list of arguments will be instantiated. All arguments must be single bytes, but there may be an arbitrary number of arguments. When the command `|` is interpreted inside of an
input block, the value at the current cell (in the larger function) is deleted (set to 0) and added to the list of arguments.

When the `input` section of a function call is exited, the function defined by `{name}` is called with the list of arguments provided and produces a list of outputs.

When the command `|` is interpreted inside of an `output` block, it writes the next output from the list of outputs to the current cell. If there are no more arguments, it writes `0` to that cell.

The commands `,` and `.` are slightly modified. When a function is called, a new list of `outputs` is instantiated. Every time the function interprets a `.`, the value at the current cell is deleted and added to the list of `outputs`. When the function terminates,
the list of outputs is returned.

When `,` is interpreted, it writes the next argument to the current byte. If there are no more arguments, it writes `0` to the current byte.

For instance, if we define the function `add` by:

`,>,[<+>-]<.`

It will read the first two arguments and add them, and then return a list containing just the result. If we wanted to, for instance, add a number to itself using this function, we might write a function `multiply_by_2` as:

`, {add}([>+>+<<-]>|>|)(|.)`

This function writes the first argument to the first cell, and then begins calling `add`. In the input, it clones the value in the first cell to the second and third cells, and then adds both of those values to the arguments, which are passed to `add`
at the end of the `input` block. In the output block, the single output returned by `add` is written to the current cell and then added to the `outputs` of `multiply_by_2`. Once that function terminates, it returns this single value.

#Using the Program

Functions can be created by creating a `.bf` file in the `Scripts` folder. The name of the function will be the name of the file (without the extension). There is not yet a build for the program, so the only way to run the program is by calling the
provided `start(args)` method in `main.rs` with a vector containing your desired arguments. This will call `main.bf` with these arguments.
