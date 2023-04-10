# Abbreviations

## Usage
`abbr put <SHORT> <LONG>` to store an abbreviation
`abbr get <SHORT>` to retrieve what an abbreviation means

**NOTE:** Can have multiple abbreviations that mean the same thing, in that case
you will get every possibility when you look it up
### Example
```sh
$ abbr put "CPU" "Central Processing Unit"
$ abbr get CPU
Central Processing Unit
```

## Background
I am in the process of learning a bunch of different computer science topics
but find that I am often struggling with acronyms. To deal with this I started
maintaining a document that contained all the abbreviations I bumped into, some
which I already knew, but added for completeness sake.

After a while I got the idea of creating a simple terminal app that I can use
to keep record of all the acronyms I bump into, so that it becomes simpler to
look up/add new abbreviations. As a bonus I can use this as an oppurtunity to
learn some **Rust** while I'm at it because that is a programming language that
I would like to start dipping my toes in.

## Features
I want to create a simple app that I can invoke in two modes with a commandline option:

- [x] Add an abbreviation
- [x] Look up an abbreviation

For adding an abbreviation, I would have to give two additional arguments, firstly
what the short form is, and secondly what it stands for. The program will then append
a line to a file kept on the local machine that stores the acronym.

For looking up an abbreviation, I would like to supply the short form, and then
the program simply reads out what it could stand for (there can be multiple answers).

