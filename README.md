Notes etc


## Qs
- Can I iterate on primitive containers and get values instead of references?
- Ways to throw, error out more elegantly (than `std::process::exit(1)` and eprintln 🫠?
- Why did we need lifetime annotators for Day 2?
- Does conditionally iterating reverse suck, or am I having skill issues?
- How do ovnership / copying shenanigans work for function return values?

## Add
- Something that downloads input files if they're not already saved
- The run function in main should probably be able to consume generic "runnable" functions (as upcoming days' results could have different types).
- Some sort of testing
- Ability to consume custom input files (instead of run functions opinionatedly reading dayXX.txt files)
