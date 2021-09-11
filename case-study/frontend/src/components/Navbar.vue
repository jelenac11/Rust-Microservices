<template>
    <v-app-bar color="blue darken-2" dark>
        <v-toolbar-title class="mx-4">Jelena's posts</v-toolbar-title>
        <v-btn class="ml-3 mx-2" outlined @click="goToPosts()">
            posts
        </v-btn>
         <v-btn class="mx-2" outlined @click="goToAddAdmin()" v-if="currentUser.role === 'ROLE_ADMIN'">
            add admin
        </v-btn>
        <v-btn class="mx-2" outlined @click="goToMyPosts()" v-if="currentUser.role === 'ROLE_ADMIN'">
            my posts
        </v-btn>
        <v-spacer></v-spacer>
        <v-menu open-on-click transition="slide-y-transition" bottom left offset-y v-if="currentUser.role !== undefined">
            <template v-slot:activator="{ on, attrs }">
                <v-btn style="margin-right: 1px;" class="account" icon v-bind="attrs" v-on="on">
                    <v-icon>mdi-account</v-icon>
                </v-btn>
                <h2 class="mr-4" style="font-weight: 400;">
                    {{currentUser.sub}}
                </h2>
            </template>
            <v-card class="mx-auto" max-width="300" tile>
                <v-list dense>
                    <v-subheader id="log-out" @click="logOut()" class="mr-2"><v-icon class="mr-2">mdi-exit-to-app</v-icon><b>Log out</b></v-subheader>
                </v-list>
            </v-card>
        </v-menu>
        <div v-else>
            <v-btn outlined @click="goToSignIn()">
                sign in
            </v-btn>
            <v-btn class="mx-4" outlined @click="goToSignUp()">
                sign up
            </v-btn>
        </div>
    </v-app-bar>
</template>

<script>
    import jwt_decode from "jwt-decode";
    import Vue from 'vue'
    
    export default {
        name: 'Navbar',
        data: () => ({
            currentUser: {},
        }),
        methods: {
            logOut: function() {
                this.$router.push('/sign-in')
                localStorage.removeItem('token');
                delete Vue.prototype.$axios.defaults.headers.common['Authorization'];
            },
            goToPosts: function() {
                let path = '/';
                if (this.$route.path !== path) this.$router.push(path);
            },
            goToAddAdmin: function() {
                let path = '/add-admin';
                if (this.$route.path !== path) this.$router.push(path);
            },
            goToSignIn: function() {
                let path = '/sign-in';
                if (this.$route.path !== path) this.$router.push(path);
            },
            goToSignUp: function() {
                let path = '/sign-up';
                if (this.$route.path !== path) this.$router.push(path);
            },
            goToMyPosts: function() {
                let path = '/my-posts';
                if (this.$route.path !== path) this.$router.push(path);
            },
        },
        created: function () {
            if (localStorage.getItem('token')) {
                var decoded = jwt_decode(localStorage.getItem('token'));
                this.currentUser = decoded;
            }
        }
    };
</script>

<style scoped>
    #log-out {
        cursor: pointer;
    }
</style>