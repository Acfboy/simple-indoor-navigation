<script setup lang="ts">
import { ref } from "vue";
import { NLayout, NLayoutContent, NLayoutFooter, NIcon, NFlex, NButton } from 'naive-ui'

import { ArrowBack } from '@vicons/ionicons5';
import Map from "./components/map.vue";
import Prompt from "./components/prompt.vue";
import Footer from "./components/footer.vue";
import Painter from "./components/painter.vue";
import Manager from "./components/manager.vue";

const screenHeight = window.innerHeight;
const screenWidth = window.innerWidth;

const promptHeight = screenHeight * 0.2;
// const timelineHeight = screenHeight * 0.2;
const mainHeight = screenHeight * 0.7;


const page = ref("paint");

function handleSwitch(nextPage: string) {
  page.value = nextPage
}

function handleBack() {
  page.value = "main"
}


</script>

<template>
  <main class="container">

    <n-layout v-if="page == 'main'">
      <!-- <n-layout-content :content-style="`height: ${timelineHeight}px; overflow: auto; padding:24px;`">
        <Timeline />
      </n-layout-content> -->

      <n-layout-content :style="`height: ${mainHeight}px;`">
        <Map :screenWidth="screenWidth" :screenHeight="mainHeight"/>
      </n-layout-content>

      <n-layout-content bordered :style="`height: ${promptHeight}px;`">
        <Prompt />
      </n-layout-content>

      <n-layout-footer bordered>
        <Footer @switch="handleSwitch" />
      </n-layout-footer>

    </n-layout>

    <n-layout v-show="page == 'info'">
      <n-layout-content :style="`height: ${screenHeight * 0.9}px; `">
        <!-- <Info /> -->
      </n-layout-content>
      <n-layout-footer bordered>
        <n-flex justify="center">
          <n-button @click="handleBack">
            <n-icon>
              <ArrowBack />
            </n-icon>
            返回
          </n-button>
        </n-flex>
      </n-layout-footer>
    </n-layout>

    <n-layout v-if="page == 'mapmanager'">
      <n-layout-content has-sider :style="`height: ${screenHeight * 0.9}px; `">
        <Manager />
      </n-layout-content>
      <n-layout-footer bordered>
        <n-flex justify="center">
          <n-button @click="handleBack">
            <n-icon>
              <ArrowBack />
            </n-icon>
            返回
          </n-button>
        </n-flex>
      </n-layout-footer>
    </n-layout>

    <Painter @switch="handleSwitch" v-if="page == 'paint'" />


  </main>
</template>

<style scoped>
.n-layout-header {
  padding: 24px;
}

.n-layout-footer {
  padding: 24px;
  background-color: white;
}
</style>