<template>
    <n-card style="height: 100%;">
        <n-card style="position: relative; top:20%;" :bordered="false">
            <!-- <n-space justify="center"> -->
                <n-text style="font-size: 2em;">{{ prompt }}</n-text>
            <!-- </n-space> -->
        </n-card>
    </n-card>
</template>

<script lang="ts">
import { NCard, NText, NSpace } from 'naive-ui';
import { listen } from '@tauri-apps/api/event';



export default {
    components: {
        NCard, NText, NSpace
    },
    data() {
        return {
            prompt: "导航未开始。"
        };
    },
    methods: {
        async setPromptListener() {
            listen<string>('prompt', (event) => {
                this.prompt = event.payload;
            });
        },
    },
    mounted() {
        this.setPromptListener();
    },
};
</script>

<style scoped></style>