import {
	type Chain,
	type ChainEvent,
	type Effect,
	isStateVariable,
	parseStateVariable,
	type StateCondition,
	type StateVariable,
} from './model';
import {
	addNamespaceToIdentifier,
	addNamespaceToKeys,
	setNamespaceInEvent,
} from './namespace';
import { validate as validateJsonSchema } from 'jsonschema';

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
			const jsonChain = { ...chainFiles[chainFile] } as JSONChain;
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			delete (jsonChain as any).default;

			const validationRes = validateJsonSchema(jsonChain, getJsonSchema());
			if (!validationRes.valid)
				throw new Error(
					`invalid chain: ${chainFile}, json schema error:${JSON.stringify(validationRes.errors)}`,
				);

			this.chains[jsonChain.title] = {
				...jsonChain,
				cooldown: jsonChain.cooldown || 0,
				autoSelect: jsonChain.autoSelect || false,
				usedVariables: getUsedVariablesIn(jsonChain),
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

function getJsonSchema() {
	const foundFiles = Object.values(
		import.meta.glob(['/chains/schema.json'], { eager: true }),
	);
	if (foundFiles.length === 0) {
		throw new Error('json schema missing');
	} else if (foundFiles.length > 1) {
		throw new Error('there was multiple json schema');
	}
	return foundFiles[0];
}

function getChainsFiles(options?: ConstructorOptions): Record<string, unknown> {
	if (options && options.overrideInputChains) {
		return options.overrideInputChains;
	}
	if (import.meta.env.VITE_ENABLE_TEST_ENV === 'true') {
		const urlParams = new URLSearchParams(window.location.search);
		const overrideInputChainFilenames = urlParams
			.getAll('overrideInputChainFilenames')
			.map((name) => `/test/chains/${name}.json`);
		const files: Record<string, unknown> = import.meta.glob(
			['/test/chains/*.json', '!**/schema.json'],
			{ eager: true },
		);

		return Object.fromEntries(
			Object.entries(files).filter(
				([fileName]) => overrideInputChainFilenames.indexOf(fileName) >= 0,
			),
		);
	}

	return import.meta.glob(['/chains/*.json', '!**/schema.json'], {
		eager: true,
	});
}

export type JSONChain = {
	title: string;
	cooldown?: number;
	trigger?: StateCondition;
	autoSelect?: boolean;
	events: {
		start: ChainEvent;
		[key: string]: ChainEvent;
	};
	effects?: {
		[key: string]: Effect;
	};
};

type ConstructorOptions = {
	overrideInputChains?: Record<string, unknown>;
};

export function getUsedVariablesIn(jsonChain: JSONChain): StateVariable[] {
	const keyToFind = 'target';
	const seen = new Set<StateVariable>();

	function recurse(jsonTree: unknown) {
		if (Array.isArray(jsonTree)) {
			for (const item of jsonTree) recurse(item);
		} else if (jsonTree && typeof jsonTree === 'object') {
			for (const [key, val] of Object.entries(jsonTree as Record<string, unknown>)) {
				if (
					key === keyToFind &&
					typeof val === 'string' &&
					isStateVariable(val)
				) {
					seen.add(parseStateVariable(String(val)));
				}
				recurse(val);
			}
		}
	}

	recurse(jsonChain);
	return Array.from(seen);
}
