<template>
  <div class="flex justify-center min-h-screen">
    <form class="my-auto p-6 shadow rounded-lg border" :class='{"bg-gray-200": wait}' action="/api/login" method="post"
          :aria-disabled="wait">
      <h2 class="text-center mb-4">Safe Storage v0.1</h2>
      <label>
        Login:
        <input class="input" type="text" name="login" :disabled="wait" v-model="login">
      </label> <br>
      <label class="mt-4">
        Password:
        <input class="input" type="password" name="password" :disabled="wait" v-model="password">
      </label> <br>
      <div class="mx-4 rounded border border-red-400 bg-red-500 py-2 px-6 text-white" v-if="problem.length > 0">
        {{ problem }}
      </div>
      <button type="submit" class="mt-4 mx-auto block btn" :disabled="wait" @click.prevent="tryLogIn">
        Log in
      </button>
    </form>
  </div>
</template>

<script lang="ts">
import {ref} from "vue";
import axios from "axios";
import {useRouter} from "vue-router";

export default {
  name: "Login",
  setup() {
    const [wait, login, password, problem] = [ref(false), ref(""), ref(""), ref("")];
    const router = useRouter();

    async function tryLogIn() {
      wait.value = true

      try {
        const resp = await axios.post("/api/login", {
          login: login.value,
          password: password.value
        }, {
          validateStatus: () => true
        });
        const data: {
          successful: boolean,
          message: string
        } = resp.data;
        if (!data.successful) {
          problem.value = data.message || "Failed to login";
        } else {
          await router.push("/");
        }
      } catch (error) {
      }
      wait.value = false
    }

    return {
      wait,
      login,
      password,
      tryLogIn,
      problem
    }
  }
}
</script>

<style scoped>

</style>