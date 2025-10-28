## Features Demo

This folder contains a set of projects to test

- Features of library
- Pulling a dependency from a custom location like github
- Different application using different set of features from same library

## Application

### [Maths operations library](./math-ops/)

This library would have four features

- sum: would perform add operation on inputs
- sub: would perform subtract operation on inputs
- div: would perform division operation on inputs
- mul: would perform multiplication operation on inputs
- noop: (default feature) would not perform any operation on inputs

### [CLI application one](./simple-ops/)

A cli application called simple-ops to takes input operands and ops on a command line. This application would only have a subset of features provided by maths ops library. Will only provide add and sub operations. Math Ops library will be pulled from github.

Sample application interaction

```shell
# addition operation
simple-ops -op1 <first_operand> -op1 <second_operant> --sum
# subtraction operation
simple-ops -op1 <first_operand> -op1 <second_operant> --sub
```

### [CLI application two](./full-ops/)

A cli application called full-ops to takes input operands and ops on a command line. This application will have all the features provided by maths ops library. Math Ops library will be pulled from github.

Sample application interaction

```shell
# addition operation
full-ops -op1 <first_operand> -op1 <second_operant> --sum
# subtraction operation
full-ops -op1 <first_operand> -op1 <second_operant> --sub
# multiplication operation
full-ops -op1 <first_operand> -op1 <second_operant> --mul
# division operation
full-ops -op1 <first_operand> -op1 <second_operant> --div
```

## Note

The very whole idea is not to implement a maths operations application, but to test

- building application that only pull some/all features from a library
- pull a library from a custom location from crate.io like github
