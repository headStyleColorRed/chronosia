import Vue from 'vue'
import VueRouter from 'vue-router'
import Home from '../views/Home.vue'
import Login from '../modules/Login/views/Login.vue'
import store from '../store/index.js'

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/login',
    name: 'Login',
    component: Login,
},
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

router.beforeEach((to, from, next) => {
    if (store.getters.isLogged == false && to.path !== "/login") {
        next({ name: "Login" })
    } else if (store.getters.isLogged == true && to.path === "/login") {
        next({ name: "Home" })
    } else {
        next()
    }
})

export default router
