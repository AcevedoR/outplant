import { describe, expect, it } from 'vitest';
import * as fs from 'fs';
import path from 'path';
import { getUsedVariablesIn, type JSONChain } from './chain_store';

describe('getUsedVariablesIn', () => {
	it('should return none when no ecology', () => {
		const chain = getChainFromDisk(
			'chain_without_ecology_dependency.json',
		) as JSONChain;
		const res = getUsedVariablesIn(chain);
		expect(res).not.contain('ecology');
	});

	it('return should include ecology when present in triggers', () => {
		const chain = getChainFromDisk(
			'chain_without_ecology_dependency.json',
		) as JSONChain;
		chain.trigger = {
			comparator: 'eq',
			value: 1,
			target: 'ecology',
		};
		const res = getUsedVariablesIn(chain);
		expect(res).contain('ecology');
	});

	it('return should include ecology when present in choices', () => {
		const chain = getChainFromDisk(
			'chain_without_ecology_dependency.json',
		) as JSONChain;
		chain.events.start.choices = [
			{
				text: '',
				if: {
					comparator: 'eq',
					value: 1,
					target: 'ecology',
				},
				next: [],
			},
		];
		const res = getUsedVariablesIn(chain);
		expect(res).contain('ecology');
	});
	it('return should include ecology when present in effects', () => {
		const chain = getChainFromDisk(
			'chain_without_ecology_dependency.json',
		) as JSONChain;
		chain.effects = {
			my_effect: {
				type: 'instant',
				operation: 'add',
				value: 1,
				target: 'ecology',
			},
		};
		const res = getUsedVariablesIn(chain);
		expect(res).contain('ecology');
	});
});

function getChainFromDisk(chainFileName: string): JSONChain {
	const res: Record<string, unknown> = {};
	res[chainFileName] = JSON.parse(
		fs.readFileSync(
			path.resolve(__dirname, '../../../test/chains/' + chainFileName),
			{
				encoding: 'utf8',
				flag: 'r',
			},
		),
	);
	return res[chainFileName] as JSONChain;
}
