import { readFileSync } from 'node:fs';
import { exit } from 'node:process';

// This script reads an event chain json file and outputs a mermaid flowchart based on it

if (process.argv.length !== 3) {
    console.error("example usage: node mermaid_graph_generator.mjs event_chains/rich_hunters.json");
    exit(1);
}

let eventChain;
try {
    eventChain = JSON.parse(readFileSync("./" + process.argv[2]));
} catch (err) {
    console.error("error reading event chain file: " + err);
    exit(1);
}

const transitions = Object.keys(eventChain.events).map(e => ({
    event: e,
    reachable: [
        ...(eventChain.events[e].next || []).map(next => next.event),
        ...(eventChain.events[e].choices || []).flatMap(choice => choice.next).map(next => next.event),
    ],
}));

const nodeLines = transitions.map(transition => {
    if (transition.event === "start") {
        return "start[[start]]";
    } else if (transition.reachable.length !== 0) {
        return `${transition.event.replace("end", "End")}(${transition.event})`;
    } else if (transition.event.includes("end")) {
        return `${transition.event.replace("end", "End")}[${transition.event}]`;
    }
    return ""
}).filter(l => l !== "")
.map(l => "\t" +l);

const arrowLines = transitions.filter(node => node.reachable.length !== 0)
    .map(node => `${node.event} --> ${node.reachable.map(event => event.replace("end", "End")).join(" & ")}`)
    .map(l => "\t" +l);

console.log([
    "flowchart TD",
    ...nodeLines,
    ...arrowLines,
].join("\n"));
