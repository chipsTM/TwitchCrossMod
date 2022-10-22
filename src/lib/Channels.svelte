<script lang="ts">
    import { createDir, Dir, readTextFile, writeFile } from '@tauri-apps/api/fs'
    import { appDir } from '@tauri-apps/api/path'
    import { channels, consoleOutput } from '../stor'

    const readChannelFile = async () => {
        try {
            let channelFileContents = await readTextFile('channels.txt', { dir: Dir.App })

            channels.set(channelFileContents.split("\n"))

        } catch (e) {
            consoleOutput.update(v => v + e + "\n")
        }
    }

    const writeChannelFile = async () => {
        try {
            await writeFile('channels.txt', channelsList.length == 0 ? "" : channelsList.join("\n"), { dir: Dir.App })

        } catch (e) {
            consoleOutput.update(v => v + e + "\n")
        }
    }

    const createAppDir = async () => {
        try{
            await createDir(".", { dir: Dir.App })

        } catch (e) {
            // consoleOutput.update(v => v + e + "\n")
        }
    }

    let newChannel = ""
    const addChannel = (evt) => {
        if (evt.key === "Enter") {
            if (!channelsList.includes(newChannel)) {
                channels.update(c => [...c, newChannel])
                writeChannelFile()
                consoleOutput.update(v => v + "Channel added: " + newChannel + "\n")
                newChannel = ""
            }
        }
    }

    const checkChannel = () => {
        if (channelsList.includes(newChannel)) {
            isDanger = true
        } else {
            isDanger = false
        }
    }

    // Create app dir on startup
    createAppDir()
    readChannelFile()

    let channelsList
    let isDanger = false
    channels.subscribe(value => {
		channelsList = value
	})

    const removeChannel = (ind) => {
        consoleOutput.update(v => v + "Channel removed: " + channelsList[ind] + "\n")
        channels.update(c => {
            c.splice(ind, 1)
            return c
        })
        writeChannelFile()
    }
</script>

<div class="column p-5">
    <p>Channels</p>
    <span class="path">
        {#await appDir()}
        fetching path
        {:then value}
        {value}channels.txt
        {:catch error}
        could not get path {error}
        {/await}
    </span>
    <div class="container">
        {#if isDanger}
        <p style="color: red;">Channel already exists</p>
        {/if}
        <input class="input is-dark" class:is-danger={isDanger} type="text" placeholder="Add Channel" bind:value={newChannel} on:keydown={addChannel} on:input={checkChannel} />
        <ul class="container">
            {#each channelsList as channel, i (channel)}
                <!-- svelte-ignore a11y-click-events-have-key-events a11y-missing-attribute -->
                <li class="m-5">{channel}<span class="is-pulled-right"><a on:click={() => removeChannel(i)}>X</a></span></li>
            {/each}
        </ul>
    </div>
</div>


<style>
    input {
        color: white;
        background-color: #444;
    }
    ::placeholder {
        color: #1CB96A;
    }
    p {
        color: white;
        font-weight: bold;
    }
    .path {
        font-size: xx-small;
    }
    li {
        color: white;
    }
</style>