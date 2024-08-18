import type { Condition, StateVariable, VariableCondition } from "./model";
import type { GameState } from "./state";
import type { VariableStore } from "./variable_store";

export function checkIsSatisfied(condition: Condition | VariableCondition | undefined, state: GameState, chainTitle: string = ''): boolean {
    if (!condition) {
        return true;
    }
    if (determineIfIsVariableCondition(condition)) {
        return checkVariableConditionIsSatisfied(condition, state.chainVariables, chainTitle);
    }
    return checkConditionIsSatisfied(condition, state);
}

function checkVariableConditionIsSatisfied(condition: VariableCondition, variables: VariableStore, chainTitle: string): boolean {
    const actualValue = variables.get(chainTitle, condition.target);

    switch (condition.comparator) {
        case 'eq':
            return actualValue === condition.value;
        case 'not':
            return actualValue !== condition.value;
    }
}

function checkConditionIsSatisfied(condition: Condition, state: GameState): boolean {
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

function determineIfIsVariableCondition(toBeDetermined: Condition | VariableCondition): toBeDetermined is VariableCondition {
    if ((typeof toBeDetermined.value) === 'string') {
      return true
    }
    return false
  }