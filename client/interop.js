// This file handles interop between our JavaScript client code,
// and the exposed Chess engine in WASM. It exposes two functions,
// bootstrap and tryMove

import init, { client_msg } from '../pkg/crawdad.js';

export async function bootstrap() {
    await init();
}

// color: 'White' | 'Black'
// from: [num, num]
// to: [num, num]
export function tryMove({ color, from, to }) {
    const colorCode = color === 'White'
        ? 0
        : 1;
    
    const [fromX, fromY] = from;
    const [toX, toY] = to;

    const moveResult = client_msg(colorCode, fromX, fromY, toX, toY);
    return deserializeMoveResult(moveResult);
}

// Mapping of integer code to piece enum
const PIECE_MAP = {
    0: null,
    1: { color: 'White', rank: 'Pawn' },
    2: { color: 'White', rank: 'Knight' },
    3: { color: 'White', rank: 'Bishop' },
    4: { color: 'White', rank: 'Rook' },
    5: { color: 'White', rank: 'Queen' },
    6: { color: 'White', rank: 'King' },
    7: { color: 'Black', rank: 'Pawn' },
    8: { color: 'Black', rank: 'Knight' },
    9: { color: 'Black', rank: 'Bishop' },
    10: { color: 'Black', rank: 'Rook' },
    11: { color: 'Black', rank: 'Queen' },
    12: { color: 'Black', rank: 'King' },
};

// Convert an int array into a meaningful move
function deserializeMoveResult([
    success, 
    from_x, 
    from_y, 
    from_piece, 
    to_x, 
    to_y, 
    to_piece, 
    turn,
]) {
    let tn = turn === 0 ? 'White' : 'Black';

    return {
        success: Boolean(success),
        from_x,
        from_y,
        from_piece: PIECE_MAP[from_piece],
        to_x,
        to_y,
        to_piece: PIECE_MAP[to_piece],
        turn: tn,
    };
}