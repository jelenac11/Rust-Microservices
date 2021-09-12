import Vue from 'vue'
import Vuex from "vuex";
import axios from 'axios'
import VueAxios from 'vue-axios'

Vue.use(Vuex)
Vue.use(VueAxios, axios)

Vue.prototype.$axios = axios;
const token = localStorage.getItem('token')
if (token) {
    Vue.prototype.$axios.defaults.headers.common['Authorization'] = 'Bearer ' + token;
}

export default new Vuex.Store({
    state: {
        token: localStorage.getItem('token') || '',
        allPosts: [],
    },
    getters: {
        authenticated: state => !!state.token,
    },
    mutations: {
        setPosts(state, posts) {
            state.allPosts = posts;
        },

        auth_success_token(state, token) {
            state.token = token;
        },

        setCurrentUser(state, user) {
            state.user = user;
        },

        logout(state) {
            localStorage.removeItem('token');
            state.token = '';
            state.user = {};
        },

        setGroceries(state, post) {
            console.log(state);
            console.log(post);
        },

    },
    actions: {
        getAllPosts({ commit }) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/posts', method: 'GET' })
                    .then(resp => {
                        commit('setPosts', resp.data);
                        resolve(resp);
                    })
                    .catch(err => {
                        reject(err);
                    });
            });
        },

        getMyPosts({ commit }) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/posts/author', method: 'GET' })
                    .then(resp => {
                        commit('setPosts', resp.data);
                        resolve(resp);
                    })
                    .catch(err => {
                        reject(err);
                    });
            });
        },

        addPost({ commit }, post) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/posts', data: post, method: 'POST' })
                    .then(resp => {
                        commit('setGroceries', resp.data);
                        resolve(resp);
                    })
                    .catch(err => {
                        reject(err);
                    });
            });
        },

        getRate({ commit }, postId) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/rates/post/' + postId, method: 'GET' })
                    .then(resp => {
                        commit('setGroceries', resp.data);
                        resolve(resp);
                    })
                    .catch(err => {
                        reject(err);
                    });
            });
        },

        ratePost({ commit }, post) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/rates', data: { value: post.currentRating, post_id: post.id }, method: 'POST' })
                    .then(resp => {
                        commit('setGroceries', resp.data);
                        resolve(resp);
                    })
                    .catch(err => {
                        reject(err);
                    });
            });
        },

        getComments({ commit }, postId) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/comments/' + postId, method: 'GET' })
                    .then(resp => {
                        commit('setGroceries', resp.data);
                        resolve(resp);
                    })
                    .catch(err => {
                        reject(err);
                    });
            });
        },

        addComment({ commit }, comment) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/comments', data: comment, method: 'POST' })
                    .then(resp => {
                        commit('setGroceries', resp.data);
                        resolve(resp);
                    })
                    .catch(err => {
                        reject(err);
                    });
            });
        },

        deleteComment({ commit }, comment) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/comments/' + comment.id, method: 'DELETE' })
                    .then(resp => {
                        commit('setGroceries', resp.data);
                        resolve(resp);
                    })
                    .catch(err => {
                        reject(err);
                    });
            });
        },

        deletePost({ commit }, post) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/posts/' + post.id, method: 'DELETE' })
                    .then(resp => {
                        commit('setGroceries', resp.data);
                        resolve(resp);
                    })
                    .catch(err => {
                        reject(err);
                    });
            });
        },

        login({ commit }, user) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/auth/login', data: user, method: 'POST' })
                    .then(resp => {
                        const token = resp.data.token;
                        localStorage.setItem('token', token);
                        axios.defaults.headers.common['Authorization'] = 'Bearer ' + token;
                        commit('auth_success_token', token);
                        resolve(resp);
                    })
                    .catch(err => {
                        localStorage.removeItem('token');
                        reject(err);
                    });
            });
        },

        register({ commit }, user) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/auth/signup', data: user, method: 'POST' })
                    .then(resp => resolve(resp))
                    .catch(err => {
                        reject(err);
                        commit('logout');
                    });
            });
        },

        addAdmin({ commit }, admin) {
            return new Promise((resolve, reject) => {
                axios({ url: 'http://localhost:8080/api/auth/admin', data: admin, method: 'POST' })
                    .then(resp => resolve(resp))
                    .catch(err => {
                        reject(err);
                        commit('logout');
                    });
            });
        },

        /*getCurrentUser({ commit }) {
            if (token) {
                var decoded = jwt_decode(this.$store.token);
            }
        },*/

    },
    modules: {}
});
