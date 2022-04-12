# An assignment repository
## Build
Inside the assignment folder created by the command “git clone”, run the following commands:
```
cd build
cmake ..
```
You only need to run these commands once.

## Compile and test
From inside the build folder (run `cd build` if needed), run the following commands to compile and test your code:

```
make tester
./tester
```

Make sure all the test cases/assertions pass.

You may run a subset of the test cases by passing an argument tag.

```
./tester [tag1]
```

This will run all the test cases tagged with `[tag1]`.

## Compile and run
From inside the build folder (run `cd build` if needed), run the following commands to compile and run your application:

```
make main
./main   
``` 

**NOTE**: All files should be read from or written to the `resources` folder.
