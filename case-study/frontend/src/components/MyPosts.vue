<template>
    <div class="home-container">
        <Navbar></Navbar>
        <v-layout align-center justify-center>
            <v-flex xs12 sm8 md11>
                <v-card class="elevation-12" style="padding: 20px; margin-top: 40px; margin-bottom: 20px;">
                    <v-card-title>
                        <v-spacer></v-spacer>
                    </v-card-title>
                    <v-card-text>
                        <v-container fluid>
                            <v-data-iterator
                                :items="posts"
                                :items-per-page.sync="itemsPerPage"
                                :page.sync="page"
                                :search="search"
                                :sort-by="sortBy.toLowerCase()"
                                :sort-desc="sortDesc"
                                hide-default-footer
                            >
                                <template v-slot:header>
                                    <v-toolbar dark color="blue darken-2" class="mb-6">
                                        <v-text-field
                                            v-model="search"
                                            clearable
                                            flat
                                            solo-inverted
                                            hide-details
                                            prepend-inner-icon="mdi-magnify"
                                            label="Search"
                                        ></v-text-field>
                                        <template v-if="$vuetify.breakpoint.mdAndUp">
                                            <v-select
                                                v-model="sortBy"
                                                flat
                                                solo-inverted
                                                hide-details
                                                :items="keys"
                                                prepend-inner-icon="mdi-magnify"
                                                label="Sort by"
                                                class="ml-4"
                                            ></v-select>
                                            <v-spacer></v-spacer>
                                            <v-btn-toggle v-model="sortDesc" mandatory>
                                                <v-btn large depressed color="blue" :value="false">
                                                    <v-icon>mdi-arrow-up</v-icon>
                                                </v-btn>
                                                <v-btn large depressed color="blue" :value="true">
                                                    <v-icon>mdi-arrow-down</v-icon>
                                                </v-btn>
                                            </v-btn-toggle>
                                        </template>
                                    </v-toolbar>
                                </template>

                                <template v-slot:default="props">
                                    <v-row>
                                        <v-col v-for="item in props.items" :key="item.id" cols="12" sm="6" md="4" lg="3">
                                            <v-card class="pb-2">
                                                <v-card-title class="subheading font-weight-bold blue-darken-4--text">
                                                    {{ item.title }}
                                                    <v-spacer></v-spacer>
                                                    <span class="blue--text ml-2">{{ item.rate }}</span><span v-if="currentUser.role === undefined || currentUser.role != 'ROLE_USER'">/5</span>
                                                </v-card-title>
                                                <v-divider></v-divider>
                                                <v-list dense>
                                                    <p class="ml-4 mr-6 mt-3" style="text-justify: inter-word;">{{ item.text }}</p>
                                                </v-list>
                                                <v-footer color="white">
                                                    <v-btn icon large class="ml-8" color="red" @click="deletePost(item)" v-if="currentUser.uid === item.author_id">
                                                        Delete<v-icon class="ml-2" dark>mdi-delete-outline</v-icon>
                                                    </v-btn>
                                                    <v-spacer></v-spacer>
                                                    <v-btn icon large class="mr-12" color="blue" @click="openCommentsDialog(item.id)">
                                                        Comments<v-icon class="ml-2" dark>mdi-comment-outline</v-icon>
                                                    </v-btn>
                                                </v-footer>
                                            </v-card>
                                        </v-col>
                                    </v-row>
                                </template>

                                <template v-slot:footer>
                                    <v-row class="mt-3" align="center" justify="center">
                                        <v-spacer></v-spacer>
                                        <span class="mr-4 grey--text">
                                            Page {{ page }} of {{ numberOfPages }}
                                        </span>
                                        <v-btn dark color="blue darken-2" class="mr-1" @click="formerPage">
                                            <v-icon>mdi-chevron-left</v-icon>
                                        </v-btn>
                                        <v-btn dark color="blue darken-2" class="ml-1" @click="nextPage">
                                            <v-icon>mdi-chevron-right</v-icon>
                                        </v-btn>
                                    </v-row>
                                </template>

                            </v-data-iterator>
                        </v-container>
                    </v-card-text>
                </v-card>
            </v-flex>

        </v-layout>
        <Comments v-model="showComments" :postId="this.showCommentsForPost" :comments="this.comments"></Comments>
        <Confirm ref="confirmDeletePost"></Confirm>
        <v-snackbar v-model="successfullyDeleted" top color="green darken-2">
            You deleted post successfully.
        </v-snackbar>
    </div>
</template>

<script>
    import Navbar from './Navbar.vue'
    import { mapState } from 'vuex'
    import Comments from './Comments.vue'
    import jwt_decode from "jwt-decode";
    import Confirm from './Confirm.vue'

    export default {
        name: 'MyPosts',
        components: {
            Navbar,
            Comments,
            Confirm
        },
        data: () => ({
            currentUser: {},
            search: '',
            sortDesc: false,
            page: 1,
            itemsPerPage: 4,
            postItems: [],
            sortBy: 'title',
            keys: [
                'Title',
            ],
            showComments: false,
            showCommentsForPost: 0,
            comments: [],
            successfullyDeleted: false
        }),
        methods : {
            descending (des) {
                this.desc = des;
            },
            nextPage () {
                if (this.page + 1 <= this.numberOfPages) this.page += 1
            },
            formerPage () {
                if (this.page - 1 >= 1) this.page -= 1
            },
            setRating: function(post) {
                this.$store.dispatch('ratePost', post).then(resp => {
                    this.postItems[this.postItems.indexOf(post)].rate = resp.data;
                });
            },
            openCommentsDialog (id) {
                this.$store.dispatch('getComments', id).then((resp) => {
                    this.comments = resp.data;
                    this.showCommentsForPost = id;
                    this.showComments = true;
                })
                .catch(err => {
                    console.log(err.response);
                });
            },
            goToAddPost : function () {
                this.$router.push({name: 'AddPost'});
            },
            deletePost : function (post) {
                this.$refs.confirmDeletePost.open('Delete', 'Are you sure you want to delete ' + post.title + '?', { color: 'orange' }).then((confirm) => {
                    if (confirm) {
                        this.$store.dispatch('deletePost', post)
                        .then(() => {
                            this.successfullyDeleted = true;
                            this.postItems = this.postItems.filter(m => m.id !== post.id);
                        })
                        .catch(err => {
                            console.log(err);
                        });
                    }
                })
            },
        },
        computed: {
            numberOfPages () {
                return Math.ceil(this.posts.length / this.itemsPerPage)
            },
            ...mapState({
                moviesState: state => state.allMovies
            }),
            posts () {
                return this.postItems;
            },
        },
        created () {
            if (localStorage.getItem('token')) {
                var decoded = jwt_decode(localStorage.getItem('token'));
                this.currentUser = decoded;
            }
            this.$store.dispatch('getMyPosts').then(resp => {
                this.postItems = resp.data;
            });
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