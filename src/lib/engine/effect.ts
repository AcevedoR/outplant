import type { Effect } from "./model";
import type { GameState } from "./state";


// TODO rework this to keep the change information,
// either by creation a stack of effects to apply, and refactoring the core code
// or just logging effects to a stack
export function applyEffectTo(effect: Effect, state: GameState) {
    const difference = effect.operation === 'subtract' ?
        -effect.value :
        effect.value;

    switch (effect.target) {
        case "population":
            state.changePopulationBy(difference);
            break;
        case "ecology":
            state.changeEcologyBy(difference);
            break;
        case "money":
            state.changeMoneyBy(difference);
            break;
    }
}
