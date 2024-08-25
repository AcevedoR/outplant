<script lang="ts">
  import Fa from "svelte-fa";

  export let label: string;
  export let value: number;
  export let previousValue: number | undefined = undefined;
  import {faArrowTrendUp, faArrowTrendDown} from '@fortawesome/free-solid-svg-icons';
</script>

<li class="variable-dashboard__item">
  <span>{label}: {value}</span>
  {#if previousValue}
    {#if previousValue < value}
      <span class="icon positive-growth">
        <Fa icon={faArrowTrendUp} />
      </span>
    {:else if previousValue > value}
      <span class="icon negative-growth">
        <Fa icon={faArrowTrendDown} />
      </span>
    {/if}
  {/if}
</li>

<style>
  .variable-dashboard__item {
    box-shadow: 0 0 20px var(--main-color);
    border: 2px solid var(--main-color-light);
    border-radius: 0 0 10px 0;
    background-color: var(--main-color-dark);
    text-shadow: 0 0 10px var(--main-color);
    color: var(--main-color-light);
    text-align: center;
    transition: all ease-out 200ms;
    margin: 0.5em;
    width: 20%;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 2em;
  }

  .icon {
    padding-left: 0.3em;
    font-size: 1.5em;
  }

  .positive-growth {
    color: #1f73b9;
  }

  .negative-growth {
    color: #a9303e;
  }

  @media (max-width: 500px) {
    .variable-dashboard__item {
        width: 40%;
    }
  }
</style>
