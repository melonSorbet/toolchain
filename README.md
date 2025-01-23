# Toolchain
Toolchain is a Utility for saving, using and distributing your commands. For example you might end up using the same command to always clean, build and test your code. Or you might have a command for checking for a specific service. Instead of having to write down all of your commands you can now just save it one time and give it a name just like an alias and use it however you want.
## How to run toolchain

## How to use Toolchain

## How to build Toolchain

### Build with Rust

### Build and deploy with Nix

## Goals of this Project

### Features that are planned
- Saving and reusing commands.
- having a good interface to change, add and combine commands.
- Add a distribution functionality so that you can easily share your commands with other people
- make a class system where you can define classes of commands. meaning things like linux/windows commands or Development/testing
- add a variable functionality so that you cant only define static commands but rather can define things like echo_variable := echo &a and when calling the toolchain you can call it by passing in the values which would look like this -> tool echo_variable hello_world
- implement a type safe small and blazingly fast application that can be used even on the worst potato

## Contribution
This project is going to be Open Sourced



