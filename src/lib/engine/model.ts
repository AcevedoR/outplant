export type Chain = {
    title: string;
    cooldown: number;
    trigger?: Condition;
    autoSelect: boolean;
    usedVariables: StateVariable[];
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
    variables?: { [key: string]: string };
};

export type Effect = {
    operation: "add" | "subtract";
    target: StateVariable;
    value: number;
    type: "instant" | "permanent";
};

export type Condition = StateCondition | VariableCondition | { allOf: Condition[] } | { anyOf: Condition[] };

export type StateCondition = {
    comparator: Comparator;
    target: StateVariable | "time";
    value: number;
};

export type StateVariable = "population" | "ecology" | "money";

export type Comparator = "lt" | "lte" | "eq" | "gte" | "gt";

export type VariableCondition = {
    comparator: VariableComparator;
    target: string;
    value: string;
}

export type VariableComparator = "eq" | "not";
