import type { Condition, StateCondition, VariableCondition } from './model';
import type { GameState } from './state';
import type { VariableStore } from './variable_store';

export function checkIsSatisfied(
	condition: Condition | undefined,
	state: GameState,
	chainTitle: string = '',
): boolean {
	if (!condition) {
		return true;
	}
	if (determineIfIsAllOfCondition(condition)) {
		return (
			condition.allOf.findIndex(
				(c) => !checkIsSatisfied(c, state, chainTitle),
			) === -1
		);
	}
	if (determineIfIsAnyOfCondition(condition)) {
		return (
			condition.anyOf.findIndex((c) =>
				checkIsSatisfied(c, state, chainTitle),
			) !== -1
		);
	}
	if (determineIfIsVariableCondition(condition)) {
		return checkVariableConditionIsSatisfied(
			condition,
			state.chainVariables,
			chainTitle,
		);
	}
	return checkStateConditionIsSatisfied(condition, state);
}

function checkVariableConditionIsSatisfied(
	condition: VariableCondition,
	variables: VariableStore,
	chainTitle: string,
): boolean {
	const actualValue = variables.get(chainTitle, condition.target);

	switch (condition.comparator) {
		case 'eq':
			return actualValue === condition.value;
		case 'not':
			return actualValue !== condition.value;
	}
}

function checkStateConditionIsSatisfied(
	condition: StateCondition,
	state: GameState,
): boolean {
	const actualValue = {
		population: state.population,
		ecology: state.ecology,
		reputation: state.reputation,
		money: state.money,
		time: state.turnCounter,
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
		case 'gt':
			return actualValue > condition.value;
	}
}

export function determineIfIsAllOfCondition(
	toBeDetermined: Condition,
): toBeDetermined is { allOf: Condition[] } {
	if (Object.hasOwn(toBeDetermined, 'allOf')) {
		return true;
	}
	return false;
}

export function determineIfIsAnyOfCondition(
	toBeDetermined: Condition,
): toBeDetermined is { anyOf: Condition[] } {
	if (Object.hasOwn(toBeDetermined, 'anyOf')) {
		return true;
	}
	return false;
}

export function determineIfIsVariableCondition(
	toBeDetermined: Condition,
): toBeDetermined is VariableCondition {
	if (
		Object.hasOwn(toBeDetermined, 'value') &&
		typeof (toBeDetermined as { value: unknown }).value === 'string'
	) {
		return true;
	}
	return false;
}
