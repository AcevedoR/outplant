import {describe, expect, expectTypeOf, it} from 'vitest'
import {Engine, ViewModel} from "./engine";
import * as fs from "fs";
import path from "path";
import {ChainStore} from "./chain_store";


describe('engine test', () => {
    it('population should grow if no event', () => {
        const engine = new Engine(new ChainStore({overrideInputChains: getChainFromDisk('never_triggers.json')}));
        expect(engine.state.population, 'starting population should be').equal(1);

        for (let i = 0; i < 6; i++) {
            engine.nextCycle();
        }

        expect(engine.state.population).equal(7);
    })
    it('should return population growth to the user', () => {
        const engine = new Engine(new ChainStore({overrideInputChains: getChainFromDisk('never_triggers.json')}));

        const res = engine.nextCycle();

        expect(res).toHaveProperty('stateInformations');
        expect(res.stateInformations.populationGrowth, 'default population growth should be').equal(1);
    })

    it('a simple autoselect chain should always resolve', () => {
        const engine = new Engine(new ChainStore({overrideInputChains: getChainFromDisk('a_simple_empty_chain.json')}));

        const firstTurn = engine.nextCycle();

        expect(firstTurn).to.deep.equal({linesByChain: {'simple empty chain': ["Hello world!"]}});
    })
})

function getChainFromDisk(chainFileName: string): Record<string, any> {
    let res: Record<string, any> = {};
    res[chainFileName] = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../../../test/chains/' + chainFileName), {
        encoding: 'utf8',
        flag: 'r'
    }));
    return res;
}