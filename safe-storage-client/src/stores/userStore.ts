import {defineStore} from "pinia";
import axios from "axios";

export const useUserStore = defineStore({
    id: "user",
    state: () => ({
        loaded: false,
        authed: false,
        login: "",
        limit: -1,
        prefix: "",
        admin: false
    }),
    actions: {
        async updateInfo() {
            try {
                const resp = await axios.get("/api/user_info");
           
                const {successful, message, login, path, limit, admin}: {
                    successful: boolean,
                    message: string,
                    login: string,
                    path: string,
                    limit: number,
                    admin: boolean,
                } = Object.assign({
                    successful: false,
                    message: "",
                    login: "",
                    path: "",
                    limit: -1,
                    admin: false
                }, resp.data);

                if (!successful) {
                    console.log(message);
                    return;
                }

                this.admin = admin;
                this.prefix = path;
                this.limit = limit
                this.login = login;
                this.authed = limit != -1;
                this.loaded = true;
            } catch (ignored) {
            }
        }
    }
})