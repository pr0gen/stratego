import { position } from "./Position";

export type strategoCase = {
    type: string;
    active: boolean,
    isSelected: boolean
    canBeSelected : boolean,
    position: position;
}
