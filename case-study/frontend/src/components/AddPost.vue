<template>
    <div class="home-container">
        <Navbar></Navbar>
        <v-layout align-center justify-center>
            <v-flex xs12 sm8 md4>
                <v-card class="elevation-12" style="padding: 20px; margin-top: 40px;">
                    <v-card-text>
                        <h1 style="color: #1976D2; margin-bottom: 30px; margin-top: 10px">Add post</h1>
                        <v-form class="mt-4" v-model="isValid" ref="formRegister">
                            <v-text-field
                                color="blue darken-2"
                                prepend-icon="mdi-filmstrip"
                                name="title"
                                label="Title"
                                type="text"
                                v-model="post.title"
                                :rules="[v => !!v || 'Title is required']"
                                required
                            ></v-text-field>
                            <v-textarea
                                color="blue darken-2"
                                label="Description"
                                no-resize
                                counter
                                clearable
                                clear-icon="mdi-close-circle"
                                prepend-icon="notes"
                                rows="2"
                                v-model="post.text"
                                :rules="[v => !!v || 'Text of post is required']"
                                required
                            ></v-textarea>
                        </v-form>
                    </v-card-text>
                    <v-card-actions class="justify-center">
                        <v-btn 
                            color="blue darken-2"
                            class="my-2" 
                            style="width: 95%; color: white;" 
                            :disabled="!isValid"
                            v-on:click="add"
                        >Add</v-btn>
                    </v-card-actions>
                </v-card>
            </v-flex>
        </v-layout>
    </div>
</template>

<script>
    import Navbar from './Navbar.vue'

    export default {
        name: 'Post',
        components: {
            Navbar
        },
        data: () => ({
            post: {
                title: null,
                text: null,
            },
            isValid: true,
        }),
        methods : {
            add : function () {
                let newPost = this.post;
                this.$store.dispatch('addPost', newPost)
                .then(() => {
                    this.$router.push('/');
                })
                .catch(err => {
                    console.log(err);
                });
            },
        }
    }
</script>

<style scoped>
    .home-container {
        background-image: url(../assets/back.jpg);
        background-repeat: repeat;
        height: 100%;
    }
</style>