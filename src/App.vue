<script setup lang="ts">
import { ref } from "vue";
import { NLayout, NLayoutSider, NLayoutContent, NLayoutFooter } from 'naive-ui'
import Intersection from "./components/intersection.vue";
import Timeline from "./components/timeline.vue";
import Compass from "./components/compass.vue";
import Prompt from "./components/prompt.vue";
import Footer from "./components/footer.vue";

const screenHeight = window.screen.height;
const screenWidth = window.screen.width;

const promptHeight = screenHeight * 0.20;
const mainHeight = screenHeight * 0.7;
const timelineWidth = ref(screenWidth * 0.2);
</script>

<template>
  <main class="container">
    <n-layout>
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
        <Footer/>
      </n-layout-footer>
    </n-layout>
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