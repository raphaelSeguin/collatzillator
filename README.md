```
collatzillator 0.1.0

USAGE:
    collatzillator [OPTIONS]

OPTIONS:
    -a, --altern-phase           
    -d, --duration <DURATION>    [default: 1000]
    -h, --help                   Print help information
    -i, --init <INIT>            [default: 1]
    -o, --output <OUTPUT>        [default: ]
    -p, --pitch <PITCH>          [default: 1]
    -r, --repeats <REPEATS>      [default: 1]
    -s, --step <STEP>            [default: 1]
    -V, --version                Print version information
```

## My favorite presets
- -i 1 -s 1000 -r 8 -p 0.4
- -i 1 -s 1000 -r 50 -p 0.5
- -i 1 -s 10000 -r 5 -p 0.2
- -i 100000 -s 1 -r 20 -p 0.25
- -i 1 -s 1 -r 1 -p 0.25
cargo run -- -d 100000 -i 1000000 -r 100 -s 1 -p 10 -a
cargo run -- -d 100000 -i 1000 -r 1000 -s 1 -p 1000 -a
cargo run -- -d 100000 -r 5 -s 1 -p 2 -a -i 200000 -n 16 --looping --drift 4
cargo run -- -d 100000 -r 100 -s 1 -p 4 -i 2 -n 8 --looping --drift 2 -a
cargo run -- -d 100000 -r 20 -s 1 -p 20 -i 9 -n 10 --looping --drift 1 -a
cargo run -- -d 100000 -r 20 -s 1 -p 120 -i 9 -n 100 --looping --drift 2
cargo run -- -d 100000 -r 4 -s 1 -p 0.5 -i 10000 -n 16 --looping --drift 1 -a
cargo run -- -d 100000 -r 4 -s 1 -p 8 -i 10000 -n 16 --looping --drift 1
cargo run -- -d 100000 -r 10 -s 3 -p 0.7777 -i 10000 -n 16 --looping --drift 8 -a
cargo run -- -d 100000 -r 1500 -s 2 -p 50 -i 1000 -n 20 --looping --drift 1 -a
