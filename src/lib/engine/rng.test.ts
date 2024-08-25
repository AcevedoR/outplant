import { expect, test, describe, it } from 'vitest';
import { RNG, type RNGOption } from './rng';

test('rng without mock should return relatively random results', () => {
    const rng = new RNG();

    const results: Array<number> = [];
    const options: Array<RNGOption<number>> = [
        {value: 0}, {value: 1}, {value: 2}
    ];

    const numberOfThrows = 1_000;

    for (let i = 0; i < numberOfThrows; i++) {
        results.push(rng.selectOption(...options));
    }

    expect(results).toContain(0);
    expect(results).toContain(1);
    expect(results).toContain(2);

    const oobResults = results
        .filter(r => r !== 0 && r !== 1 && r !== 2);

    expect(oobResults.length).toBe(0);
});

test('rng without mock should pick all elements of an array if asked to pick more than what is available', () => {
    const rng = new RNG();

    const elements: Array<number> = [1, 2];

    const result = rng.selectRandomlyFromArray(elements, 4);

    expect(result).toBe(elements);
});

test('rng without mock should pick relatively random elements from an array', () => {
    const rng = new RNG();

    const elements: Array<number> = [1, 2, 3, 4];

    const numberOfThrows = 1_000;

    const results: Array<Array<number>> = [];

    for (let i = 0; i < numberOfThrows; i++) {
        results.push(rng.selectRandomlyFromArray(elements, 3));
    }

    const normalizedResults = results
        .map(result => Array.from(new Set(result)).sort())
        .map(a => JSON.stringify(a));

    const expectedNormalized: string[] = [
        [2, 3, 4],
        [1, 3, 4],
        [1, 2, 4],
        [1, 2, 3],
    ].map(a => JSON.stringify(a));

    for (let normalizedResult of normalizedResults) {
        expect(expectedNormalized).toContain(normalizedResult); // each throw should have exactly 3 distincts elements from the base array
    }

    const numberOfDifferentNormalized = new Set(normalizedResults).size;
    expect(numberOfDifferentNormalized).toBe(4); // each combination should be thrown at least once

    const numberOfDifferent = new Set(results).size;
    expect(numberOfDifferent).toBeGreaterThanOrEqual(5); // at least one combination should be featured in more than one order
});
