import { ChainStore } from './chain_store';
import { checkIsSatisfied } from './condition';
import { applyEffectTo } from './effect';
import type { Chain, Outcome } from './model';
import { addNamespaceToIdentifier, extractNamespace } from './namespace';
import { RNG } from './rng';
import { GameState } from './state';
import { translate } from './translator';

const DEFAULT_LOCALE = 'en_US';

export type ViewModel =
	| {
			linesByChain: { [key: string]: Array<string> };
			lastChain?: string;
			choices?: Array<ViewChoice>;
	  }
	| {
			isVictory: boolean;
	  };

export type ViewChoice = {
	text: string;
	hidden: boolean;
};

type OngoingEventChain = {
	timer: number;
	event: string;
};

export class Engine {
	readonly state: GameState = new GameState();
	private chainStore: ChainStore;
	private eventsToResolveThisTurn: Array<OngoingEventChain> = [];
	private eventsToResolveLater: Array<OngoingEventChain> = [];
	private ongoingPermanentEffects: Set<string> = new Set<string>();
	private justAppliedPermanentEffects: Array<string> = [];
	private readonly rng: RNG = new RNG();
	private coolingDownChains: { [key: string]: number } = {};

	constructor(chainStore?: ChainStore) {
		if (chainStore) {
			this.chainStore = chainStore;
		} else {
			this.chainStore = new ChainStore();
		}
	}

	public nextCycle(): ViewModel {
		if (this.eventsToResolveThisTurn.length !== 0) {
			throw new Error(
				'nextCycle called when some events of current turn were waiting to be resolved',
			);
		}

		// Apply chain cooldowns
		for (const chainTitle in this.coolingDownChains) {
			if (this.coolingDownChains[chainTitle] === 0) {
				delete this.coolingDownChains[chainTitle];
			} else {
				this.coolingDownChains[chainTitle]--;
			}
		}

		// Apply "natural" effects
		this.state.nextTurn();

		// Apply ongoing effects
		const ongoingEffectsToApply = Array.from(this.ongoingPermanentEffects)
			.filter(
				(effectId) => !this.justAppliedPermanentEffects.includes(effectId),
			)
			.map((effectId) => this.chainStore.getEffect(effectId));
		ongoingEffectsToApply.forEach((effect) =>
			applyEffectTo(effect, this.state),
		);

		// Queue new chains
		this.eventsToResolveThisTurn.push(...this.selectChains());

		// Queue ongoing chains
		this.eventsToResolveThisTurn.push(...this.eventsToResolveLater.reverse());
		this.eventsToResolveLater = [];

		return this.unstackEventsToResolveThisTurn();
	}

	public makeChoice(index: number): ViewModel {
		if (this.eventsToResolveThisTurn.length === 0) {
			throw new Error(
				'makeChoice called when no events of current turn were waiting to be resolved',
			);
		}

		this.justAppliedPermanentEffects = [];

		const eventWithChoiceName = this.eventsToResolveThisTurn.pop()!.event;
		const eventWithChoice = this.chainStore.getEvent(eventWithChoiceName);
		const choice = eventWithChoice.choices![index];

		const outcome = this.selectNextEvent(choice.next);

		if (choice.effects) {
			this.applyEffects(choice.effects);
		}
		if (outcome.effects) {
			this.applyEffects(outcome.effects);
		}
		if (outcome.variables) {
			for (const variable in outcome.variables) {
				this.state.chainVariables.set(
					extractNamespace(eventWithChoiceName),
					variable,
					outcome.variables[variable],
				);
			}
		}

		this.eventsToResolveThisTurn.push({
			event: outcome.event,
			timer: outcome.in || 0,
		});

		return this.unstackEventsToResolveThisTurn();
	}

	selectChains(): Array<OngoingEventChain> {
		const possibleCandidateChains: Chain[] = this.chainStore
			.getChains()
			.filter((chain) => checkIsSatisfied(chain.trigger, this.state)) // filter out chains with unsatisfied trigger
			.filter((chain) => !this.coolingDownChains[chain.title]) // filter out chains that are cooling down
			.filter(
				(chain) =>
					!this.eventsToResolveLater.find((event) =>
						event.event.startsWith(chain.title),
					),
			) // filter out ongoing chains
			.filter(
				(chain) =>
					!chain.usedVariables.some(
						(v) => !this.state.getUnlockedVariables().includes(v),
					),
			);

		const autoSelectChains = possibleCandidateChains.filter(
			(chain) => chain.autoSelect,
		);

		const targetNumberOfChains = this.state.turnCounter <= 10 ? 1 : 2;

		const numberOfRandomChainsToSelect =
			targetNumberOfChains -
			(this.eventsToResolveLater.length + autoSelectChains.length);

		const selectedChains = autoSelectChains;
		if (numberOfRandomChainsToSelect > 0) {
			const possibleNonAutoSelectChains = possibleCandidateChains.filter(
				(chain) => !chain.autoSelect,
			);
			const chainsToAdd = this.rng.selectRandomlyFromArray(
				possibleNonAutoSelectChains,
				numberOfRandomChainsToSelect,
			);
			selectedChains.push(...chainsToAdd);
		}

		return selectedChains.map((chain) => ({
			event: addNamespaceToIdentifier('start', chain.title),
			timer: 0,
		}));
	}

