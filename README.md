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
cargo run -- -d 100000 -i 1 -s 1000 -r 8 -o -1.3219280948873622  
cargo run -- -d 100000 -i 1 -s 1000 -r 50 -o -1  
cargo run -- -d 100000 -i 1 -s 10000 -r 5 -o -2.321928094887362  
cargo run -- -d 100000 -i 100000 -s 1 -r 20 -o -2  
cargo run -- -d 100000 -i 1 -s 1 -r 1 -o -2  
cargo run -- -d 100000 -i 1000000 -r 100 -s 1 -o 3.321 -a  
cargo run -- -d 100000 -i 1000 -r 1000 -s 1 -o 9.965 -a  
cargo run -- -d 100000 -r 5 -s 1 -o 1 -a -i 200000 -n 16 --looping --drift 4  
cargo run -- -d 100000 -r 100 -s 1 -o 2-i 2 -n 8 --looping --drift 2 -a  
cargo run -- -d 100000 -r 20 -s 1 -o 4.321928094887363 -i 9 -n 10 --looping --drift 1 -a  
cargo run -- -d 100000 -r 20 -s 1 -o 6.906 -i 9 -n 100 --looping --drift 2  
cargo run -- -d 100000 -r 4 -s 1 -o -1 -i 10000 -n 16 --looping --drift 1 -a  
cargo run -- -d 100000 -r 4 -s 1 -o 3 -i 10000 -n 16 --looping --drift 1  
cargo run -- -d 100000 -r 10 -s 3 -o -0.3640134962543769 -i 10000 -n 16 --looping --drift 8 -a  
cargo run -- -d 100000 -r 1500 -s 2 -o 5.643856189774724 -i 1000 -n 20 --looping --drift 1 -a  
  
cargo run -- -d 100000 -r 201 -s 1 -o 8 -i 10000  
cargo run -- -d 100000 -r 201 -s 1 -o 8 -i 1 -n 20 --looping --drift 1 -a  
cargo run -- -d 100000 -r 2 -s 1 -o -4 -i 9999999999999  
cargo run -- -d 100000 -r 2 -s 1 -o -2 -i 9999999999999  
cargo run -- -d 100000 -r 1 -s 1 -o -3 -i 9999999999995 --looping -n 16 --drift 1  
cargo run -- -d 100000 -r 1 -s 1 -o -3 -i 999999999999999999999999995 --looping -n 16 --drift 1  
cargo run -- -d 100000 -r 8 -s 500 -o -1 -i 999999999999995 -a  
cargo run -- -d 100000 -r 500 -s 500 -o 7 -i 999999999999995 -a --looping -n 2000 --drift 1  
cargo run -- -d 100000 -r 500 -s 500 -o 7 -i 9995 -a --looping -n 2000 --drift 1  
  
cargo run -- -d 100000 -r 250 -s 997 -o 8 -i 1 -a --looping -n 99900 --drift 100  
cargo run -- -d 100000 -r 8 -s 100 -o 0 -i 99999999 -a --looping -n 5000 --drift 1  
cargo run -- -d 100000 -r 16 -s 10 -o -1 -i 99999999 -a --looping -n 50 --drift 5  
  
