import type { StateVariable } from "./model";
import type { GameState } from "./state";

export type Comparator = "lt" | "lte" | "eq" | "gte" | "gt";

export class Trigger {
    readonly comparator: Comparator;
    readonly target: StateVariable;
    readonly value: number;

    constructor(comparator: Comparator, target: StateVariable, value: number) {
        this.comparator = comparator;
        this.target = target;
        this.value = value;
    }

    isSatisfiedBy(state: GameState): boolean {
        const actualValue = {
            'population': state.population,
            'ecology': state.ecology,
            'money': state.money,
        }[this.target];

        switch (this.comparator) {
            case 'lt':
                return actualValue < this.value;
            case 'lte':
                return actualValue <= this.value;                
            case 'eq':
                return actualValue === this.value;
            case 'gte':
                return actualValue >= this.value;
            case "gt":
                return actualValue > this.value;
        }
    }
};
