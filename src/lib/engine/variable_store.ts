export class VariableStore {
    private _inner: Map<string, Map<string, string>> = new Map();

    set(namespace: string, name: string, value: string) {
        this._inner.set(namespace, this._inner.get(namespace) || new Map());
        this._inner.get(namespace)?.set(name, value);
    }

    get(namespace: string, name: string): string | undefined {
        return this._inner.get(namespace)?.get(name);
    }

    clearNamespace(namespace: string) {
        this._inner.delete(namespace);
    }
}