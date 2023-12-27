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
