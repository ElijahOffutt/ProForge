<template>
    <v-container class="pt-12 mt-12">
        <v-row justify="space-around">
            <v-col cols="10">
                <v-window v-model="tab">
                    <v-window-item>
                        <h2>🥳 Welcome to Proforge 🎆</h2> <br />
                        <p>Get started by selecting where to put your Data on your Device. Your Data belongs to you and
                            will never be
                            shared with anyone with out your consent. Your data will be encrypted at rest using <b><u><a
                                        href="https://surrealdb.com/">SurrealDB 🚀</a></u></b> and with a
                            <i>User Name</i> and
                            <i>Password</i>. They will also be used to sign in to ProForge on later Logins. All Secure,
                            All on Device.
                        </p>
                        <br />
                        <v-divider />
                        <br />
                        <v-row>
                            <v-col cols="12">
                                <v-btn @click="databaseInitialized ? installDatabase() : selectDirectory()"
                                    size="x-large" block color="primary">
                                    {{ databaseInitialized ? '📥 Install Database 🗄️' : '💾 Select Data Location 📌' }}
                                </v-btn>
                            </v-col>
                            <v-col v-if="databaseInitialized" cols="12">
                                <v-btn @click="selectDirectory" size="large" block>
                                    Selected Location: <b>{{ databasePath }}</b>
                                </v-btn>
                            </v-col>
                            <v-col>
                                <v-btn @click="testHandler" block size="x-large">test hanlder</v-btn>
                            </v-col>
                        </v-row>
                    </v-window-item>
                </v-window>
            </v-col>
        </v-row>
    </v-container>
</template>

<script setup>

import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { useLocalStorage } from '@vueuse/core'

let tab = ref(null)
let Username = ref('')
let Password = ref('')
let databasePath = useLocalStorage('DatabaseLocation', '')
let databaseInitialized = computed(() => databasePath.value !== '')

let installDatabase = async () => {
    // Here you would add the logic to initialize and install the database
    // For example, you might call a backend API or run some setup code
    console.log('Installing database at:', databasePath.value)
    // Simulate installation delay
    await new Promise(resolve => setTimeout(resolve, 2000))
    alert('Database installed successfully at: ' + databasePath.value)
}

let selectDirectory = async () => {

    open({ directory: true, multiple: false })

        .then(async (location) => {
            try {

                let payload = {
                    function: 'configure_database_path',
                    data: location
                }
                let response = await invoke('database_handler', { payload })
                console.log(response)

            } catch (error) {

                console.error('Error selecting directory:', error)

            }
        }).catch((error) => {

            console.error('Error opening directory dialog:', error)

        })
}

let testHandler = async () => {
    try {
        let payload = {
            function: 'test_handler',
            data: 'test data string'
        }
        let response = await invoke('database_handler', { payload })
        console.log('Response from test handler:', response)
    } catch (error) {
        console.error('Error in test handler:', error)
    }
}

</script>