<template>
    <v-dialog max-width="500">
        <template v-slot:activator="{ props: activatorProps }">
            <v-btn v-bind="activatorProps" text="create" block size="x-large" append-icon="mdi-plus"></v-btn>
        </template>

        <template v-slot:default="{ isActive }">
            <v-card title="✨New Resume">
                <v-card-text>

                    <v-window>
                        <v-window-item>
                            <p>it looks like you have not choose</p>
                        </v-window-item>
                    </v-window>

                </v-card-text>

                <v-card-actions>
                    <v-spacer></v-spacer>

                    <v-btn text="Close Dialog" @click="isActive.value = false"></v-btn>
                </v-card-actions>
            </v-card>
        </template>
    </v-dialog>
</template>