Takes in a json file as an argument, parses, and returns a Latin Hypercube Sample of the parameter space

Currently it parses the json file correctly and generates the data for the points. 
Argument handling can be improved, error handling can be improved, efficiency needs to be improved.
Documentation is also needed.
The current main issue is with generating permutations of the level ranges

You should always have more samples than factors, so generating more permutations of levels than you have levels themselves is an error

This is mostly to teach myself Design of Experiment and Rust.
