# salsa step by step
this repository explain how to use salsa step by step.

1. Input Structure
2. Tracked Structure
3. Tracked Function
4. Interned Structure
5. Accumulator Structure
6. Deep Dive salsa

# Execute Samples
1. clone this repository
```bash
git clone https://github.com/takeshid/salsa_steps.git
```

> (optional) enter nix devshell or dienv
> ```bash
> nix develop .#
> # or
> direnv allow
> ```

2. run samples
for example, run `01_input` like following
```bash
$ cargo run --package input
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/input`
Debug: SourceProgram {
    [salsa id]: Id(0),
}
Id(0)
"print 11 + 11"
Id(1)
"print 11 + 11"
Id(2)
"print 11 + 12"
```
