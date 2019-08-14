# Fluence Ketos

A [Fluence](https://fluence.network/) application that allows uploading and executing [Ketos](https://github.com/murarth/ketos) scripts

[Live App](http://ipfs.fluence.one:8080/ipfs/QmYCqjCBmECUuWRQZmmSCBBQq178kkvgYftvxRcDEQrMbH/index.html)

![Screenshot](Screenshot.png)

## What is Fluence?
Fluence is an efficient trustless computation platform that allows to achieve few seconds request processing latency and cost efficiency similar to traditional cloud computing. To run computations, Fluence uses a WebAssembly virtual machine, which allows to deploy into the decentralized environment applications written in multiple programming languages.

## What is Ketos?
Ketos is a Lisp dialect functional programming language.
The primary goal of Ketos is to serve as a scripting and extension language for programs written in the Rust programming language.

## Example

#### Add

- Input

```
(+ 1 1)
```

- Output

```
2
```

#### Factorial

- Input

```
(define (factorial n)
          (cond
            ((< n 0) (panic "factorial got negative integer"))
            ((<= n 1) 1)
            (else (* n (factorial (- n 1))))))
(factorial 6)
(factorial 7)
```

- Output

```
factorial
720
5040
```

#### Fibonacci

- Input

```
(define (fibonacci n)
  (if (<= n 2)
      1
      (+ (fibonacci (- n 1)) (fibonacci (- n 2)))))
(fibonacci 10)
(fibonacci 11)
(fibonacci 12)
```

- Output

```
fibonacci
55
89
144
```

## Compile

```bash
~$ cargo +nightly build --target wasm32-unknown-unknown --release
```
