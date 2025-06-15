<template>
    <v-dialog max-width="500">
        <template v-slot:activator="{ props: activatorProps }">
            <v-btn v-bind="activatorProps" text="create" block size="x-large" append-icon="mdi-plus"></v-btn>
        </template>

        <template v-slot:default="{ isActive }">
            <v-card title="✨New Resume">
                <v-card-text>

                    <v-form>
                        <v-text-field label="Title" />
                    </v-form>

                </v-card-text>

                <v-card-actions>
                    <v-spacer></v-spacer>

                    <v-btn text="Close Dialog" @click="isActive.value = false"></v-btn>
                </v-card-actions>
            </v-card>
        </template>
    </v-dialog>
</template>