<template>
    <v-navigation-drawer width="300" location="left" permanent :width="400">
        <h1 style="text-align: center;" class="mt-5">💻 Elijah Offutt 🤖</h1>
        <span style="text-align: center; display: inline-block; width: 100%; font-size: .8em;" class="mb-5">
            <u><i>Software</i></u>
            <b> Developer</b>
        </span>
        <v-row justify="space-around">
            <v-col cols="10">
                <v-list-item v-for="link in links" style="font-size: 1.5em" :to="link.location">
                    <template v-slot:prepend>
                        <span class="mr-2">{{ link.icon }}</span>
                    </template>
                    <v-list-item-title style="font-size: .85em;">
                        {{ link.text }}
                    </v-list-item-title>
                </v-list-item>
            </v-col>
        </v-row>

        <template v-slot:append>
            <v-row class="pb-5" justify="space-around">
                <v-col cols="10">
                    <h3>👔 ProForge ⚒️ <span style="font-size:.65em">v0.1</span></h3>
                    <p>Proudly <b>FOSS</b> under the <b>MIT</b> License, by <b><a href="">Elijah Offutt</a></b></p>
                </v-col>
            </v-row>
        </template>
    </v-navigation-drawer>
</template>

<script setup>

let links = [
    {
        text: 'Resumes',
        icon: '🗃️',
        location: '/',
    },
    {
        text: 'Profile',
        icon: '📋',
        location: '/Profile',
    },
    {
        text: 'Connect',
        icon: '🌀',
        location: '/Connect',
    },
    {
        text: 'Campaigns',
        icon: '💼',
        location: '/'
    },
    {
        text: 'Settings',
        icon: '⚙️',
        location: '/Settings'
    }
]

</script>