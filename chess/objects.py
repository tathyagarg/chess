class Piece:
    def __init__(self, character: str, is_black: bool = False) -> None:
        self.character = character
        self.has_moved = False
        self.is_black = is_black

    def can_move_to(self, board, src: tuple[int, int], dest: tuple[int, int]) -> bool:
        return False

class Pawn(Piece):
    def __init__(self, is_black: bool = False) -> None:
        super().__init__(character=['P', 'p'][is_black], is_black=is_black)

    def can_move_to(self, board, src: tuple[int, int], dest: tuple[int, int]) -> bool:
        forward_col = [tile for tile in board.tiles if tile.x == src[0] and (tile.y > src[1] if self.is_black else tile.y < src[1])]

        distance = abs(src[1] - dest[1])

        if distance == 2 and self.has_moved:
            return False

        tiles_in_path = forward_col[:distance] if board.black_turn else forward_col[-distance:]

        return all(tile.piece is None for tile in tiles_in_path)

class Rook(Piece):
    def __init__(self, is_black: bool = False) -> None:
        super().__init__(character=['R', 'r'][is_black], is_black=is_black)

class Bishop(Piece):
    def __init__(self, is_black: bool = False) -> None:
        super().__init__(character=['B', 'b'][is_black], is_black=is_black)

class Knight(Piece):
    def __init__(self, is_black: bool = False) -> None:
        super().__init__(character=['N', 'n'][is_black], is_black=is_black)

class Queen(Piece):
    def __init__(self, is_black: bool = False) -> None:
        super().__init__(character=['Q', 'q'][is_black], is_black=is_black)

class King(Piece):
    def __init__(self, is_black: bool = False) -> None:
        super().__init__(character=['K', 'k'][is_black], is_black=is_black)


