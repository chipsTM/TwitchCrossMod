<script lang="ts">
    import { consoleOutput, channels } from '../stor'
    import { afterUpdate, beforeUpdate } from 'svelte';
    import { open } from '@tauri-apps/api/shell';
    import { Dir, readTextFile, writeFile } from '@tauri-apps/api/fs'

    import { Client } from 'tmi.js'


    let channelsList
    channels.subscribe(value => {
		channelsList = value
	});

    const sleep = (milliseconds) => {
       return new Promise(resolve => setTimeout(resolve, milliseconds))
    }

    const execute = async () => {
        if (userList.trim().split("\n").length != 0) {
            for (const c of channelsList) {
                await client.join(c)
                for (const u of userList.split("\n")) {
                    try {
                        await client.ban(c, u, reasonValue)
                    } catch (e) {
                        consoleOutput.update(v => v + "Error with ban " + u + "\n")
                    }
                    await sleep(500)
                }
                await client.part(c)
            }
            consoleOutput.update(v => v + "Completed!\n")
        }
    }

    let userList = ""
    let reasonValue = ""

    let output;
    consoleOutput.subscribe(value => {
		output = value;
	});
    let statusBox;
    let autoscroll;
    beforeUpdate(() => {
		autoscroll = statusBox && (statusBox.offsetHeight + statusBox.scrollTop) > (statusBox.scrollHeight-20);
	});
    afterUpdate(() => {
		if (autoscroll) statusBox.scrollTo(0, statusBox.scrollHeight);
	});
    const scrollDown = () => {
        // do something
    }

    const writeConfigFile = async () => {
        try {
            await writeFile('config.json', "{\"username\":\"" + un + "\",\"pass\":\"" + ps + "\"}", { dir: Dir.App })

        } catch (e) {
            consoleOutput.update(v => v + e + "\n")
        }
    }

    const readConfigFile = async () => {
        try {
            let configFileContents = await readTextFile('config.json', { dir: Dir.App })

            let contents = JSON.parse(configFileContents)
            un = contents["username"]
            ps = contents["pass"]
        } catch (e) {
            consoleOutput.update(v => v + e + "\n")
        }
    }

    let un = ""
    let ps = ""
    readConfigFile().then(() => {
        if (un != "" && ps != "") {
            client.getOptions().identity.username = un
            client.getOptions().identity.password = ps
            client.connect().catch((e) => {
                consoleOutput.update(v => v + e + "\n")
            })
        } else {
            consoleOutput.update(v => v + "No credentials entered\n")
        }
    })

    let client = new Client({
        options: { debug: true, skipUpdatingEmotesets: true, skipMembership: true},
        identity: {
            username: un,
            password: ps
        }
    })
    client.on("connecting", () => {
        consoleOutput.update(v => v + "Connecting...\n")
    })
    client.on("connected", () => {
        consoleOutput.update(v => v + "Connected to twitch\n")
    })
    client.on("join", (c, u) => {
        consoleOutput.update(v => v + "Joined channel: " + c + "\n")
    })
    client.on("part", (c, u) => {
        consoleOutput.update(v => v + "Parted channel: " + c + "\n")
    })
    client.on("ban", (c, u, r) => {
        consoleOutput.update(v => v + u + " banned in " + c + ": " + reasonValue + "\n")
    })

    let modalHidden = true
    const settings = () => {
        if (modalHidden) {
            readConfigFile()
        } else {
            writeConfigFile()
            client.getOptions().identity.username = un
            client.getOptions().identity.password = ps
            client.connect().catch((e) => {
                consoleOutput.update(v => v + e + "\n")
            })
        }
        modalHidden = !modalHidden
    }
    const openGenerator = async () => {
        await open("https://twitchapps.com/tmi/")
    }
</script>
<div id="form" class="column p-5">
    <!-- svelte-ignore a11y-missing-attribute -->
    <p>Status <span class="is-pulled-right"><a on:click={settings}>&#9881;</a></span></p>
    <div class:is-hidden={modalHidden}>
        <!-- svelte-ignore a11y-missing-attribute -->
        <a on:click={openGenerator}>Generate token</a>
        <input class="input" type="text" placeholder="username" bind:value={un} />
        <input class="input" type="password" placeholder="oauth:XXXXXXXXXXXXXXXXXX" bind:value={ps} />
    </div>
    <textarea bind:this={statusBox} class="textarea is-dark" readonly bind:value={output} on:change="{scrollDown}"></textarea>
    <p>Form</p>
    <textarea class="textarea is-dark" type="textarea" placeholder="User List" bind:value={userList}></textarea>
    <textarea class="textarea is-dark" type="text" placeholder="Reason" bind:value={reasonValue}></textarea>
    <div class="has-text-centered">
        <button class="button is-primary is-outlined" on:click={execute}>Ban</button>
    </div>
</div>

<style>
    .button.is-primary.is-outlined {
        background-color: transparent;
        border-color: #1CB96A;
        color: #1CB96A;
    }
    .button.is-primary.is-outlined:hover {
        background-color: #1CB96A;
        border-color: #1CB96A;
        color: white;
    }
    textarea {
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
</style>