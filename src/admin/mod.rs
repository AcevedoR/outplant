use std::collections::HashMap;
use std::fs;
use std::io::Write;

use crate::engine::chain_store::ChainStore;
use crate::engine::effect::{Effect, EffectType};

pub fn create_diagram() {
    println!("flowchart LR;");
    let store = ChainStore::new(vec!["./chains/earthquake.json".to_string()]);
    // println!("{:?}", store);
    let mut text: String = "flowchart LR;\n".to_owned();

    store.chains.iter().for_each(|chain| {
        chain.events.iter().for_each(|event| {
            if let Some(choices) = event.1.clone().choices {
                if !!!choices.is_empty() {
                    for (i, choice) in choices.iter().enumerate() {
                        // it is a choice event
                        // let choice_id = format!("choice_{}_{}([\"{}\"])", event.0, i, shorten(&choice.text)); could not reuse the ID AND display the text :/
                        let choice_id = format!("choice_{}_{}", event.0, i);
                        text.push_str(&format!("{}{{{{{} fa:fa-question-circle}}}} --> {}([\"{}\"])\n", event.0, event.0, choice_id, shorten(&choice.text)));
                        if !!!choice.next.is_empty() {
                            for choice_outcome in choice.clone().next {
                                text.push_str(&format!("{} --\"{}\"--> {}\n", choice_id, format_weight_and_timer(choice_outcome.weight, choice_outcome.timer), choice_outcome.event));
                                text.push_str(format_effects(&store, chain.clone().title, &choice_id, choice_outcome.effects.clone()).as_str());
                            }
                        }
                        text.push_str(format_effects(&store, chain.clone().title, &choice_id, choice.effects.clone()).as_str());
                    }
                }
            }
            println!("{:?}", event);
            if let Some(next_events) = event.1.clone().next {
                if !!!next_events.is_empty() {
                    for next_event in next_events.iter() {
                        let event_text = if event.1.text.is_empty() { event.0.to_string() } else { event.1.text.clone() };
                        text.push_str(&format!("{}[{}] --\"{}\"--> {}\n", event.0, shorten(&event_text), format_weight_and_timer(next_event.weight, next_event.timer), next_event.event));
                    }
                }
            }
            text.push_str(format_effects(&store, chain.clone().title, event.0, event.1.effects.clone()).as_str());
        })
    });

    fs::write("./generated_mermaid_graph.mermaid", text).expect("unable to write mermaid file");
}

fn format_effects(store: &ChainStore, chain: String, parent_name: &String, effects_opt: Option<HashMap<String, bool>>) -> String{
    let mut text = "".to_owned();

    if let Some(effects) = effects_opt {
        for (effect_activation_index, effect_activation) in effects.iter().enumerate() {
            let effect = store
                .clone()
                .get_by_name(chain.clone())
                .unwrap()
                .effects
                .get(effect_activation.0)
                .unwrap()
                .to_owned();
            text.push_str(format_effect(parent_name, effect, effect_activation_index, effect_activation.1).as_str());
        }
    }
    return text;
}

fn format_effect(parent: &String, effect: Effect, effect_index: usize, is_activated: &bool) -> String {
    //fa:fa-sync-alt stopwatch
    let mut text = " ".to_owned();
    let background_color = if *is_activated { "#0f0" } else { "#f00" };
    let border_style = match effect.effect_type {
        EffectType::Instant => { "stroke:#f66,stroke-width:1px,stroke-dasharray: 5 5" }
        EffectType::Permanent => { "stroke:#066,stroke-width:5px" }
    };
    let effect_id: String = format!("effect_{}_{}", parent, effect_index);

    text.push_str(&format!("{} --> {}[/{}/]\n", parent, effect_id, effect_id));

    text.push_str(&format!("style {} fill:{},color:#fff,{}", effect_id, background_color, border_style));

    text.push_str("\n");
    return text;
}

fn shorten(text: &str) -> &str {
    let max_size_allowed = 50;
    return if text.len() >= max_size_allowed {
        &text[..50]
    } else {
        text
    };
}

fn format_weight_and_timer(weight: Option<u32>, timer: Option<u32>) -> String {
    let mut s = " ".to_owned();
    if let Some(w) = weight {
        s.push_str(w.to_string().as_str());
    }
    if let Some(t) = timer {
        s.push_str(format!(" in {}", t).as_str())
    }
    return s;
}

macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        enum $name {
            $($variant = $val),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}