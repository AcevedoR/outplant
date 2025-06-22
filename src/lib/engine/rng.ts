export type RNGOption<Type> = { value: Type; weight?: number };

export class RNG {
	selectOption<Type>(...options: Array<RNGOption<Type>>): Type {
		const totalWeights = options
			.map((option) => option.weight || 1)
			.reduce((accumulator, currentValue) => accumulator + currentValue, 0);

		const randomValue = Math.floor(Math.random() * totalWeights);

		let value = 0;
		let i = 0;
		while (
			i < options.length - 1 &&
			value + (options[i].weight || 1) <= randomValue
		) {
			value += options[i].weight || 1;
			i++;
		}
		return options[i].value;
	}

	selectRandomlyFromArray<Type>(
		elements: Array<Type>,
		numberToSelect: number,
	): Array<Type> {
		if (numberToSelect > elements.length) {
			return elements;
		}

		const selectedIndices: Set<number> = new Set();
		while (selectedIndices.size < numberToSelect) {
			selectedIndices.add(Math.floor(Math.random() * elements.length));
		}
		return Array.from(selectedIndices).map((index) => elements[index]);
	}
}
