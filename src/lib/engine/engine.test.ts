import { describe, expect, it } from 'vitest';
import { Engine } from './engine';
import * as fs from 'fs';
import path from 'path';
import { ChainStore } from './chain_store';

describe('engine test', () => {
	it('population should grow if no event', () => {
		const engine = new Engine(
			new ChainStore({
				overrideInputChains: getChainFromDisk('never_triggers.json'),
			}),
		);

		for (let i = 0; i < 6; i++) {
			engine.nextCycle();
		}

		expect(engine.state.population).equal(7);
	});

	it('a simple autoselect chain should always resolve', () => {
		const engine = new Engine(
			new ChainStore({
				overrideInputChains: getChainFromDisk('a_simple_empty_chain.json'),
			}),
		);

		const firstTurn = engine.nextCycle();

		expect(firstTurn).to.deep.equal({
			linesByChain: { 'simple empty chain': ['Hello world!'] },
		});
	});

	it('a simple autoselect chain should always resolve', () => {
		const engine = new Engine(
			new ChainStore({
				overrideInputChains: getChainFromDisk('a_simple_empty_chain.json'),
			}),
		);

		const firstTurn = engine.nextCycle();

		expect(firstTurn).to.deep.equal({
			linesByChain: { 'simple empty chain': ['Hello world!'] },
		});
	});

	it('a chain depending on ecology should not be available before ecology is unlocked', () => {
		const engine = new Engine(
			new ChainStore({
				overrideInputChains: getChainFromDisk(
					'chain_with_ecology_dependency.json',
				),
			}),
		);

		const firstTurn = engine.nextCycle();

		expect(engine.state.getUnlockedVariables()).not.contain('ecology');
		expect(firstTurn).to.deep.equal({ linesByChain: {} });
	});
	it('a chain depending on population should be available because population is always unlocked', () => {
		const engine = new Engine(
			new ChainStore({
				overrideInputChains: getChainFromDisk(
					'chain_with_population_dependency.json',
				),
			}),
		);

		const firstTurn = engine.nextCycle();

		expect(engine.state.getUnlockedVariables()).contain('population');
		expect(firstTurn).to.deep.equal({
			linesByChain: { 'simple empty chain': ['Hello world!'] },
		});
	});
});

function getChainFromDisk(chainFileName: string): Record<string, unknown> {
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
	return res;
}
