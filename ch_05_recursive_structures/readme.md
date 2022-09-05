# Chapter 5: Recursive Structures and Processes

Creation of recursive sequence diagrams, like "Diagram G", given in Chapter 5.

_Diagram G_

```
G(n) =  n - G(G(n - 1))  ~ for n > 0

G(0) =  0
```

## Running

```
USAGE:
    recursive_structures [OPTIONS]

OPTIONS:
    -f, --flip                       Use "flipped G" ?
    -h, --help                       Print help information
    -i, --iterations <ITERATIONS>    Number of iterations [default: 20]
    -V, --version                    Print version information
```