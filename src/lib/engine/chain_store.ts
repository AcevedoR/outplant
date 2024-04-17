import type {Chain, ChainEvent, Effect, StateVariable} from "./model";
import {type Comparator, Trigger} from "./trigger";
import {addNamespaceToIdentifier, addNamespaceToKeys, setNamespaceInEvent} from "./namespace";


export class ChainStore {
    readonly chains: { [key: string]: Chain } = {};
    events: { [key: string]: ChainEvent } = {};
    effects: { [key: string]: Effect } = {};


    public getChain(title: string): Chain {
        return this.chains[title];
    }

    public getChains(): Array<Chain> {
        return Object.values(this.chains);
    }

    public getEvent(eventName: string): ChainEvent {
        return this.events[eventName];
    }

    public getEffect(effectName: string): Effect {
        return this.effects[effectName];
    }

    constructor(options?: ConstructorOptions) {
        const chainFiles = getChainsFiles(options);
        for (const chainFile in chainFiles) {
            const jsonChain = chainFiles[chainFile] as JSONChain;

            const trigger = jsonChain.trigger ?
                new Trigger(jsonChain.trigger.comparator, jsonChain.trigger.target, jsonChain.trigger.value) :
                undefined;

            this.chains[jsonChain.title] = {
                ...jsonChain,
                trigger,
                cooldown: jsonChain.cooldown || 0,
                autoSelect: jsonChain.autoSelect || false,
            };

            for (const eventId in jsonChain.events) {
                const event = jsonChain.events[eventId];
                setNamespaceInEvent(event, jsonChain.title);
                this.events[addNamespaceToIdentifier(eventId, jsonChain.title)] = event;
            }

            if (jsonChain.effects) {
                addNamespaceToKeys(jsonChain.effects, jsonChain.title);
                this.effects = {
                    ...this.effects,
                    ...jsonChain.effects,
                };
            }
        }
    }
}

function getChainsFiles(options?: ConstructorOptions): Record<string, any> {
    if (options && options.overrideInputChains) {
        return options.overrideInputChains;
    }
    return import.meta.glob(["/chains/*.json", "!**/schema.json"], {eager: true});
}

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

type ConstructorOptions = {
    overrideInputChains?: Record<string, any>
}