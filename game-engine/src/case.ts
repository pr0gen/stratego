export interface Case {
	state: CaseState,
	x: number,
	y: number
}

export enum CaseState {
	Unreachable = -1,
	Empty = 0,
	Full = 1
}
