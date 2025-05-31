import {describe, expect, it} from 'vitest'
import * as fs from "fs";
import path from "path";
import {getUsedVariablesIn, getUsedVariablesInCondition, type JSONChain} from "./chain_store";
import type {Condition, StateCondition, VariableCondition} from "./model";

describe('getUsedVariablesIn', () => {

    it('should return none when no ecology', () => {
        const chain = getChainFromDisk('chain_without_ecology_dependency.json') as JSONChain;
        const res = getUsedVariablesIn(chain);
        expect(res).not.contain("ecology");
    })

    it('return should include ecology when present in triggers', () => {
        const chain = getChainFromDisk('chain_without_ecology_dependency.json') as JSONChain;
        chain.trigger = {
            comparator: "eq",
            value: 1,
            target: "ecology",
        };
        const res = getUsedVariablesIn(chain);
        expect(res).contain("ecology");
    })

    it('return should include ecology when present in choices', () => {
        const chain = getChainFromDisk('chain_without_ecology_dependency.json') as JSONChain;
        chain.events.start.choices= [{
           text: "",
           if: {
               comparator: "eq",
               value: 1,
               target: "ecology",
           },
            next: []
        }];
        const res = getUsedVariablesIn(chain);
        expect(res).contain("ecology");
    })
})
describe('getUsedVariablesInCondition', () => {
    it('return should return none when no ecology', () => {
        const condition: Condition = {
            comparator: "eq",
            value: 1,
            target: "money",
        };
        const res = getUsedVariablesInCondition(condition);
        expect(res).not.contain("ecology");
    })
    it('StateCondition with ecology should return ecology', () => {
        const condition: StateCondition = {
            comparator: "eq",
            value: 1,
            target: "ecology",
        };
        const res = getUsedVariablesInCondition(condition);
        expect(res).contain("ecology");
    })
    it('All of should return none', () => {
        const condition: { allOf: Condition[] } = {
            allOf: [{
                comparator: "eq",
                value: 1,
                target: "money",
            }]
        };
        const res = getUsedVariablesInCondition(condition);
        expect(res).not.contain("ecology");
    })
    it('All of with ecology should return ecology', () => {
        const condition: { allOf: Condition[] } = {
            allOf: [{
                comparator: "eq",
                value: 1,
                target: "ecology",
            }]
        };
        const res = getUsedVariablesInCondition(condition);
        expect(res).contain("ecology");
    })
    it('Any of should return none', () => {
        const condition: { anyOf: Condition[] } = {
            anyOf: [{
                comparator: "eq",
                value: 1,
                target: "money",
            }]
        };
        const res = getUsedVariablesInCondition(condition);
        expect(res).not.contain("ecology");
    })
    it('Any of with ecology should return ecology', () => {
        const condition: { anyOf: Condition[] } = {
            anyOf: [{
                comparator: "eq",
                value: 1,
                target: "ecology",
            }]
        };
        const res = getUsedVariablesInCondition(condition);
        expect(res).contain("ecology");
    })
    it('VariableCondition should always return nothing', () => {
        const condition: VariableCondition = {
                comparator: "eq",
                value: "lmystrung",
                target: "ecology",
        };
        const res = getUsedVariablesInCondition(condition);
        expect(res).not.contain("ecology");
    })
})

function getChainFromDisk(chainFileName: string): JSONChain {
    let res: Record<string, any> = {};
    res[chainFileName] = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../../../test/chains/' + chainFileName), {
        encoding: 'utf8',
        flag: 'r'
    }));
    return res[chainFileName] as JSONChain;
}