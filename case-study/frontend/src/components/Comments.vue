<template>
    <div>
        <v-row justify="center">
            <v-dialog v-model="show" max-width="800px">
                <v-card class="px-4 py-4">
                    <v-card-title>
                        <h2 class="ml-3" style="color: #1976D2; margin-bottom: 10px; margin-top: 10px">Comments</h2>
                    </v-card-title>
                    <v-card-text>
                        <v-container fluid>
                            <v-data-iterator :items="comments" :items-per-page="comments.length" hide-default-footer>
                                <template v-slot:default="props">
                                    <v-row>
                                        <v-col v-for="item in props.items" :key="item.name" cols="12" sm="6" md="4" lg="12">
                                            <v-card>
                                                <v-card-title class="subheading font-weight-semibold mb-n3">
                                                    {{ item.username }}
                                                    <v-spacer></v-spacer>
                                                    <v-btn fab small v-if="currentUser.username === item.username" color=red dark @click="deleteComment(item)">
                                                        <v-icon>mdi-delete-outline</v-icon>
                                                    </v-btn>
                                                </v-card-title>
                                                <v-list dense>
                                                    <v-list-item>
                                                        <v-list-item-content class="align-end">
                                                            {{ item.text }}
                                                        </v-list-item-content>
                                                    </v-list-item>
                                                </v-list>
                                            </v-card>
                                        </v-col>
                                    </v-row>
                                </template>
                            </v-data-iterator>
                        </v-container>
                    </v-card-text>
                    <v-card-actions class="ml-5" v-if="this.currentUser.role === undefined || this.currentUser.role === 'ROLE_USER'">
                        <v-text-field v-model="commentText" solo label="Your comment..."></v-text-field>
                        <v-btn color="blue darken-1" fab class="mb-7 ml-2" text @click="addComment()">
                            <v-icon>mdi-send</v-icon>
                        </v-btn>
                    </v-card-actions>
                </v-card>
            </v-dialog>
        </v-row>
        <NotAllowedDialog v-model="notAllowedDialog"></NotAllowedDialog>
    </div>
</template>

<script>
    import NotAllowedDialog from './NotAllowedDialog.vue'
    import jwt_decode from "jwt-decode";

    export default {
        name: 'Comments',
        components: {
            NotAllowedDialog
        },
        data: () => ({
            commentText: "",
            notAllowedDialog: false,
            currentUser: {}
        }),
        props: ['comments', 'value', 'postId'],
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
        created () {
            if (localStorage.getItem('token')) {
                var decoded = jwt_decode(localStorage.getItem('token'));
                this.currentUser = decoded;
            }
        },
        methods : {
            addComment : function () {
                if (this.currentUser.role === undefined) {
                    this.notAllowedDialog = true;
                    return;
                } else {
                    if (this.commentText !== '') {
                        this.$store.dispatch('addComment', { "text": this.commentText, "post_id": this.postId }).then(resp => {
                            this.comments.push(resp.data);
                            this.commentText = "";
                        })
                        .catch(err => {
                            console.log(err.response);
                        });
                    }
                }
            },
            deleteComment : function (comment) {
                this.$store.dispatch('deleteComment', comment).then(() => {
                    this.comments = this.comments.filter(c => c.id !== comment.id);
                })
            }
        },
    }
</script>