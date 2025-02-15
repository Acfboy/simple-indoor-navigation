<script setup lang="ts">
import { ref } from "vue";
import { NLayout, NLayoutSider, NLayoutContent, NLayoutFooter, NIcon, NFlex } from 'naive-ui'

import { ArrowBack } from '@vicons/ionicons5';
import Intersection from "./components/intersection.vue";
import Timeline from "./components/timeline.vue";
import Compass from "./components/compass.vue";
import Prompt from "./components/prompt.vue";
import Footer from "./components/footer.vue";
import Painter from "./components/painter.vue";
import Manager from "./components/manager.vue";
import { exists, BaseDirectory, readTextFile } from '@tauri-apps/plugin-fs';

const screenHeight = window.screen.height * 0.99;
const screenWidth = window.screen.width;

const promptHeight = screenHeight * 0.20;
const mainHeight = screenHeight * 0.7;
const timelineWidth = ref(screenWidth * 0.2);

const page = ref("main");

function handleSwitch(nextPage: string) {
  page.value = nextPage
}

function handleBack() {
  page.value = "main"
}


</script>

<template>
  <main class="container">

    <n-layout v-show="page == 'main'">
      <n-layout-content has-sider :style="`height: ${mainHeight}px; `">
        <n-layout-sider collapse-mode="width" :collapsed-width="timelineWidth" :width="timelineWidth * 2"
          show-trigger="arrow-circle" content-style="padding: 24px;" bordered>
          <Timeline />
        </n-layout-sider>
        <n-layout-content>
          <div :style="`height: ${mainHeight * 0.5}px;`">
            <Intersection />
          </div>
          <div :style="`height: ${mainHeight * 0.5}px;`">
            <Compass />
          </div>
        </n-layout-content>
      </n-layout-content>
      <n-layout-content :style="`height: ${promptHeight}px;`" bordered>
        <Prompt />
      </n-layout-content>
      <n-layout-footer bordered>
        <Footer @switch="handleSwitch" />
      </n-layout-footer>
    </n-layout>

    <n-layout v-show="page == 'info'">
      <n-layout-content has-sider :style="`height: ${screenHeight * 0.9}px; `">
        <Info />
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

    <Painter @switch="handleSwitch" :screenHeight="screenHeight" v-if="page == 'paint'"/>


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