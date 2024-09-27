# Shell Doc

## What should the Shell do?
The shell's job is to be interactive and helpful when calling commands. Fish is arguably the gold standard when it comes to this. 

## Ancillary Responsibilities

##### Command Line argument Parsing
In this shell, it should also be able to parse commandline arguments according to a file that describes all flags. This leaves the utils to just do what they are supposed to.

##### Warnings
The shell should also provide warnings based on certain conditions. This will make the commands safer to use.
The warnings come from the same file that defines the commandline arguments.

##### Command Code Completion
The shell should provide completions for each command. This again will be provided in the same file that defines the commandline arguments.
However, in the meantime, I think it is best if we can use another shell's completions to fill in the gap.

##### Syntax Highlighting
There should be helpful highlighting that the shell can use to signify to the user that something is wrong

## Other Design Considerations
Like the other utils, each util should be its own C function in a shared library. The shell is no exception.
However, due to the completity of the shell, it should be split into multiple functions. That way it may provide easy testiblity.
