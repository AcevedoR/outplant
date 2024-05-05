import {describe, expect, expectTypeOf, it} from 'vitest'
import {Engine, isEndOfGameInfos} from "./engine";
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

        if(isEndOfGameInfos(res)){
            throw new Error('type should not be endofgame')
        }
        expect(res.stateInformations).not.empty;
        expect(res.stateInformations?.populationChange, 'default population growth should be').equal(1);
    })

    it('should return decreasing ecology when having a bad permanent effect', () => {
        const engine = new Engine(new ChainStore({overrideInputChains: getChainFromDisk('ecology_permanent_drop.json')}));

        const res = engine.nextCycle();

        if(isEndOfGameInfos(res)){
            throw new Error('type should not be endofgame')
        }
        expect(res.stateInformations).not.empty;
        expect(res.stateInformations?.ecologyChange, 'ecology should be decreasing').equal(-2);
        expect(engine.state.ecology, 'ecology should have decrease by 2').equal(8);
    })

    it('a simple autoselect chain should always resolve', () => {
        const engine = new Engine(new ChainStore({overrideInputChains: getChainFromDisk('a_simple_empty_chain.json')}));

        const firstTurn = engine.nextCycle();

        if(isEndOfGameInfos(firstTurn)){
            throw new Error('type should not be endofgame')
        }
    
        expect(firstTurn.linesByChain).toStrictEqual({'simple empty chain': ["Hello world!"]});
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