<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Tag } from "../types";

const greetMsg = ref<Tag>();
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // greetMsg.value = await invoke("greet", { name: name.value });
  invoke<Tag>("load_file").then((res) => {
    greetMsg.value = res;
  }).catch((err) => {
    alert("Error: " + err);
  })
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <Tag v-if="greetMsg?.value" v-for="(name, tag) in greetMsg.value" :key="name" :tag="tag" :name="name" />
</template>
