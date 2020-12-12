export enum AvailableMoves {
    Immovable,
    Normal,
    Scout,
}

export interface Move {
    min: number,
    max: number,
}


export function equals(m: Move, availableMove: AvailableMoves) {
    let am = create(availableMove);
    return am.min === m.min && am.max === m.max;
}

export function create(piece: AvailableMoves): Move {
    switch (piece) {
        case AvailableMoves.Immovable:
            return { min: 0, max: 0 };
        case AvailableMoves.Normal:
            return { min: 0, max: 1 };
        case AvailableMoves.Scout:
            return { min: 0, max: 10 };
    }
}

