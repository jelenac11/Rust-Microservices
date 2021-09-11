import Vue from "vue";
import VueRouter from "vue-router";
import SignIn from "../components/SignIn.vue";
import SignUp from "../components/SignUp.vue";
import Posts from "../components/Posts.vue";
import AddPost from "../components/AddPost.vue";
import AddAdmin from "../components/AddAdmin.vue";
import MyPosts from "../components/MyPosts.vue";

Vue.use(VueRouter);

const routes = [
  {
    path: "/",
    name: "Posts",
    component: Posts
  },
  {
    path: "/sign-in",
    name: "SignIn",
    component: SignIn
  },
  {
    path: "/sign-up",
    name: "SignUp",
    component: SignUp
  },
  {
    path: "/add-post",
    name: "AddPost",
    component: AddPost
  },
  {
    path: "/add-admin",
    name: "AddAdmin",
    component: AddAdmin
  },
  {
    path: "/my-posts",
    name: "MyPosts",
    component: MyPosts
  },
];

const router = new VueRouter({
  routes
});

export default router;
