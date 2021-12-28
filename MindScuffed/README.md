| Character | Function |
| :-------: | :------- |
| `>` | move pointer to right |
| `<` | move pointer to left |
| `+` | increments cell's value by 1 |
| `-` | decreases cell's value by 1 |
| `:` | sets the cell's value to `CELL_MAX` |
| `;` | sets the cell's value to 0 |
| `.` | prints the cell's value |
| `,` | takes user input |
| `[` | enters a loop if cell is not zero |
| `]` | jumps to front of loop |
| `*` | multiplies the cell's value by the cell right of the current, then sets the cell to the right to 0 |
| `/` | divides the cell's value by the cell right of the current, then sets the cell to the right to 0 |
| `&` | adds the cell's value to the right of the current cell, then sets the cell to the right to 0 |
| `~` | sets current cell's value to a random number between 0 and `CELL_MAX` |
| `\` | subtracts the cell's value to the right of the current cell, then sets the cell to the right to 0 |
| `l` | prints a list of all cell's value's |

## Example
```
++++++++>++</.l
```
- Program Will Output
  > 4
  > | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
