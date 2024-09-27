# Design Doc

## De-duplication
The current coreutils (and shell commands in general) have a lot of duplicated efforts. Here are a two of them.
* Command Line Argument Parsing
* Regex interpretation and implementation

### Command Line Argument Parsing
Every command ever has to implement some form of command line argument parsing. This can leave holes in their implemenation. Common problems 
include: handling spaces incorrectly, forgetting to add `--` to terminate flag parsing, not having shortened flags or only having shortened flags,
not being able to merge single character flags together.

##### What about `getopt`/`argp`?
While these are great libraries for C, they generally are not compatable with other languages. While there are great libraries in every language 
for command line argument parsing, they are all duplicated efforts that not everyone knows about.

##### What I want to do about it
I think that the shell should handle all argument parsing. I think that a command should have a file that describes its command line argument
configuration and options which the shell should use to parse the arguments for the program. Then environment variables are passed to the 
command. One holds the free floating arguments, and another should hold the flags, and another holds all key-value-pair arguments. 
This should simpify the effort that programs run will have to do. I will also provide libraries to allow for easily working with these arguments.

### Regex
The commands `awk`, `sed`, and `grep` all use regex to function. In the GNU Coreutils, only `sed` uses the POSIX regex library.
This means that there are 3 different regex implementations that each need to be maintained. Only the POSIX regex library is exposed.
There are reasons for these decisions but I believe that these special functions and commands should expose themselves as functions in a shared
object file that the user may link against. This should allow for much easier porting of shell scripts to programs.
This means that a command is just a wrapper around a shared library function.

## Single Responsiblity Principle
I wish to apply this principle to all tools in this repository. Every tool should be orthogonal to each other. This also can provide safety.
No longer will `rm` remove directories, this can prevent the user from accidentally deleting their operating system or files.
The goal should be to do one thing, and do it well.


## Minimal Dependencies
While using Rust with std, the only libraries that should be used are ones that abstract away operating system differences.
Libraries may also be used as placeholders until a more suitable replacement is found or is rewritten.
Also, every command must exist as a linkable C function.
