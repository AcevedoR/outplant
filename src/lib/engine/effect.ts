import type { Effect } from './model';
import type { GameState } from './state';

export function applyEffectTo(effect: Effect, state: GameState) {
	const difference =
		effect.operation === 'subtract' ? -effect.value : effect.value;

	switch (effect.target) {
		case 'population':
			state.changePopulationBy(difference);
			break;
		case 'ecology':
			state.changeEcologyBy(difference);
			break;
		case 'money':
			state.changeMoneyBy(difference);
			break;
	}
}
