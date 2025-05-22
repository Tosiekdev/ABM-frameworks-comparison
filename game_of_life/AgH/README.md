## Installation

Create a folder for build files and cd into it.
```
mkdir build
cd build
```

Call cmake on a parent directory.
```
cmake ..
```

Build a project.
```
make
```

Run a built program.
```
./AgH <number of epochs> <file with init state> <size>
```

Provided size should match number of rows and columns in 
the file with an initial state. Initial state should be 
denoted as chain of 0 and 1 seperated only by the newlines
implying end of the row.
