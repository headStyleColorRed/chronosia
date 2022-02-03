import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Login from '../modules/Login/views/Login.vue'
import store from '../store/index.js'

const routes = [
    {
        path: '/login',
        name: 'Login',
        component: Login,
    },
    {
        path: '/',
        name: 'Home',
        component: Home
    }
]

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
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
