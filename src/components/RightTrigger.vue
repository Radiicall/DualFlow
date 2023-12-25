<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const strength0 = ref<number>(0);
const strength1 = ref<number>(0);
const strength2 = ref<number>(0);
const strength3 = ref<number>(0);
const strength4 = ref<number>(0);
const strength5 = ref<number>(0);
const strength6 = ref<number>(0);

const triggerMode = ref<string>("");

async function changeStrength() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("change_strength", { strength: strength0.value, trigger: "right", which: 0 });
  await invoke("change_strength", { strength: strength1.value, trigger: "right", which: 1 });
  await invoke("change_strength", { strength: strength2.value, trigger: "right", which: 2 });
  await invoke("change_strength", { strength: strength3.value, trigger: "right", which: 3 });
  await invoke("change_strength", { strength: strength4.value, trigger: "right", which: 4 });
  await invoke("change_strength", { strength: strength5.value, trigger: "right", which: 5 });
  await invoke("change_strength", { strength: strength6.value, trigger: "right", which: 6 });
}

async function changeTriggerMode() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("change_trigger_mode", { mode: triggerMode.value, trigger: "right" });
}

</script>

<template>
  <div>
    <h2>Right:</h2>
    <div class="item-div">
      <p class="text">Resistance start: </p>
      <input class="input" id="strength-right" v-model="strength0" :on-change="changeStrength()" type="number" :max="220" :min="0" />
    </div>
    <div class="item-div">
      <p class="text">Force: </p>
      <input class="input" id="strength-right" v-model="strength1" :on-change="changeStrength()" type="number" :max="255" :min="0" />
    </div>
    <div class="item-div">
      <p class="text">Force Range: </p>
      <input class="input" id="strength-right" v-model="strength2" :on-change="changeStrength()" type="number" :max="255" :min="0" />
    </div>
    <div class="item-div">
      <p class="text">Effect near release: </p>
      <input class="input" id="strength-right" v-model="strength3" :on-change="changeStrength()" type="number" :max="255" :min="0" />
    </div>
    <div class="item-div">
      <p class="text">Effect near middle: </p>
      <input class="input" id="strength-right" v-model="strength4" :on-change="changeStrength()" type="number" :max="255" :min="0" />
    </div>
    <div class="item-div">
      <p class="text">Effect when pressed: </p>
      <input class="input" id="strength-right" v-model="strength5" :on-change="changeStrength()" type="number" :max="255" :min="0" />
    </div>
    <div class="item-div">
      <p class="text">Effect actuation freq in Hz: </p>
      <input class="input" id="strength-right" v-model="strength6" :on-change="changeStrength()" type="number" :max="255" :min="0" />
    </div>

    <div class="item-div">
      <p class="text">Mode: </p>
      <select class="input" v-model="triggerMode" :on-change="changeTriggerMode()">
        <option>Off</option>
        <option>Rigid</option>
        <option>RigidA</option>
        <option>RigidB</option>
        <option>RigidAB</option>
        <option>Pulse</option>
        <option>PulseA</option>
        <option>PulseB</option>
        <option>PulseAB</option>
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
  height: 1%;
}

</style>
