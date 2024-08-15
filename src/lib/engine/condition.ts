import type { Condition, StateVariable } from "./model";
import type { GameState } from "./state";

export function checkIsSatisfied(condition: Condition | undefined, state: GameState): boolean {
    if (!condition) {
        return true;
    }
    const actualValue = {
        'population': state.population,
        'ecology': state.ecology,
        'money': state.money,
    }[condition.target];

    switch (condition.comparator) {
        case 'lt':
            return actualValue < condition.value;
        case 'lte':
            return actualValue <= condition.value;
        case 'eq':
            return actualValue === condition.value;
        case 'gte':
            return actualValue >= condition.value;
        case "gt":
            return actualValue > condition.value;
    }
}
