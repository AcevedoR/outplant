import {VariableStore} from "./variable_store";
import type {StateVariable} from "./model";

const MAX_POPULATION = 10;

export class GameState {
    private _population: number;
    private _ecology: UnlockableVariable;
    private _reputation: UnlockableVariable;
    private _money: number;
    private _turnCounter: number;

    readonly chainVariables: VariableStore;

    constructor(population = 1, ecology = 10, reputation = 0, money = 1000, turnCounter = 0) {
        this._population = population;
        this._ecology = new UnlockableVariable(ecology);
        this._reputation = new UnlockableVariable(reputation);
        this._money = money;
        this._turnCounter = turnCounter;
        this.chainVariables = new VariableStore();
    }

    get population(): number {
        return this._population;
    }

    get ecology(): number {
        return this._ecology.value;
    }

    get reputation(): number {
        return this._reputation.value;
    }

    get money(): number {
        return this._money;
    }

    get turnCounter(): number {
        return this._turnCounter;
    }

    getUnlockedVariables(): StateVariable[] {
        let unlockedVariables: StateVariable[] = ['population', 'money'];
        if (this._ecology.unlocked) {
            unlockedVariables.push('ecology');
        }
        if (this._reputation.unlocked) {
            unlockedVariables.push('reputation');
        }
        return unlockedVariables;
    }

    changePopulationBy(difference: number) {
        this._population += difference;
        this._population = clamp(this._population, 0, MAX_POPULATION);
    }

    changeEcologyBy(difference: number) {
        if (!this._ecology.unlocked) {
            throw Error(`cannot changeEcologyBy when not unlocked`);
        }

        this._ecology.value += difference;
        this._ecology.value = clamp(this._ecology.value, 0, 10);
    }

    changeMoneyBy(difference: number) {
        this._money += difference;
    }

    changeReputationBy(difference: number) {
        if (!this._reputation.unlocked) {
            throw Error(`cannot changeReputationBy when not unlocked`);
        }
        this._reputation.value += difference;
        this._reputation.value = clamp(this._reputation.value, -10, 10);
    }

    nextTurn() {
        this._turnCounter++;
        if (this.turnCounter === 10) {
            this.unlockEcology();
        }
        if (this.turnCounter === 15) {
            this._reputation.unlocked = true;
        }

        this.changePopulationBy(1);
    }

    unlockEcology() {
        this._ecology.unlocked = true;
    }

    hasLost(): boolean {
        return this._money < -400 || this._population === 0 || this._ecology.value === 0;
    }

    hasWon(): boolean {
        return this._population === MAX_POPULATION;
    }

}


class UnlockableVariable {
    private _value: number;
    private _unlocked: boolean;

    constructor(value: number) {
        this._value = value;
        this._unlocked = false;
    }

    get value(): number {
        return this._value;
    }

    set value(value: number) {
        this._value = value;
    }

    get unlocked(): boolean {
        return this._unlocked;
    }

    set unlocked(value: boolean) {
        if(this._unlocked) {
            throw new Error("Unlockable already unlocked");
        }
        this._unlocked = value;
    }
}

function clamp(value: number, min: number = -Infinity, max: number = Infinity): number {
    if (value < min) {
        return min;
    }
    if (value > max) {
        return max;
    }
    return value;
}
