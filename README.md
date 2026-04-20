# Build docker build -t cjyx

## Run normally

  docker run -it cjyx

## Run as PID 1 (no init process wrapping your shell)

docker run -it --init=false cjyx

## BUILD & RUN shell

cd userland && make rn

## SHELL DEBUG MODE

make dbg

gdb ./cjsh

### DBG FUNCTIONS

(gdb) break main          # set breakpoint at function
(gdb) break exec.c:42    # set breakpoint at file:line
(gdb) run                 # start the program
(gdb) next        (n)     # step over — execute line, don't enter functions
(gdb) step        (s)     # step into — follow function calls
(gdb) print path  (p)     # print a variable's value
(gdb) backtrace   (bt)    # show the call stack (where did I come from?)
(gdb) continue    (c)     # resume until next breakpoint
(gdb) quit        (q)     # exit

─
