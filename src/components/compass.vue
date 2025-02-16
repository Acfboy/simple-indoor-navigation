<template>
    <n-card bordered title="参考前进方向" size="small" style="height: 100%;">
        <n-flex justify="center" style="position: relative; top:20%" v-if="target != -1">
            <n-icon depth="3" size="100" color="#228900">
                <ArrowUp :style="`transform: rotate(${arrow}deg);`" />
            </n-icon>
        </n-flex>
        <n-empty description="导航未开始" style="position: relative; top:20%" v-else>
        </n-empty>
        <template #footer>
            <n-text depth="3" style="font-size: 0.6em;">
                参考方向由磁力计确定，可能有较大误差，仅供参考。
            </n-text>
        </template>
    </n-card>
</template>

<script lang="ts">
import { NIcon, NCard, NFlex, NText, NEmpty } from 'naive-ui';
import { listen } from '@tauri-apps/api/event';
import { ArrowUp } from '@vicons/ionicons5'
import { invoke } from '@tauri-apps/api/core';

export default {
    components: {
        NIcon,
        ArrowUp,
        NCard,
        NFlex,
        NText,
        NEmpty
    },
    data() {
        return {
            orien: 0,
            target: -1,
            arrow: 0
        };
    },
    methods: {
        async setTargetListener() {
            await listen<number>('target-change', (event) => {
                this.target = event.payload;
                this.updateArrow();
            });
        },
        updateArrow() {
            let d = this.target - this.orien;
            if (d < 0) d += 360;
            if (d >= 335 || d <= 25) d = 0;
            else if (d > 25 && d <= 65) d = 45;
            else if (d > 65 && d <= 115) d = 90;
            else if (d > 115 && d <= 155) d = 135;
            else if (d > 155 && d <= 205) d = 180;
            else if (d > 205 && d <= 245) d = 225;
            else if (d > 245 && d <= 295) d = 270;
            else if (d > 295 && d <= 335) d = 315;
            this.arrow = d;
        }
    },
    mounted() {
        this.setTargetListener();
        setInterval(async () => {
            type OrientationData = {
                orientation: number;
            };
            const resp: OrientationData = await invoke("plugin:mobilesensors|get_orientation");
            this.orien = resp.orientation;
            this.updateArrow();
        }, 200);
    },
};
</script>

<style scoped>
</style>