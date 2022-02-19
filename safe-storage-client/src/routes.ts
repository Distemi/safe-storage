import {createRouter, createWebHistory} from "vue-router";
import {useUserStore} from "./stores/userStore";

export const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: "/",
            component: () => import("./pages/Index.vue"),
            meta: {
                reqAuth: true
            }
        },
        {
            path: "/login",
            component: () => import("./pages/Login.vue"),
            meta: {
                reqAuth: false
            }
        }
    ]
})

router.beforeEach((to) => {
    const user = useUserStore()
    if (to.meta.reqAuth && !user.authed) return "/login"
})