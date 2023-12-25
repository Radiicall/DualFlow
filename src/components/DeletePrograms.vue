<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

const program = ref<string>("");

async function deleteProgram() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("delete_program",  { program: program.value });

  document.getElementById("programs")!.innerHTML = `<option disabled selected>Select a program</option>`;
}

</script>

<template>
  <div class="item-div">
    <select class="text" id="programs" v-model="program">
      <option disabled selected>Select a program</option>
    </select>

    <button class="input" @click="deleteProgram">Remove program</button>
  </div>
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
  margin-top: 1%;
  margin-bottom: 0%;
  height: 100%;
}
</style>

<script lang="ts">
await listen('apps', (event: any) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object

  let apps: string[] = event.payload.apps;

  let i: number = 0;

  apps.forEach((app: string) => {
    app = app.slice(1, app.length - 1)

    if (document.getElementById("programs")!.innerHTML.includes(app)) {
      i++
      return;
    }
    document.getElementById("programs")!.innerHTML += `<option id="${i}">${app}</option>`;
  });
})
</script>