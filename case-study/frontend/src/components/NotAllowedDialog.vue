<template>
    <div>
        <v-row justify="center">
            <v-dialog v-model="show" max-width="400px">
                <v-card class="px-4 py-4">
                    <v-card-title>
                        <h2 style="color: #1976D2; margin-bottom: 10px; margin-top: 10px">Sign in</h2>
                    </v-card-title>
                    <v-card-text>
                        You have to be signed in to complete this action.
                    </v-card-text>
                    <v-card-actions class="ml-3">
                        <v-spacer></v-spacer>
                        <v-btn color="blue darken-1" text @click="goToSignIn()">
                            Sign in here
                        </v-btn>
                    </v-card-actions>
                </v-card>
            </v-dialog>
        </v-row>
    </div>
</template>

<script>
    export default {
        name: 'NotAllowedDialog',
        data: () => ({
            
        }),
        props: ['value'],
        computed: {
            show: {
                get () {
                    return this.value
                },
                set (value) {
                    this.$emit('input', value)
                }
            },
        },
        methods : {
            goToSignIn : function () {
                this.$router.push('/sign-in');
            }
        },
    }
</script>