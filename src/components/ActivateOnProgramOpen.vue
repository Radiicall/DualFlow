<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
const program = ref<string>("");

async function useWithProgram() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("use_with_program",  { program: program.value });;
}
</script>

<template>
  <form class="item-div" @submit.prevent="useWithProgram">
    <input class="text" v-model="program" placeholder="Program" />
    <button class="input" type="submit">Save</button>
  </form>
</template>

<style>
.item-div {
  display: flex;
  flex-direction: row;
  justify-content: center;
}

.text {
  display: flex;
  justify-content: left;
  margin-right:auto;
  margin-left: 15%;
  margin-top: 2%;
  margin-bottom: 0%;
}

.input {
  margin-left: auto;
  margin-right: 15%;
  height: 100%;
  margin-top: 1%;
  margin-bottom: 0%;
}
</style>
