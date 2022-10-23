<script lang="ts">
    import { consoleOutput, channels } from '../stor'
    import { afterUpdate, beforeUpdate } from 'svelte'
    import { open } from '@tauri-apps/api/shell'
    import { Dir, readTextFile, writeFile } from '@tauri-apps/api/fs'

    import axios from "axios"

    let channelsList
    channels.subscribe(value => {
		channelsList = value
	})

    const sleep = (milliseconds) => {
       return new Promise(resolve => setTimeout(resolve, milliseconds))
    }

    let cached_users = {}
    let client_id = "fjeq1q11z4cggh3zmzk6q5giu68qpe"
    const execute = async () => {
        if (userList.trim().split("\n").length != 0) {
            for (const c of channelsList) {
                let c_resp
                try {
                    c_resp = await axios({
                        method: 'get',
                        url: 'https://api.twitch.tv/helix/users',
                        params: {
                            login: c,
                        },
                        headers: {
                            "Client-ID": client_id
                        }
                    })
                    consoleOutput.update(v => v + "In channel " + c + "\n")
                } catch (e) {
                    if (e.response.status == 401) {
                        axios({
                            method: 'post',
                            url: 'https://id.twitch.tv/oauth2/revoke',
                            data: 'client_id='+ client_id + '&token=' + ps,
                            headers: { "Content-Type": "application/x-www-form-urlencoded" },
                        }).catch(function (error){
                            if (error.response.status == 400) {
                                consoleOutput.update(v => v + "Could not revoke token\n")
                            }
                        })
                        consoleOutput.update(v => v + "Credentials Invalid. Get new token\n")
                        return
                    }
                }
                
                for (const u of userList.split("\n")) {
                    if (!(u in cached_users)) {
                        let u_resp
                        try {
                            u_resp = await axios({
                                method: 'get',
                                url: 'https://api.twitch.tv/helix/users',
                                params: {
                                    login: u,
                                },
                                headers: {
                                    "Client-ID": client_id
                                }
                            })
                            cached_users[u] = u_resp.data["data"][0]["id"]
                        } catch (e) {
                            if (e.response.status == 401) {
                                consoleOutput.update(v => v + "Credentials Invalid. Get new token\n")
                                return
                            }
                        }
                    }
                    
                    try {
                        let ban_resp = await axios({
                            method: 'post',
                            url: 'https://api.twitch.tv/helix/moderation/bans',
                            params: {
                                broadcaster_id: c_resp.data["data"][0]["id"],
                                moderator_id: un_id
                            },
                            headers: {
                                "Client-ID": client_id
                            },
                            data: {
                                data: {
                                    user_id: cached_users[u],
                                    reason: reasonValue
                                }
                            }
                        })

                        if (ban_resp.status == 200) {
                            consoleOutput.update(v => v + "Banned user " + u + " (" + cached_users[u] + ")\n")
                        }

                    } catch (e) {
                        
                        if (e.response.status == 400) {
                            consoleOutput.update(v => v + "User already banned " + u + "\n")
                        } else if (e.response.status == 401) {
                            consoleOutput.update(v => v + "Credentials Invalid. Get new token\n")
                            return
                        } else if (e.response.status == 403) {
                            consoleOutput.update(v => v + "Forbidden (you're probably not a mod) ¯\\_(ツ)_/¯ \n")
                        } else if (e.response.status == 409) {
                            consoleOutput.update(v => v + "Conflict. Someone else was moderating user " + u +"\n")
                        } else if (e.response.status == 429) {
                            consoleOutput.update(v => v + "Too Many Requests. Stopping...\n")
                            return
                        } else if (e.response.status == 500) {
                            consoleOutput.update(v => v + "Twitch Brokey\n")
                            return
                        }

                    }

                    await sleep(250)
                }
            }
            consoleOutput.update(v => v + "Completed!\n")
        }
    }

    let userList = ""
    let reasonValue = ""

    let output
    consoleOutput.subscribe(value => {
		output = value
	})
    let statusBox
    let autoscroll
    beforeUpdate(() => {
		autoscroll = statusBox && (statusBox.offsetHeight + statusBox.scrollTop) > (statusBox.scrollHeight-20)
	})
    afterUpdate(() => {
		if (autoscroll) statusBox.scrollTo(0, statusBox.scrollHeight)
	})
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
    let un_id
    let ps = ""
    readConfigFile().then(() => {
        if (un != "" && ps != "") {
            axios.defaults.headers.common['Authorization'] = "Bearer " + ps
            axios({
                method: 'get',
                url: 'https://id.twitch.tv/oauth2/validate'
            }).then(function (response) {
                if (response.status == 200) {
                    un_id = response.data["user_id"]
                    consoleOutput.update(v => v + "Connected as " + un + " (" + un_id + ")\n")
                    let seconds = response.data["expires_in"]
                    let d = Math.floor(seconds / (3600*24))
                    let h = Math.floor(seconds % (3600*24) / 3600)
                    let m = Math.floor(seconds % 3600 / 60)
                    let s = Math.floor(seconds % 60)

                    let dDisplay = d > 0 ? d + (d == 1 ? " day, " : " days, ") : ""
                    let hDisplay = h > 0 ? h + (h == 1 ? " hour, " : " hours, ") : ""
                    let mDisplay = m > 0 ? m + (m == 1 ? " minute, " : " minutes, ") : ""
                    let sDisplay = s > 0 ? s + (s == 1 ? " second" : " seconds") : ""
                    let time_string = dDisplay + hDisplay + mDisplay + sDisplay
                    consoleOutput.update(v => v + "Token expires in " + time_string + "\n")
                }
            }).catch(function (error) {
                if (error.response.status == 401) {
                    consoleOutput.update(v => v + "Credentials Invalid. Get new token\n")
                }
            })
        } else {
            consoleOutput.update(v => v + "No credentials entered\n")
        }
    })

    let modalHidden = true
    const settings = () => {
        if (modalHidden) {
            readConfigFile()
        } else {
            writeConfigFile()
            axios.defaults.headers.common['Authorization'] = "Bearer " + ps
            axios({
                method: 'get',
                url: 'https://id.twitch.tv/oauth2/validate'
            }).then(function (response) {
                if (response.status == 200) {
                    un_id = response.data["user_id"]
                    consoleOutput.update(v => v + "Connected as " + un + " (" + un_id + ")\n")
                    let seconds = response.data["expires_in"]
                    let d = Math.floor(seconds / (3600*24))
                    let h = Math.floor(seconds % (3600*24) / 3600)
                    let m = Math.floor(seconds % 3600 / 60)
                    let s = Math.floor(seconds % 60)

                    let dDisplay = d > 0 ? d + (d == 1 ? " day, " : " days, ") : ""
                    let hDisplay = h > 0 ? h + (h == 1 ? " hour, " : " hours, ") : ""
                    let mDisplay = m > 0 ? m + (m == 1 ? " minute, " : " minutes, ") : ""
                    let sDisplay = s > 0 ? s + (s == 1 ? " second" : " seconds") : ""
                    let time_string = dDisplay + hDisplay + mDisplay + sDisplay
                    consoleOutput.update(v => v + "Token expires in " + time_string + "\n")
                }
            }).catch(function (error) {
                if (error.response.status == 401) {
                    consoleOutput.update(v => v + "Credentials Invalid. Get new token\n")
                }
            })
        }
        modalHidden = !modalHidden
    }
    const openGenerator = async () => {
        await open("https://id.twitch.tv/oauth2/authorize?client_id=" + client_id + "&redirect_uri=https://chipstm.github.io/TwitchCrossMod&response_type=token&scope=moderator:manage:banned_users")
    }
</script>
<div id="form" class="column p-5">
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-missing-attribute -->
    <p>Status <span class="is-pulled-right"><a on:click={settings}>&#9881;</a></span></p>
    <div class:is-hidden={modalHidden}>
        <!-- svelte-ignore a11y-click-events-have-key-events a11y-missing-attribute -->
        <a on:click={openGenerator}>Generate token</a>
        <input class="input" type="text" placeholder="username" bind:value={un} />
        <input class="input" type="password" placeholder="XXXXXXXXXXXXXXXXXX" bind:value={ps} />
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