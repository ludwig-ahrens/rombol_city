# Rombol city solver

The code solves the Rombol city puzzle. The goal of the game is to place nine
blocks with different shapes on a square. One spot on the square is blocked by
a pin.

The user enters the position of the pin and the program prints all possible
block arrangements for the given pin position.

The problem is solved via backtracking. Blocks are placed subsequently. If a
block cannot be placed, a previous block is placed in another position or
rotated.
