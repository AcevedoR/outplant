import { expect, test, describe, it } from 'vitest';
import {
	addNamespaceToKeys,
	extractNamespace,
	setNamespaceInEvent,
} from './namespace';
import type { ChainEvent } from './model';

describe('setNamespaceInEvent', () => {
	it('should gracefully handle undefined attributes', () => {
		const event: ChainEvent = { text: 'Hello' };

		setNamespaceInEvent(event, 'chain');

		expect(event).toEqual({ text: 'Hello' });
	});

	it('should correctly and deeply set namespace in an event with choices', () => {
		const event: ChainEvent = {
			text: 'Hello',
			choices: [
				{
					text: 'Choice 1',
					next: [
						{
							event: 'choice_1_outcome_1',
							weight: 3,
						},
						{
							event: 'choice_1_outcome_2',
							in: 3,
							effects: {
								effect1: true,
							},
						},
					],
				},
				{
					text: 'Choice 2',
					effects: {
						effect2: false,
					},
					next: [
						{
							event: 'choice_2_outcome',
						},
					],
				},
			],
		};

		const namespace = 'chain';

		const namespacedEvent: ChainEvent = {
			text: 'Hello',
			choices: [
				{
					text: 'Choice 1',
					next: [
						{
							event: 'chain/choice_1_outcome_1',
							weight: 3,
						},
						{
							event: 'chain/choice_1_outcome_2',
							in: 3,
							effects: {
								'chain/effect1': true,
							},
						},
					],
				},
				{
					text: 'Choice 2',
					effects: {
						'chain/effect2': false,
					},
					next: [
						{
							event: 'chain/choice_2_outcome',
						},
					],
				},
			],
		};

		setNamespaceInEvent(event, namespace);

		expect(event).toEqual(namespacedEvent);
	});

	it('should correctly and deeply set namespace in an event with outcomes and effects', () => {
		const event: ChainEvent = {
			text: 'Hello',
			next: [
				{
					event: 'outcome_1',
					weight: 3,
				},
				{
					event: 'outcome_2',
					in: 3,
					effects: {
						effect1: true,
					},
				},
			],
			effects: {
				effect2: false,
			},
		};

		const namespace = 'chain';

		const namespacedEvent: ChainEvent = {
			text: 'Hello',
			next: [
				{
					event: 'chain/outcome_1',
					weight: 3,
				},
				{
					event: 'chain/outcome_2',
					in: 3,
					effects: {
						'chain/effect1': true,
					},
				},
			],
			effects: {
				'chain/effect2': false,
			},
		};

		setNamespaceInEvent(event, namespace);

		expect(event).toEqual(namespacedEvent);
	});
});

test('addNamespaceToKeys', () => {
	const object = {
		event1: { text: 'Hello' },
		event2: { text: 'Goodbye' },
	};
	const prefix = 'chain';

	const expectedOutput = {
		'chain/event1': { text: 'Hello' },
		'chain/event2': { text: 'Goodbye' },
	};

	addNamespaceToKeys(object, prefix);

	expect(object).toEqual(expectedOutput);
});

test('extractNamespace', () => {
	expect(extractNamespace('missing_form/form_filled_fine_unpaid')).toEqual(
		'missing_form',
	);
});
