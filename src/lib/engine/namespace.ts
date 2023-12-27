import type { ChainEvent, Choice, Outcome } from "./model";

export function setNamespaceInEvent(event: ChainEvent, namespace: string) {
    if (event.effects) {
        addNamespaceToKeys(event.effects, namespace);
    }
    if (event.choices) {
        event.choices.forEach(choice => setNamespaceInChoice(choice, namespace));
    }
    if (event.next) {
        event.next.forEach(outcome => setNamespaceInOutcome(outcome, namespace));
    }
}

function setNamespaceInChoice(choice: Choice, namespace: string) {
    choice.next.forEach(outcome => setNamespaceInOutcome(outcome, namespace));
    if (choice.effects) {
        addNamespaceToKeys(choice.effects, namespace);
    }
}

function setNamespaceInOutcome(outcome: Outcome, namespace: string) {
    outcome.event = `${namespace}/${outcome.event}`;
    if (outcome.effects) {
        addNamespaceToKeys(outcome.effects, namespace);
    }
}

export function addNamespaceToIdentifier(identifier: string, namespace: string): string {
    return `${namespace}/${identifier}`;
}

export function extractNamespace(namespacedIdentifier: string): string {
    return namespacedIdentifier.split("/")[0];
}

export function addNamespaceToKeys(object: {[key: string]: unknown}, namespace: string) {
    for (const key in object) {
        object[addNamespaceToIdentifier(key, namespace)] = object[key];
        delete(object[key]);
    }
}
