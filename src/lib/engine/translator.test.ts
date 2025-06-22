import { expect, test, describe, it } from 'vitest';
import {
	extractFileInfo,
	parseFileContent,
	parseLine,
	translate,
	type TranslationStore,
} from './translator';

describe('translate', () => {
	const mockTranslations: TranslationStore = {
		fr_FR: {
			ho65md: 'Cependant, un rapport se distingue:',
		},
	};

	it('should return the translated version of a line if it exists', () => {
		const text = 'One report catches your attention though: #ho65md';
		expect(translate(text, 'fr_FR', mockTranslations)).toEqual(
			'Cependant, un rapport se distingue:',
		);
	});

	it('should return the original version of a line if the requested locale is not available', () => {
		const text = 'One report catches your attention though: #ho65md';
		expect(translate(text, 'it_IT', mockTranslations)).toEqual(
			'One report catches your attention though:',
		);
	});

	it('should return the original version of a line if the requested line is not translated for the requested locale', () => {
		const text =
			'Praise them for being so skilled at “tactical retreat” #4nz2up';
		expect(translate(text, 'fr_FR', mockTranslations)).toEqual(
			'Praise them for being so skilled at “tactical retreat”',
		);
	});
});

test('extractFileInfo', () => {
	expect(extractFileInfo('/chains/earthquake.fr_FR.tsv')).toEqual({
		chainTitle: 'earthquake',
		locale: 'fr_FR',
	});
});

test('parseFileContext', () => {
	const fileContent = `ho65md	Cependant, un rapport se distingue:
hlux9j	“On y va maintenant ! Chaque minute compte !”
`;

	const expectedResult = {
		ho65md: 'Cependant, un rapport se distingue:',
		hlux9j: '“On y va maintenant ! Chaque minute compte !”',
	};

	expect(parseFileContent(fileContent)).toEqual(expectedResult);
});

test('parseLine', () => {
	expect(parseLine('ho65md	Cependant, un rapport se distingue:')).toEqual({
		lineId: 'ho65md',
		text: 'Cependant, un rapport se distingue:',
	});
});
