export class GameState {
    private _population: number;
    private _ecology: number;
    private _money: number;
    private _turnCounter: number;

    constructor(population = 1, ecology = 10, money = 1000, turnCounter = 0) {
        this._population = population;
        this._ecology = ecology;
        this._money = money;
        this._turnCounter = turnCounter;
    }

    get population(): number {
        return this._population;
    }

    get ecology(): number {
        return this._ecology;
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
        this._ecology += difference;
        if (this._ecology > 10) {
            this._ecology = 10;
        } else if (this._ecology < 0) {
            this._ecology = 0;
        }
    }

    changeMoneyBy(difference: number) {
        this._money += difference;
    }

    nextTurn() {
        this._turnCounter++;

        this.changePopulationBy(1);
    }
}
