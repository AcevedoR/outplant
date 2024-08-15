<script lang="ts">
	import { createEventDispatcher } from 'svelte';
    import type { ViewChoice } from './engine/engine';

    export let choices: ViewChoice[] | undefined;
    export let gameStart: boolean;

    const dispatch = createEventDispatcher();

    function makeChoice(i: number) {
		dispatch('choiceMade', {
			index: i,
		});
	}

    function next() {
        dispatch('next', {});
    }
</script>

<section class="choice-displayer">
    {#if !!choices && choices.length !== 0}
        <p class="choice-displayer__cta">{ "What's your response?" }</p>
        <ul class="choice-displayer__entries">
            {#each choices as choice, i (i)}
                {#if !choice.hidden }
                    <li>
                        <button
                            type="button"
                            class="choice-displayer__button"
                            on:click={() => makeChoice(i)}
                        > { choice.text } </button>
                    </li>
                {/if }
            {/each}
        </ul>
    {:else if gameStart}
        <button
            type="button"
            class="choice-displayer__button"
            on:click={() => next()}
        > {"Let's begin!"}</button>
    {:else}
        <p class="choice-displayer__cta">{"There is nothing to do for you right now."}</p>
        <button
            type="button"
            class="choice-displayer__button"
            on:click={() => next()}
        > {"Wait until next cycle"}</button>
    {/if}
</section>

<style>

.choice-displayer__entries {
  display: flex;
  justify-content: space-around;
  flex-wrap: wrap;
  flex-direction: column;
}

.choice-displayer__button {
  box-shadow: 0 0 10px var(--main-color);
  border: 2px solid var(--main-color-light);
  background-color: color-mix(in srgb, var(--main-color) 50%, var(--main-color-dark));
  color: var(--main-color-light);
  padding: 5px 20px;
  transition: all ease-out 200ms;
  font-family: 'Orbitron', sans-serif;
  margin: 0.2em;
  font-size: 16px;
  border-radius: 0 0 10px 0;
  width: 100%;
  line-height: 1.4;
}

.choice-displayer__button:active {
  background-color: transparent;
}

.choice-displayer__button:focus {
  border: 3px solid white;
}

.choice-displayer__cta {
  margin-top: 0;
}
</style>
