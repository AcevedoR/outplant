export type RNGOption<Type> = {value: Type; weight?: number};

export class RNG {
    selectOption<Type>(...options: Array<RNGOption<Type>>): Type {

        const totalWeights = options
            .map(option => option.weight || 1)
            .reduce((accumulator, currentValue) => accumulator + currentValue, 0);

        const randomValue = Math.floor(Math.random() * totalWeights);

        let value = 0;
        let i = 0
        while (i < options.length - 1 && value + (options[i].weight || 1) <= randomValue) {
            value += options[i].weight || 1;
            i++;
        }
        return options[i].value;
    }
}
