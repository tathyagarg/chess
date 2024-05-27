from enum import Enum

class Color(Enum):
    WHITE = 0
    BLACK = 1

class Piece:
    characters = '--'
    def __init__(self, color: Color) -> None:
        """
        @param characters: The black character, then white character in a string
        """
        self.color = color
        self.character = self.__class__.characters[color.value]

    def is_pinned(self, src: tuple[int, int], dest: tuple[int, int], board):
        ...

class Pawn(Piece):
    characters = 'pP'
    def __init__(self, color: Color) -> None:
        super().__init__(color)


class Tile:
    def __init__(self, x: int, y: int) -> None:
        self.x, self.y = x, y
        self.position = x, y

        self.piece = None

class Board:
    def __init__(self):
        self.tiles = [[Tile(i, j) for i in range(8)] for j in range(8)]


