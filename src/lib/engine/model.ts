import {Trigger} from './trigger';

export type Chain = {
    title: string;
    cooldown: number;
    trigger?: Trigger;
    autoSelect: boolean;
};

export type ChainEvent = {
    text: string;
    effects?: { [key: string]: boolean };
    choices?: Array<Choice>,
    next?: Array<Outcome>;
};

export type Choice = {
    text: string;
    next: Array<Outcome>;
    effects?: { [key: string]: boolean };
};

export type Outcome = {
    event: string;
    in?: number;
    weight?: number;
    effects?: { [key: string]: boolean };
};

export type Effect = {
    operation: "add" | "subtract";
    target: StateVariable;
    value: number;
    type: "instant" | "permanent";
};

export type StateVariable = "population" | "ecology" | "money";
