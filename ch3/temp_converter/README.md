# temperature converter

convert fahrenheit <> celsius

I need to learn how to handle errors better, evidently

```
$ cargo run
   Compiling temp_converter v0.1.0 (/home/ram/code/rust_playground/temp_converter)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/temp_converter`
Please enter a temperature to convert.
135.5
Is this temperature fahrenheit [f] or celsius [c]?
f
original: 135.5 unit: f
135.50°f converted is 57.50°
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/temp_converter`
Please enter a temperature to convert.
57.5
Is this temperature fahrenheit [f] or celsius [c]?
c
original: 57.5 unit: c
57.50°c converted is 135.50°
```
