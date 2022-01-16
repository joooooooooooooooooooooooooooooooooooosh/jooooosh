# jooooosh
`jooooosh` is a BF-equivalent language that consists of programs of the form `jo+sh` (plus newlines).
Program execution begins at the first `j` encountered in the file, and ends when a matching `sh` is found. If there is no matching `sh` then no operation is performed.

Each line in between (inclusive) the `j` and `sh` represents a single bf command based on the number of `o`s contained on that line.

| Command        | Description                                                       |
| -------------- | ----------------------------------------------------------------- |
|   `o`          | Move the pointer to the right                                     |
|   `oo`         | Move the pointer to the left                                      |
|   `ooo`        | Increment the memory cell at the pointer                          |
|   `oooo`       | Decrement the memory cell at the pointer                          |
|   `ooooo`      | Output the character signified by the cell at the pointer         |
|   `oooooo`     | Input the character and store it at the cell at the pointer       |
|   `ooooooo`    | Jump past the matching `oooooooo` if the cell at the pointer is 0        |
|   `oooooooo`   | Jump back to the matching `ooooooo` if the cell at the pointer is not 0 |

When reading input, `EOF` leaves the current cell unchanged.
