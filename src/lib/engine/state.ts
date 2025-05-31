import { VariableStore } from "./variable_store";

export class GameState {
    private _population: number;
    private _ecology: UnlockableVariable;
    private _money: number;
    private _turnCounter: number;

    readonly chainVariables: VariableStore;

    constructor(population = 1, ecology = 10, money = 1000, turnCounter = 0) {
        this._population = population;
        this._ecology = new UnlockableVariable(ecology);
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

    isEcologyUnlocked(): boolean {
        return this._ecology.unlocked;
    }

    get money(): number {
        return this._money;
    }

    get turnCounter(): number {
        return this._turnCounter;
    }

    changePopulationBy(difference: number) {
        this._population += difference;
        if (this._population > 10) {
            this._population = 10;
        } else if (this._population < 0) {
            this._population = 0;
        }
    }

    changeEcologyBy(difference: number) {
        if(!this._ecology.unlocked){
            throw Error(`cannot changeEcologyBy when not unlocked`);
        }
        this._ecology.value += difference;
        if (this._ecology.value > 10) {
            this._ecology.value = 10;
        } else if (this._ecology.value < 0) {
            this._ecology.value = 0;
        }
    }

    changeMoneyBy(difference: number) {
        this._money += difference;
    }

    nextTurn() {
        this._turnCounter++;
        console.log(this._ecology)
        if(this.turnCounter === 10){
            this.unlockEcology();
        }

        this.changePopulationBy(1);
    }

    unlockEcology() {
        this._ecology.unlocked = true;
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