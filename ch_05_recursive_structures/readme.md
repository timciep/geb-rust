# Chapter 5: Recursive Structures and Processes

Creation of recursive sequence diagrams, like "Diagram G" (`--rule` option), given in Chapter 5.

_Diagram G_

```
G(n) =  n - G(G(n - 1))  ~ for n > 0

G(0) =  0
```

## Usage

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

## Example

<img width="453" alt="image" src="https://user-images.githubusercontent.com/2245341/188519312-a0287663-af28-4c05-8d47-ea9618779df8.png">
