<script lang="ts">
  export let label: string;
  export let value: number;
</script>

<li class="variable-dashboard__item">
  {label}: {value}
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
  }

  @media (max-width: 500px) {
    .variable-dashboard__item {
        width: 90%;
    }
  }
</style>
