const translationFiles = import.meta.glob(["/chains/*.tsv"], {as: "raw", eager: true});

export type TranslationStore = {
    [locale: string]: { [lineId: string]: string };
};

const defaultTranslations: TranslationStore = {};

export function translate(text: string, locale: string, translations : TranslationStore = defaultTranslations ): string {
    if ((text.length < 7) || (text[text.length - 7] !== '#')) {
        return text;
    }
    const lineId = text.slice(-6);
    return translations[locale]?.[lineId] || text.slice(0, text.length-8)
}

function initTranslations() {
    for (const translationFile in translationFiles) {
        const {locale} = extractFileInfo(translationFile);
        defaultTranslations[locale] = {
            ...(defaultTranslations[locale] || {}),
            ...parseFileContent(translationFiles[translationFile]),
        };
    }
}

const translationRegexp = /^\/chains\/(.*)\.(.*)\.tsv$/;

export function extractFileInfo(path: string): {chainTitle: string; locale: string} {
    const [_, chainTitle, locale] = path.match(translationRegexp)!;
    return {chainTitle, locale};
}

export function parseFileContent(fileContent: string): {[lineId: string]: string} {
    const translations: { [lineId: string]: string } = {};

    for (const line of fileContent.split("\n")) {
        const parsed = parseLine(line);
        translations[parsed.lineId] = parsed.text; 
    }

    return translations;
}

export function parseLine(line: string): {lineId: string; text: string} {
    const [lineId, text] = line.split("\t");
    return {lineId, text};
}

initTranslations();
