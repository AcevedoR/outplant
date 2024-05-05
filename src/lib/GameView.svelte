<script lang="ts">
    import VariableDashboardItem from "./VariableDashboardItem.svelte";
    import StarBackground from "./StarBackground.svelte";
    import {Engine, type GameInfos, isEndOfGameInfos, isOngoingGameInfos, type OngoingGameInfos} from "./engine/engine";
    import LogDisplayer from "./LogDisplayer.svelte";
    import ChoiceDisplayer, {type NextCycleEvent} from "./ChoiceDisplayer.svelte";
    import NextCycleTransition from "./animation/NextCycleTransition.svelte";

    const engine = new Engine();

    let viewModel: GameInfos = {
        linesByChain: {
            intro: [
                "Hello, I'm Aude, your personal AI that you created a while ago to help you and to remind you of your tasks. We are currently orbiting around an uninhabited planet, and your job is to introduce a new species on it. The end goal is to study how this species adapts to its environment, as well as finding new evolutionary traits that your company could patent and sell! You have just implemented the first subjects that you previously created in your lab. You told me that you were eager to watch them grow, and, hopefully, survive and adapt! You did not seem too sure to what extent it is wise for you to physically intervene with them, so, for now, you would try to let them be as much possible on their own.",
                "I will report to you events that you need to be aware of, and, for some of them, to react to. Your current goal is for you species to develop enough to reach 8 'Pop' (Population), and it would be a catastrophic failure if it went down to 0.",
                "Additionally, the 'Eco' (Ecology) represents the stability of the current biome your species is in, the maximum value is 12 and means the biome is stable and is in a perfect state for you species to grow in! Obviously, the more it goes down, the more you will have issues.",
                "You better start emerging soon from your sleep and get to work, go grab a coffee!",
            ],
        },
    };

    $: displayModel = (() => {
        if (isEndOfGameInfos(viewModel)) {
            return {linesByChain: {end:[viewModel.isVictory ? "You won!" : "You lose!"]}} as OngoingGameInfos;
        }
        return viewModel as GameInfos;
    })();

    $: choices = (() => {
        if ("choices" in displayModel) {
            return displayModel.choices;
        }
        return undefined;
    })();

    $: gameStart = (() => {
        if (isOngoingGameInfos(displayModel)) {
            return !!(displayModel as OngoingGameInfos).linesByChain['intro'];
        }
        return false;
    })();

    let playNextCycleTransition: (textToDisplay: string) => Promise<void>;

    function handleNextCycle(event: CustomEvent) {
        const nextCycleEvent = event.detail as NextCycleEvent;
        playNextCycleTransition(nextCycleEvent.type)
            ?.then(() => {
                viewModel = engine.nextCycle();
                updateCounters();
            });
    }

    function handleMakeChoice(event: CustomEvent) {
        viewModel = engine.makeChoice(event.detail.index);
        updateCounters();
    }

    let pop = engine.state.population;
    let eco = engine.state.ecology;
    let money = engine.state.money;
    let turnCounter = engine.state.turnCounter;

    function updateCounters() {
        pop = engine.state.population;
        eco = engine.state.ecology;
        money = engine.state.money;
        turnCounter = engine.state.turnCounter;
    }
</script>

<div>
    <header>
        <ul class="variable-dashboard">
            <VariableDashboardItem label="Pop" value={pop} growth={isOngoingGameInfos(viewModel) ? viewModel.stateInformations?.populationChange : undefined} />
            <VariableDashboardItem label="Eco" value={eco} growth={isOngoingGameInfos(viewModel) ? viewModel.stateInformations?.ecologyChange : undefined} />
            <VariableDashboardItem label="€€€" value={money} growth={isOngoingGameInfos(viewModel) ? viewModel.stateInformations?.moneyChange : undefined} />
            <VariableDashboardItem label="Turn" value={turnCounter} />
        </ul>
    </header>

    {#if 'linesByChain' in displayModel && Object.keys(displayModel.linesByChain).length > 0}
        <LogDisplayer linesByChain={displayModel.linesByChain} />
    {/if}

    {#if !("isVictory" in viewModel)}
        <ChoiceDisplayer
                {choices}
                {gameStart}
                on:nextCycle={handleNextCycle}
                on:choiceMade={handleMakeChoice}
        />
    {/if}

    <NextCycleTransition bind:playAnimation={playNextCycleTransition}/>

    <StarBackground/>
</div>

<style>
    .variable-dashboard {
        display: flex;
        justify-content: space-around;
        align-items: center;
        flex-wrap: wrap;
    }

    @media (max-width: 500px) {
        .variable-dashboard {
            flex-direction: column;
            align-items: center;
        }
    }
</style>
