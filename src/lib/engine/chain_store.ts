import type {Chain, ChainEvent, Condition, Effect, StateVariable} from "./model";
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

            this.chains[jsonChain.title] = {
                ...jsonChain,
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
    trigger?: Condition;
    autoSelect?: boolean;
    events: {
        start: ChainEvent;
        [key: string]: ChainEvent;
    };
    effects?: {
        [key: string]: Effect;
    }
};

type ConstructorOptions = {
    overrideInputChains?: Record<string, any>
}