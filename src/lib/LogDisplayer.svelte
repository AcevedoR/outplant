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
        padding-top: 32px;
        text-align: justify;
        line-height: 1.6;
    }

    .log-displayer__entry:first-of-type {
        padding-top: 0;
    }
</style>
