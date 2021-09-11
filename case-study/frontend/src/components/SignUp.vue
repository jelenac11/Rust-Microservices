<template>
  <v-container fluid fill-height class="register-container">
        <v-layout align-center justify-center>
            <v-flex xs12 sm8 md4>
                <v-card class="elevation-12" style="padding: 20px">
                    <v-card-text>
                    <h1 style="color: #1976D2; margin-bottom: 30px; margin-top: 10px">Sign up</h1>
                        <v-form class="mt-4" v-model="isValid" ref="formRegister">
                            <v-text-field
                                color="blue darken-2"
                                prepend-icon="person"
                                name="username"
                                label="Username"
                                type="text"
                                v-model="user.username"
                                :rules="[v => !!v || 'Username is required']"
                                required
                            ></v-text-field>
                            <v-text-field
                                color="red"
                                prepend-icon="person"
                                name="email"
                                label="Email"
                                type="text"
                                v-model="user.email"
                                :rules="emailRules"
                                required
                            ></v-text-field>
                            <v-text-field
                                color="blue darken-2"
                                prepend-icon="person"
                                name="firstName"
                                label="First name"
                                type="text"
                                v-model="user.firstname"
                                :rules="[v => !!v || 'First name is required']"
                                required
                            ></v-text-field>
                            <v-text-field
                                color="blue darken-2"                            
                                prepend-icon="person"
                                name="lastName"
                                label="Last name"
                                type="text"
                                v-model="user.lastname"
                                :rules="[v => !!v || 'Last name is required']"
                                required
                            ></v-text-field>
                            <v-text-field
                                color="blue darken-2"
                                id="password"
                                prepend-icon="lock"
                                name="password"
                                label="Password"
                                type="password"
                                v-model="user.password"
                                :rules="passwordRules"
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
                            v-on:click="register"
                        >Sign up</v-btn>
                    </v-card-actions>
                    <router-link to="/sign-in" class="ml-5" style="color: #1976D2;">Already have an account? Sign in here.</router-link>
                    <br/>
                    <router-link to="/" class="ml-5" style="color: #1976D2;">Go to homepage</router-link>
                </v-card>
            </v-flex>
        </v-layout>
        <v-snackbar v-model="conflict" top color="red darken-4">
            User with given username or email already exists. Try again.
        </v-snackbar>
    </v-container>
</template>

<script>
    export default {
        name: 'SignUp',
        props: {
            source: String,
        },
        data: () => ({
            user: {
                email: null,
                username: null,
                firstname: null,
                lastname: null,
                password: null,
                role: 'ROLE_USER'
            },
            isValid: true,
            conflict: false,
            emailRules: [ 
                v => !!v || 'Email is required',
                v => /.+@.+/.test(v) || 'E-mail must be valid' 
            ],
            passwordRules: [ 
                v => !!v || 'Password is required', 
                v => (v && v.length >= 6) || 'Password must have at least 6 characters' 
            ]
        }),
        methods : {
            register : function () {
                let newUser = this.user;
                this.$store.dispatch('register', newUser)
                .then(() => this.$router.push('/sign-in'))
                .catch(err => {
                    console.log(err);
                    this.conflict = true;
                });
            },
        }
    }
</script>
<style scoped>
    .register-container {
        background-image: url(../assets/s.jpg);
    }
</style>
