## Features Demo

This folder contains a set of projects to test

- Features of library
- Pulling a dependency from a custom location like github
- Different application using different set of features from same library

## Application

### Maths operations library

This library would have four features

- add: would perform add operation on inputs
- sub: would perform subtract operation on inputs
- div: would perform division operation on inputs
- mul: would perform multiplication operation on inputs
- noop: (default feature) would not perform any operation on inputs

### CLI application one

A cli application called simple-ops to takes inputs on and ops on command line. This application would only have a subset of features provided by maths ops library. Will only provide add and sub operations. This library will pull the maths library dependency from github.

Sample application interaction

```shell
# addition operation
simple-ops -op1 <first_operand> -op1 <second_operant> --add
# subtraction operation
simple-ops -op1 <first_operand> -op1 <second_operant> --sub
```

### CLI application two

A cli application called full-ops to takes inputs on and ops on command line. This application would only all the features provided by maths ops library. This library will pull the maths library dependency from github.

Sample application interaction

```shell
# addition operation
full-ops -op1 <first_operand> -op1 <second_operant> --add
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
