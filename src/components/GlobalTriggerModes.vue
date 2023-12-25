<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const triggerMode = ref<string>("");

async function changeMode() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  if (triggerMode.value === "Off") {
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 0 });
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 1 });
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 2 });
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 3 });
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 4 });
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 5 });
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 6 });
    await invoke("change_trigger_mode", { mode: "Off", trigger: "left" });

    await invoke("change_strength", { strength: 0,   trigger: "right", which: 0 });
    await invoke("change_strength", { strength: 0,   trigger: "right", which: 1 });
    await invoke("change_strength", { strength: 0,   trigger: "right", which: 2 });
    await invoke("change_strength", { strength: 0,   trigger: "right", which: 3 });
    await invoke("change_strength", { strength: 0,   trigger: "right", which: 4 });
    await invoke("change_strength", { strength: 0,   trigger: "right", which: 5 });
    await invoke("change_strength", { strength: 0,   trigger: "right", which: 6 });
    await invoke("change_trigger_mode", { mode: "Off", trigger: "right" });
  } else if (triggerMode.value === "Steam Soft Push") {
    await invoke("change_strength", { strength: 144, trigger: "left", which: 0 });
    await invoke("change_strength", { strength: 160, trigger: "left", which: 1 });
    await invoke("change_strength", { strength: 255, trigger: "left", which: 2 });
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 3 });
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 4 });
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 5 });
    await invoke("change_strength", { strength: 0,   trigger: "left", which: 6 });
    await invoke("change_trigger_mode", { mode: "Pulse", trigger: "left" });

    await invoke("change_strength", { strength: 144, trigger: "right", which: 0 });
    await invoke("change_strength", { strength: 160, trigger: "right", which: 1 });
    await invoke("change_strength", { strength: 255, trigger: "right", which: 2 });
    await invoke("change_strength", { strength: 0,   trigger: "right", which: 3 });
    await invoke("change_strength", { strength: 0,   trigger: "right", which: 4 });
    await invoke("change_strength", { strength: 0,   trigger: "right", which: 5 });
    await invoke("change_strength", { strength: 0,   trigger: "right", which: 6 });
    await invoke("change_trigger_mode", { mode: "Pulse", trigger: "right" });
  }
}

</script>

<template>
  <div>
    <div class="item-div">
      <p class="global-trigger-text">Global mode: </p>
      <select class="global-trigger-input" v-model="triggerMode" :on-change="changeMode()">
        <option>Off</option>
        <option>Steam Soft Push</option>
      </select>
    </div>
  </div>
</template>

<style>
.item-div {
  display: flex;
  flex-direction: row;
  justify-content: center;
}

.global-trigger-text {
  display: flex;
  justify-content: left;
  margin-right:auto;
  margin-left: 36%;
  margin-top: 2%;
  margin-bottom: 0%;
}

.global-trigger-input {
  margin-left: auto;
  margin-right: 36%;
  margin-top: 1%;
  margin-bottom: 0%;
  height: 1%;
}

</style>
