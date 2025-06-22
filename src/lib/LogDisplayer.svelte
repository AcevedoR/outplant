<script lang="ts">
    export let linesByChain: { [key: string]: Array<string> } = {};
    $: currentChain = getLastChain(linesByChain);

    const getLastChain = (linesByChain: { [key: string]: Array<string> }) => {
        let entries = Object.entries(linesByChain);
        if(entries.map(([_chainName, lines]) => lines.length).flat().reduce((a,b) => a+b) > 1) {
            return entries[entries.length-1][0];
        }
    };
</script>
<section>
    {#each Object.entries(linesByChain) as [chainName, lines] (chainName)}
        <ul class="log-displayer__entry-group { (currentChain && currentChain === chainName) ? currentChain + ' highlight-group' : ''}">
            {#each lines as line, i (i)}
                <li class="log-displayer__entry">{line}</li>
            {/each}
        </ul>
    {/each}
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

    .log-displayer__entry-group {
        position: relative;
    }

    /* between each ul.log-displayer__entry-group */
    section ul.log-displayer__entry-group + ul.log-displayer__entry-group {
        border-top: 1px solid var(--main-color-light);
        padding-top: 20px;
        margin-top: 20px
    }

    .highlight-group.log-displayer__entry-group .log-displayer__entry:last-of-type::before {
        white-space: nowrap;
        content: ">";
        display: inline;
        padding-left: 0;
        position: absolute;
        left: -15px;
    }
</style>
