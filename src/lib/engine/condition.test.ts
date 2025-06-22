import { expect, describe, it } from 'vitest';
import { GameState } from './state';
import type { Condition, StateCondition, VariableCondition } from './model';
import { checkIsSatisfied } from './condition';

describe('condition', () => {
	const sampleChainTitle = 'whatever';

	const state: GameState = (() => {
		const state = new GameState();
		state.unlockEcology();
		state.changeEcologyBy(-state.ecology);
		state.changeEcologyBy(5);
		state.changeMoneyBy(-state.money);
		state.changeMoneyBy(10_000);

		state.chainVariables.set(sampleChainTitle, 'plantType', 'ferns');
		state.chainVariables.set(sampleChainTitle, 'monitorBotanists', 'no');

		return state;
	})();

	it('should evaluate a simple true state condition', () => {
		const c: StateCondition = {
			target: 'ecology',
			comparator: 'eq',
			value: 5,
		};
		expect(checkIsSatisfied(c, state, sampleChainTitle)).toBeTruthy();
	});

	it('should evaluate a simple false state condition', () => {
		const c: StateCondition = {
			target: 'ecology',
			comparator: 'gt',
			value: 5,
		};
		expect(checkIsSatisfied(c, state, sampleChainTitle)).toBeFalsy();
	});

	it('should evaluate a simple true variable condition', () => {
		const c: VariableCondition = {
			target: 'plantType',
			comparator: 'eq',
			value: 'ferns',
		};
		expect(checkIsSatisfied(c, state, sampleChainTitle)).toBeTruthy();
	});

	it('should evaluate a simple false variable condition', () => {
		const c: VariableCondition = {
			target: 'plantType',
			comparator: 'not',
			value: 'ferns',
		};
		expect(checkIsSatisfied(c, state, sampleChainTitle)).toBeFalsy();
	});

	it('should evaluate a true "allOf" condition', () => {
		const c: Condition = {
			allOf: [
				{
					target: 'plantType',
					comparator: 'eq',
					value: 'ferns',
				},
				{
					target: 'ecology',
					comparator: 'eq',
					value: 5,
				},
			],
		};
		expect(checkIsSatisfied(c, state, sampleChainTitle)).toBeTruthy();
	});

	it('should evaluate a false "allOf" condition', () => {
		const c: Condition = {
			allOf: [
				{
					target: 'plantType',
					comparator: 'eq',
					value: 'ferns',
				},
				{
					target: 'ecology',
					comparator: 'gt',
					value: 5,
				},
			],
		};
		expect(checkIsSatisfied(c, state, sampleChainTitle)).toBeFalsy();
	});

	it('should evaluate a true "anyOf" condition', () => {
		const c: Condition = {
			anyOf: [
				{
					target: 'plantType',
					comparator: 'not',
					value: 'ferns',
				},
				{
					target: 'ecology',
					comparator: 'eq',
					value: 5,
				},
			],
		};
		expect(checkIsSatisfied(c, state, sampleChainTitle)).toBeTruthy();
	});

	it('should evaluate a false "anyOf" condition', () => {
		const c: Condition = {
			anyOf: [
				{
					target: 'plantType',
					comparator: 'not',
					value: 'ferns',
				},
				{
					target: 'ecology',
					comparator: 'gt',
					value: 5,
				},
			],
		};
		expect(checkIsSatisfied(c, state, sampleChainTitle)).toBeFalsy();
	});

	it('should evaluate a true complex condition', () => {
		const c: Condition = {
			anyOf: [
				{
					target: 'plantType',
					comparator: 'not',
					value: 'ferns',
				},
				{
					allOf: [
						{
							target: 'ecology',
							comparator: 'eq',
							value: 5,
						},
						{
							target: 'money',
							comparator: 'gt',
							value: 4_000,
						},
					],
				},
			],
		};
		expect(checkIsSatisfied(c, state, sampleChainTitle)).toBeTruthy();
	});

	it('should evaluate a false complex condition', () => {
		const c: Condition = {
			allOf: [
				{
					target: 'plantType',
					comparator: 'not',
					value: 'ferns',
				},
				{
					anyOf: [
						{
							target: 'ecology',
							comparator: 'gt',
							value: 5,
						},
						{
							target: 'money',
							comparator: 'gt',
							value: 4_000,
						},
					],
				},
			],
		};
		expect(checkIsSatisfied(c, state, sampleChainTitle)).toBeFalsy();
	});
});
