from objects import Piece, Pawn
from board import Tile, Board

board = Board.starting_board()
board.display()

# board.black_turn = True

board.play_move('e4')
board.play_move('e5')

board.display()
