import { existsSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';

const locales = ["fr_FR"];
const usedTags = [];

function main() {

    const chainFiles = readdirSync("./chains")
        .filter(filename => filename.endsWith(".json"))
        .filter(filename => filename != "schema.json")
        .map(filename => "./chains/"+filename);

    chainFiles.map(fileName => collectTranslationTagsOfChain(fileName))
        .forEach(tags => usedTags.push(...tags));

        chainFiles.forEach(fileName => updateTranslationTagsOfChain(fileName));
}

function collectTranslationTagsOfChain(chainFileName) {
    const eventChain = JSON.parse(readFileSync(chainFileName));

    const tags = [];

    for (const event in eventChain.events) {
        const eventText = eventChain.events[event].text;
        if (eventText) {
            let tag = extractTag(eventText);
            if (tag) {
                tags.push(tag)
            }
        }
    
        const eventChoices = eventChain.events[event].choices;
        if (eventChoices) {
            for (let i = 0; i < eventChoices.length; i++) {
                const choiceText = eventChain.events[event].choices[i].text;
                let tag = extractTag(choiceText);
                if (tag) {
                    tags.push(tag)
                }
            }
        }
    }

    return tags;
}

function updateTranslationTagsOfChain(chainFileName) {
    const eventChain = JSON.parse(readFileSync(chainFileName));

    const lines = [];

    for (const event in eventChain.events) {
        const eventText = eventChain.events[event].text;
        if (eventText) {
            let tag = extractTag(eventText);
            if (!tag) {
                tag = generateNewTag();
                eventChain.events[event].text += " #" + tag 
            }
            lines.push({tag, text: removeTag(eventChain.events[event].text)});
        }
    
        const eventChoices = eventChain.events[event].choices;
        if (eventChoices) {
            for (let i = 0; i < eventChoices.length; i++) {
                const choiceText = eventChain.events[event].choices[i].text;
                let tag = extractTag(choiceText);
                if (!tag) {
                    tag = generateNewTag();
                    eventChain.events[event].choices[i].text += " #" + tag
                }
                lines.push({tag, text: removeTag(eventChain.events[event].choices[i].text)});
            }
        }
    }
    
    writeFileSync(chainFileName, JSON.stringify(eventChain, null, "  "));

    locales.forEach(locale => {
        const tsvFileName = toTranslationFileName(chainFileName, locale);
        const alreadyTranslatedContent = readTranslationFile(tsvFileName);
        
        let tsvContent = "";
        for (let i = 0; i < lines.length; i++) {
            const tag = lines[i].tag;
            const alreadyTranslatedText = alreadyTranslatedContent.find(line => line.tag === tag)?.text;
            const text = alreadyTranslatedText || lines[i].text;
    
            tsvContent += `${tag}\t${text}\n`;
        }
        writeFileSync(tsvFileName, tsvContent);
    });
}

function extractTag(line) {
    if (line.length > 7 && line[line.length-7] === '#') {
        return line.slice(-6);
    }
    return null;
}

function removeTag(line) {
    return line.slice(0, -8);
}

function generateNewTag() {
    const charset = "abcdefghijklmnopqrstuvwxyz0123456789";
    let tag = '';

    for (let i = 0; i < 6; i++) {
        const randomIndex = Math.floor(Math.random() * charset.length);
        tag += charset.charAt(randomIndex);
    }

    if (usedTags.includes(tag)) {
        return generateNewTag(); // as long as we don't handle 1+ billion of lines, that'll be good enough probability-wise
    }
    usedTags.push(tag);

    return tag;
}      

function readTranslationFile(tsvFileName) {
    if (!existsSync(tsvFileName)) {
        return [];
    }
    return readFileSync(tsvFileName).toString()
            .split("\n")
            .map(line => {
                const s = line.split("\t");
                return { tag: s[0], text: s[1] };
            });
}

function toTranslationFileName(chainFileName, locale) {
    return chainFileName.replace(".json", `.${locale}.tsv`);
}

main();
