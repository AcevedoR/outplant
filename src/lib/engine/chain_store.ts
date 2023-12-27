import type { Chain, ChainEvent, Effect, StateVariable } from "./model";
import { addNamespaceToIdentifier, addNamespaceToKeys, setNamespaceInEvent } from "./namespace";
import { Trigger, type Comparator } from "./trigger";

const chainFiles = import.meta.glob(["/chains/*.json", "!**/schema.json"], {eager: true});

const defaultStore: ChainStore = {
    chains: {},
    events: {},
    effects: {},
};

export function getChain(title: string): Chain {
    return defaultStore.chains[title];
}

export function getChains(): Array<Chain> {
    return Object.values(defaultStore.chains);
}

export function getEvent(eventName: string): ChainEvent {
    return defaultStore.events[eventName];
}

export function getEffect(effectName: string): Effect {
    return defaultStore.effects[effectName];
}

function initStore() {
    for (const chainFile in chainFiles) {
        const jsonChain = chainFiles[chainFile] as JSONChain;

        const trigger = jsonChain.trigger ?
            new Trigger(jsonChain.trigger.comparator, jsonChain.trigger.target, jsonChain.trigger.value) :
            undefined;

        defaultStore.chains[jsonChain.title] = {
            ...jsonChain,
            trigger,
            cooldown: jsonChain.cooldown || 0,
            autoSelect: jsonChain.autoSelect || false,
        };

        for (const eventId in jsonChain.events) {
            const event = jsonChain.events[eventId];
            setNamespaceInEvent(event, jsonChain.title);
            defaultStore.events[addNamespaceToIdentifier(eventId, jsonChain.title)] = event;
        }

        if (jsonChain.effects) {
            addNamespaceToKeys(jsonChain.effects, jsonChain.title);
            defaultStore.effects = {
                ...defaultStore.effects,
                ...jsonChain.effects,
            };
        }
    }
}

type ChainStore = {
    chains: { [key: string]: Chain };
    events: { [key: string]: ChainEvent };
    effects: { [key: string]: Effect };
};

type JSONChain = {
    title: string;
    cooldown?: number;
    trigger?: JSONTrigger;
    autoSelect?: boolean;
    events: {
        start: ChainEvent;
        [key: string]: ChainEvent;
    };
    effects?: {
        [key: string]: Effect;
    }
};

type JSONTrigger = {
    comparator: Comparator;
    target: StateVariable;
    value: number;
};

initStore();
