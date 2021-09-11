<template>
    <v-container fluid fill-height class="login-container">
        <v-layout align-center justify-center>
            <v-flex xs12 sm8 md3>
                <v-card class="elevation-12" style="padding: 20px">
                    <v-card-text>
                        <h1 style="color: #1976D2; margin-bottom: 30px; margin-top: 10px">Sign in</h1>
                        <v-form class="mt-4" v-model="isValid">
                            <v-text-field
                                color="blue darken-2"
                                prepend-icon="person"
                                name="email"
                                label="Email"
                                type="text"
                                v-model="userLogin.email"
                                :rules="[v => !!v || 'Email is required']"
                                required
                            ></v-text-field>
                            <v-text-field
                                color="blue darken-2"
                                id="password"
                                prepend-icon="lock"
                                name="password"
                                label="Password"
                                type="password"
                                v-model="userLogin.password"
                                :rules="[v => !!v || 'Password is required']"
                                required
                            ></v-text-field>
                        </v-form>
                    </v-card-text>
                    <v-card-actions class="justify-center">
                        <v-btn 
                            color="blue darken-2"
                            class="my-2" 
                            style="width: 95%; color: white;" 
                            :disabled="!isValid"
                            v-on:click="login"
                        >Sign in</v-btn>
                    </v-card-actions>
                    <router-link to="/sign-up" class="ml-5" style="color: #1976D2;">Don't have an account? Sign up here.</router-link>
                    <br/>
                    <router-link to="/" class="ml-5" style="color: #1976D2;">Go to homepage</router-link>
                </v-card>
            </v-flex>
        </v-layout>
        <v-snackbar v-model="wrong" top color="red darken-3">
            Wrong email or password. Try again.
        </v-snackbar>
    </v-container>
</template>

<script>
  export default {
    name: 'SignIn',
    props: {
        source: String,
    },
    data: () => ({
        userLogin: {
            email: null,
            password: null,
        },
        isValid: true,
        wrong: false,
        forbidden: false,
        token: null
    }),
    created() {
		localStorage.removeItem("token");
	},
    methods : {
		login : function () {
            let email = this.userLogin.email;
            let password = this.userLogin.password;
            this.$store.dispatch('login', { "email": email, "password": password })
            .then(() => this.$router.push('/'))
            .catch(() => {
                this.wrong = true;
            });
		},
    }
}
</script>
<style scoped>
    .login-container {
        background-image: url(../assets/s.jpg);
    }
</style>