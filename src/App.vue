<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import RightTrigger from "./components/RightTrigger.vue";
import LeftTrigger from "./components/LeftTrigger.vue";
import GlobalTriggerModes from "./components/GlobalTriggerModes.vue";
import ActivateOnProgramOpen from "./components/ActivateOnProgramOpen.vue";
import DeletePrograms from "./components/DeletePrograms.vue";
</script>

<template>
  <div class="container">
    <h1 class="title">DualFlow</h1>
    <GlobalTriggerModes />
    <div class="flex-container">
      <LeftTrigger class="flex-child" />
      <RightTrigger class="flex-child" />
    </div>
    <br />
    <div class="flex-container">
      <ActivateOnProgramOpen class="flex-child" />
      <DeletePrograms class="flex-child" />
    </div>
  </div>
</template>

<style scoped>
.title {
  margin-top: -4%;
}

.flex-container {
    display: flex;
}

.flex-child {
    flex: 1;
}

.flex-child:first-child {
    margin-right: 20px;
} 
</style>
