import {ChainStore} from "./chain_store";
import {checkIsSatisfied} from "./condition";
import {applyEffectTo} from "./effect";
import type {Effect, Outcome} from "./model";
import {addNamespaceToIdentifier, extractNamespace} from "./namespace";
import {RNG} from "./rng";
import {GameState} from "./state";
import {translate} from "./translator";

const DEFAULT_LOCALE = "en_US";

export type GameInfos = OngoingGameInfos | EndOfGameInfos;
export type EndOfGameInfos = {
    isVictory: boolean;
};
export type OngoingGameInfos = {
    linesByChain: { [key: string]: Array<string> };
    lastChain?: string;
    choices?: Array<ViewChoice>;
    stateInformations?: StateInformations;
}
export const isEndOfGameInfos = (gameInfos: GameInfos): gameInfos is EndOfGameInfos =>
    'isVictory' in gameInfos;
export const isOngoingGameInfos = (gameInfos: GameInfos): gameInfos is OngoingGameInfos =>
    'linesByChain' in gameInfos;


export type StateInformations = {
    populationChange: number,
    ecologyChange: number,
    moneyChange: number,

    // TODO we need to differenciate permanent effects (populationChange)
    // from instant effects
    // we could return the instant effects as an array of Changes
    // and add a UI Feature to display these changes as Popups
}

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
    private ongoingPermanentEffects: Record<string, Effect> = {};
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

    public nextCycle(): GameInfos {
        if (this.eventsToResolveThisTurn.length !== 0) {
            throw new Error('nextCycle called when some events of current turn were waiting to be resolved');
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
        const ongoingEffectsToApply = Object.keys(this.ongoingPermanentEffects)
            .filter(effectId => !this.justAppliedPermanentEffects.includes(effectId))
            .map(effectId => this.chainStore.getEffect(effectId));
        ongoingEffectsToApply.forEach(effect => applyEffectTo(effect, this.state));

        // Queue new chains
        this.eventsToResolveThisTurn.push(...this.selectChains());

        // Queue ongoing chains
        this.eventsToResolveThisTurn.push(...this.eventsToResolveLater.reverse())
        this.eventsToResolveLater = [];

        return this.unstackEventsToResolveThisTurn();
    }

    public makeChoice(index: number): GameInfos {
        if (this.eventsToResolveThisTurn.length === 0) {
            throw new Error('makeChoice called when no events of current turn were waiting to be resolved');
        }

        this.justAppliedPermanentEffects = [];

        const eventWithChoiceName = this.eventsToResolveThisTurn.pop()!.event;
        const eventWithChoice = this.chainStore.getEvent(eventWithChoiceName);
        const choice = (eventWithChoice.choices)![index];

        const outcome = this.selectNextEvent(choice.next);

        if (choice.effects) {
            this.applyEffects(choice.effects);
        }
        if (outcome.effects) {
            this.applyEffects(outcome.effects);
        }

        this.eventsToResolveThisTurn.push({
            event: outcome.event,
            timer: outcome.in || 0,
        });

        return this.unstackEventsToResolveThisTurn();
    }

    selectChains(): Array<OngoingEventChain> {
        return this.chainStore.getChains()
            .filter(chain => checkIsSatisfied(chain.trigger, this.state)) // filter out chains with unsatisfied trigger
            .filter(chain => !this.coolingDownChains[chain.title]) // filter out chains that are cooling down
            .filter(chain => !this.eventsToResolveLater.find(event => event.event.startsWith(chain.title))) // filter out ongoing chains
            .filter(chain => chain.autoSelect || this.rng.selectOption({value: true, weight: 40}, {
                value: false,
                weight: 60
            })) // chains without autoselect have a 40% chance of being selected
            .map(chain => ({
                event: addNamespaceToIdentifier("start", chain.title),
                timer: 0,
            }));
    }

    unstackEventsToResolveThisTurn(): GameInfos {
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
                if (this.hasWon()) {
                    return { isVictory: true };
                } else if (this.hasLost()) {
                    return { isVictory: false };
                }

                return {
                    linesByChain: eventsByChain,
                    lastChain: chainTitle,
                    choices: event.choices
                        .map(choice => ({
                            text: translate(choice.text, DEFAULT_LOCALE),
                            hidden: !checkIsSatisfied(choice.if, this.state),
                        })),
                    stateInformations: this.createStateInformations(),
                }
            }

            if (event.next && event.next.length > 0) {
                const next = this.selectNextEvent(event.next);
                const nextEvent = this.chainStore.getEvent(next.event);

                if (nextEvent.effects) {
                    this.applyEffects(nextEvent.effects);
                }

                this.eventsToResolveThisTurn.push({
                    timer: next.in || 0,
                    event: next.event,
                });
            } else {
                // We've reached the end of the chain
                this.coolingDownChains[chainTitle] = this.chainStore.getChain(chainTitle).cooldown;
            }
        }

        // We resolved every event that could be during this turn
        // Test for win and lose conditions
        if (this.hasWon()) {
            return { isVictory: true };
        } else if (this.hasLost()) {
            return { isVictory: false };
        }

        return {
            linesByChain: eventsByChain,
            stateInformations: this.createStateInformations(),
        };
    }

    applyEffects(effects: { [key: string]: boolean }) {
        const effectActivations = Object.keys(effects)
            .filter(effectName => effects[effectName])
            .map(effectName => ({
                name: effectName,
                ...this.chainStore.getEffect(effectName),
            }));

        const activatedInstantEffects = effectActivations
            .filter(effect => effect.type === 'instant');
        activatedInstantEffects.forEach(effect => applyEffectTo(effect, this.state));

        const newlyActivatedPermanentEffects = effectActivations
            .filter(effect => effect.type === 'permanent')
            .filter(effect => !Object.keys(this.ongoingPermanentEffects).find(effectId => effectId === effect.name));

        newlyActivatedPermanentEffects.forEach(effect => {
            applyEffectTo(effect, this.state);
            this.ongoingPermanentEffects[effect.name] = effect;
            this.justAppliedPermanentEffects.push(effect.name);
        });

        const effectDeactivations = Object.keys(effects)
            .filter(effectName => !effects[effectName]);
        effectDeactivations.forEach(effectName => delete this.ongoingPermanentEffects[effectName]);
    }

    hasLost(): boolean {
        return this.state.money < 0 || this.state.population === 0;
    }

    hasWon(): boolean {
        return this.state.population === 10;
    }

    selectNextEvent(outcomes: Array<Outcome>): Outcome {
        if (outcomes.length === 0) {
            throw new Error("cannot selectNextEvent when there is none to select");
        }
        return this.rng.selectOption(...outcomes
            .filter(outcome => checkIsSatisfied(outcome.if, this.state))
            .map(outcome => ({
                value: outcome,
                weight: outcome.weight,
            }))
        );
    }

    createStateInformations(): StateInformations {
        let populationChange = 1;
        let moneyChange = 0;
        let ecologyChange = 0;

        for (const [effectId, effect] of Object.entries(this.ongoingPermanentEffects)) {
            const difference = effect.operation === 'subtract' ? -effect.value : effect.value;
            switch (effect.target) {
                case "population":
                    populationChange += difference;
                    break;
                case "ecology":
                    ecologyChange += difference;
                    break;
                case "money":
                    moneyChange += difference;
                    break;
            }
        }

        return {
            ecologyChange,
            populationChange,
            moneyChange
        }
    }
}
