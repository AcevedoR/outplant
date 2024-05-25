<script lang="ts">
  import Fa from "svelte-fa";

  export let label: string;
  export let value: number;
  export let growth: number | undefined = undefined;
  import {faArrowTrendUp, faArrowTrendDown} from '@fortawesome/free-solid-svg-icons';
</script>

<li class="variable-dashboard__item">
  <p>{label}: {value}</p>
  {#if growth}
    {#if growth > 0}
      <div class="icon positive-growth">
        <Fa icon={faArrowTrendUp} />
      </div>
    {:else if growth < 0}
      <div class="icon negative-growth">
        <Fa icon={faArrowTrendDown} />
      </div>
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
    padding: 5px;
    text-align: center;
    transition: all ease-out 200ms;
    margin: 0.5em;
    width: 20%;
    display: inline-block;
  }
  .variable-dashboard__item p{
    display: inline-block;
  }
  .variable-dashboard__item .icon{
    display: inline-block;
    font-size: 30px;
    padding-left: 10px;
    top: 5px;
    position: relative;
  }
  .variable-dashboard__item .icon.positive-growth {
    color: #1f73b9;
  }
  .variable-dashboard__item .icon.negative-growth {
    color: #a9303e;
  }

  @media (max-width: 500px) {
    .variable-dashboard__item {
        width: 90%;
    }
  }
</style>
