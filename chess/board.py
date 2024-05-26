from objects import Piece, Pawn, Rook, Knight, Bishop, Queen, King

piece_map = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook]

def move_to_tup(move: str) -> tuple[int, int]:
    return 'abcdefgh'.index(move[0]), 8 - int(move[1])

class Tile:
    def __init__(self, x: int, y: int, piece: Piece | None = None) -> None:
        self.x = x
        self.y = y

        self.position = y * 8 + x
        self.piece = piece

        self.position_as_tup = x, y

    def __repr__(self) -> str:
        if self.piece:
            return self.piece.character
        return ' '

    def can_move_to(self, board, dest: tuple[int, int]) -> None:
        return self.piece.can_move_to(board, self.position_as_tup, dest)

class Board:
    def __init__(self, tiles: list[Tile] = None) -> None:
        self.tiles = tiles or [Tile(i, j) for i in range(8) for j in range(8)]
        self.black_turn = False

    @classmethod
    def starting_board(cls):
        return Board(tiles=[*[
            Tile(x, y, piece=(
                Pawn(True) if y == 1 else piece_map[x](True)
            )) for y in range(2) for x in range(8)
        ], *[
            Tile(x, y) for y in range(2, 6) for x in range(8)
        ], *[
            Tile(x, y, piece=(
                Pawn(False) if y == 6 else piece_map[x](False)
            )) for y in range(6, 8) for x in range(8)
        ]])

    def place(self, piece: Piece, x: int, y: int) -> None:
        self.tiles[y * 8 + x].piece = piece

    def display(self) -> None:
        for i, tile in enumerate(self.tiles):
            if i % 8 == 0:
                print(f'\n{8 - i // 8} |', end='')
            print(tile, end='|')
        print('\n   a b c d e f g h')


    def play_move(self, move: str) -> None:
        # Pawn move
        if len(move) == 2:
            col = 'abcdefgh'.index(move[0])
            pawns_in_col = [tile for tile in self.tiles if tile.x == col and isinstance(tile.piece, Pawn) and tile.piece.is_black == self.black_turn]

            tup_location = move_to_tup(move)

            res = None
            for tile in pawns_in_col:
                if tile.can_move_to(self, tup_location):
                    res = tile

            if not res:
                raise ValueError("Impossible Move")

        elif len(move) == 3:
            # Knight move
            if move[0] == 'N':
                ...

        dest_tile = self.tiles[tup_location[1] * 8 + tup_location[0]]
        res.piece, dest_tile.piece = None, res.piece
        dest_tile.piece.has_moved = True
        self.black_turn = not self.black_turn


