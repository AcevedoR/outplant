export type Chain = {
    title: string;
    cooldown: number;
    trigger?: Condition;
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
    if?: Condition;
};

export type Outcome = {
    event: string;
    in?: number;
    weight?: number;
    effects?: { [key: string]: boolean };
    if?: Condition;
};

export type Effect = {
    operation: "add" | "subtract";
    target: StateVariable;
    value: number;
    type: "instant" | "permanent";
};

export type Condition = {
    comparator: Comparator;
    target: StateVariable;
    value: number;
};

export type StateVariable = "population" | "ecology" | "money";

export type Comparator = "lt" | "lte" | "eq" | "gte" | "gt";
