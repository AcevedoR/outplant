<script lang="ts">
    export let lines: string[];
</script>

<section>
    <ul>
        {#each lines as line}
            <li class="log-displayer__entry">{line}</li>
        {/each}
    </ul>
</section>

<style>
    .log-displayer__entry {
        text-align: justify;
        line-height: 1.6;
    }
</style>
