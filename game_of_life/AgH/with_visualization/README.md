## Installation

### Build library
In order to use `Agent Harmony` you have to build it first.
Make sure that you have submodules initialized:
```bash
git submodule update --init --recursive
```
Then go into library directory.
```
cd Agent-Harmony
```
And do required steps to build it.
```
mkdir build
cd build
cmake ..
make
```

### Build project
Having library built make sure that you have `SFML-3.0.0` installed,
if not [get](https://www.sfml-dev.org/download/sfml/3.0.0/) yourself one.

Next, change paths in `CMakeList.txt`.
```cmake
include_directories(../../../Agent-Harmony/include /path/to/SFML/include)
link_directories(../../../Agent-Harmony/build /path/to/SFML/lib)
```

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
