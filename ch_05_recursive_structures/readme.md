# Chapter 5: Recursive Structures and Processes

Creation of recursive sequence diagrams, like "Diagram G" (`--rule` option), given in Chapter 5.

_Diagram G_

```
G(n) =  n - G(G(n - 1))  ~ for n > 0

G(0) =  0
```

## Running

```
USAGE:
    ./recursive_structures [OPTIONS] --rule <RULE>
    or: cargo r -- [OPTIONS] --rule <RULE>
    ex: cargo r -- --rule g

OPTIONS:
    -h, --help                       Print help information
    -i, --iterations <ITERATIONS>    Number of iterations [default: 20]
    -r, --rule <RULE>                Which rule to use? (g, g_flip, f, m)
    -s, --steps                      Show steps?
    -V, --version                    Print version information
```