	unstackEventsToResolveThisTurn(): ViewModel {
		const eventsByChain: { [key: string]: Array<string> } = {};

		while (this.eventsToResolveThisTurn.length !== 0) {
			const eventToPlay = this.eventsToResolveThisTurn.pop()!;
			if (eventToPlay.timer !== 0) {
				// The event is scheduled for later
				eventToPlay.timer--;
				this.eventsToResolveLater.push(eventToPlay);
				continue;
			}

			const event = this.chainStore.getEvent(eventToPlay.event);
			const chainTitle = extractNamespace(eventToPlay.event);

			if (event.text) {
				eventsByChain[chainTitle] = eventsByChain[chainTitle] || [];
				eventsByChain[chainTitle].push(translate(event.text, DEFAULT_LOCALE));
			}

			// Apply effects, if any
			if (event.effects) {
				this.applyEffects(event.effects);
			}

			if (event.choices) {
				// We encountered a choice the player has to make
				this.eventsToResolveThisTurn.push(eventToPlay);

				// Test for win and lose conditions
				if (this.state.hasWon()) {
					return { isVictory: true };
				} else if (this.state.hasLost()) {
					return { isVictory: false };
				}

				return {
					linesByChain: eventsByChain,
					lastChain: chainTitle,
					choices: event.choices.map((choice) => ({
						text: translate(choice.text, DEFAULT_LOCALE),
						hidden: !checkIsSatisfied(choice.if, this.state, chainTitle),
					})),
				};
			}

			if (event.next && event.next.length > 0) {
				const next = this.selectNextEvent(event.next);
				const nextEvent = this.chainStore.getEvent(next.event);

				if (nextEvent.effects) {
					this.applyEffects(nextEvent.effects);
				}
				if (next.variables) {
					for (const variable in next.variables) {
						this.state.chainVariables.set(
							chainTitle,
							variable,
							next.variables[variable],
						);
					}
				}

				this.eventsToResolveThisTurn.push({
					timer: next.in || 0,
					event: next.event,
				});
			} else {
				// We've reached the end of the chain
				this.coolingDownChains[chainTitle] =
					this.chainStore.getChain(chainTitle).cooldown;
				this.state.chainVariables.clearNamespace(chainTitle);
			}
		}

		// We resolved every event that could be during this turn
		// Test for win and lose conditions
		if (this.state.hasWon()) {
			return { isVictory: true };
		} else if (this.state.hasLost()) {
			return { isVictory: false };
		}

		return { linesByChain: eventsByChain };
	}

	applyEffects(effects: { [key: string]: boolean }) {
		const effectActivations = Object.keys(effects)
			.filter((effectName) => effects[effectName])
			.map((effectName) => ({
				name: effectName,
				...this.chainStore.getEffect(effectName),
			}));

		const activatedInstantEffects = effectActivations.filter(
			(effect) => effect.type === 'instant',
		);
		activatedInstantEffects.forEach((effect) =>
			applyEffectTo(effect, this.state),
		);

		const newlyActivatedPermanentEffects = effectActivations
			.filter((effect) => effect.type === 'permanent')
			.filter((effect) => !this.ongoingPermanentEffects.has(effect.name));

		newlyActivatedPermanentEffects.forEach((effect) => {
			applyEffectTo(effect, this.state);
			this.ongoingPermanentEffects.add(effect.name);
			this.justAppliedPermanentEffects.push(effect.name);
		});

		const effectDeactivations = Object.keys(effects).filter(
			(effectName) => !effects[effectName],
		);
		effectDeactivations.forEach((effectName) =>
			this.ongoingPermanentEffects.delete(effectName),
		);
	}

	selectNextEvent(outcomes: Array<Outcome>): Outcome {
		if (outcomes.length === 0) {
			throw new Error('cannot selectNextEvent when there is none to select');
		}
		return this.rng.selectOption(
			...outcomes
				.filter((outcome) =>
					checkIsSatisfied(
						outcome.if,
						this.state,
						extractNamespace(outcome.event),
					),
				)
				.map((outcome) => ({
					value: outcome,
					weight: outcome.weight,
				})),
		);
	}
}
