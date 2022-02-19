<script setup lang="ts">
import {useUserStore} from "./stores/userStore";

const user = useUserStore()
</script>

<template>
  <router-view v-if="user.loaded"/>
  <div v-else
       style="position: absolute; margin-top: 50vh; margin-left: 50vw; transform: translate(-50%,-50%); text-align: center">
    Loading... <br>
    Please wait
  </div>
</template>

<style>

</style>
